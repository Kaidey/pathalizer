use crate::{
    algos::dijkstra::Dijkstra,
    temp::ResultDto,
};

pub mod algos;
pub mod graph;
pub mod temp;

use rouille::{Request, Response};
use temp::decode;

fn main() {
    let addr = "127.0.0.1:8080";

    println!("Listening on {addr}");

    rouille::start_server(addr, |request| handler(request));
}

// Temporary
fn handler(request: &Request) -> Response {
    match (request.method(), request.url().as_str()) {
        ("POST", "/solve") => match decode(request) {
            Ok(req) => {
                let res = req.graph.calc_sp::<Dijkstra>(req.start, req.end);

                match res {
                    Some(result) => {
                        let res_dto = ResultDto::from(result);
                        let json = &serde_json::json!({
                            "result": {
                                "path": &res_dto.path,
                                "total_cost": &res_dto.cost,
                                "trace": &res_dto.trace
                            }
                        });

                        return Response::json(json);
                    }
                    None => {
                        return Response::json(&serde_json::json!({"result" : "No path found"}));
                    }
                }
            }
            Err(resp) => resp,
        },
        _ => Response::empty_404(),
    }
}
