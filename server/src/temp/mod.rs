// This module is temporary
// Eventually all json serialization/de-serialization will be hand-made
// AI-assisted

use crate::{
    algos::PathFinderResult,
    graph::{AdjacencyList, Edge, NodeId, ValidationError},
};
use rouille::{Request, Response};
use serde::{Deserialize, Serialize};
use std::io::Read;

const MAX_BODY_BYTES: u64 = 4 * 1024 * 1024;

pub struct SolveRequest {
    pub graph: AdjacencyList,
    pub start: NodeId,
    pub end: NodeId,
}

// ---------- wire types (the only part that is used in interactions with serde) ----------

#[derive(Deserialize)]
struct EdgeDto {
    target: usize,
    weight: u32,
}

#[derive(Deserialize)]
struct RequestDto {
    adjacency: Vec<Vec<EdgeDto>>, // adjacency[id] = outgoing edges
    start: usize,
    end: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ResultDto {
    pub path: Vec<NodeId>,
    pub cost: u32,
}

impl From<PathFinderResult> for ResultDto {
    fn from(value: PathFinderResult) -> Self {
        Self {
            path: value.ordered_path,
            cost: value.total_cost.0,
        }
    }
}

impl TryFrom<RequestDto> for SolveRequest {
    type Error = ValidationError;

    fn try_from(dto: RequestDto) -> Result<Self, Self::Error> {
        let adj = dto
            .adjacency
            .into_iter()
            .map(|edges| {
                edges
                    .into_iter()
                    .map(|e| Edge {
                        target: e.target,
                        weight: e.weight,
                    })
                    .collect()
            })
            .collect();

        let graph = AdjacencyList::new(adj)?;

        for id in [dto.start, dto.end] {
            if id >= graph.node_count() {
                return Err(ValidationError::NodeOutOfBounds(id));
            }
        }

        Ok(SolveRequest {
            graph,
            start: dto.start,
            end: dto.end,
        })
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

fn error_response(status: u16, msg: impl Into<String>) -> Response {
    Response::json(&ErrorBody { error: msg.into() }).with_status_code(status)
}

// ---------- codec: bytes -> SolveRequest ----------

pub fn decode(request: &Request) -> Result<SolveRequest, Response> {
    let body = request
        .data()
        .ok_or_else(|| error_response(400, "missing request body"))?;

    // Read at most MAX_BODY_BYTES + 1 so we can detect oversized bodies
    let mut bytes = Vec::new();
    body.take(MAX_BODY_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| error_response(400, "failed to read body"))?;

    let dto: RequestDto = serde_json::from_slice(&bytes)
        .map_err(|e| error_response(400, format!("invalid JSON: {e}")))?;

    SolveRequest::try_from(dto).map_err(|e| error_response(422, format!("{:?}", e)))
}
