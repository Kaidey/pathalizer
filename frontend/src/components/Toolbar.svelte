<script lang="ts">
    import "../styles/toolbar.css";
    import GridSizer from "./GridSizer.svelte";
    import { appState } from "../state/app.svelte";
    import { grid } from "../state/grid.svelte";
    import type { Trace } from "../state/grid.svelte";

    let showSizer = $state(false);

    interface SolveResponse {
        path: Array<number>;
        cost: number;
        trace: Array<Trace>;
    }

    async function solve() {
        let obj = {
            start: grid.start,
            end: grid.end,
            adjacency: grid.toAdjacencyList(),
        };

        try {
            // /solve is the relative URL configured in the vite proxy to avoid CORS issues
            // https://dev.to/aartijangid23/how-to-fix-cors-errors-in-a-local-development-environment-132l
            const res = await fetch("/solve", {
                method: "POST",
                body: JSON.stringify(obj),
            });

            if (!res.ok){
                throw new Error("Server returned ${res.status}");
            }

            const res_data: SolveResponse = await res.json();
            grid.setSolution(res_data.path);
            grid.setVisited(res_data.trace);

        } catch (err) {
            throw new Error("Failed to solve: ${err}");
        }
    }

    function clearPath(){
        grid.clearSolution()
    }
</script>

{#if showSizer}
    <GridSizer onClose={() => (showSizer = false)} />
{/if}

<div id="toolbar">
    <button onclick={() => (showSizer = true)}> Generate grid </button>
    <select
        onchange={(e) =>
            appState.switchPaintEntity(Number(e.currentTarget.value))}
        id="node-selector"
    >
        <option value="">Nodes</option>
        {#each appState.getAvailableNodes() as node}
            <option value={node.value}>{node.label}</option>
        {/each}
    </select>
    <button onclick={solve}> Solve </button>
    <button onclick={clearPath}> Clear Path </button>
</div>
