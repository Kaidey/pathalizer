<script lang="ts">
    import "../styles/toolbar.css";
    import GridSizer from "./GridSizer.svelte";
    import { appState } from "../state/app.svelte";
    import { grid } from "../state/grid.svelte";

    let showSizer = $state(false);

    function solve() {
        let obj = {
            start: grid.start,
            end: grid.end,
            adjacency: grid.toAdjacencyList(),
        };

        let res = fetch("/solve", {
            method: "POST",
            body: JSON.stringify(obj),
        }).then((r) => r.json());

        console.log(res);
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
    <button
        onclick={solve}
    >
        Solve
    </button>
</div>
