<script lang="ts">
    import "../styles/toolbar.css";
    import GridSizer from "./GridSizer.svelte";
    import { appState } from "../state/app.svelte";

    let showSizer = $state(false);
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
</div>
