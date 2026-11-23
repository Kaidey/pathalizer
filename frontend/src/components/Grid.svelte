<script lang="ts">
    import { grid } from "../state/grid.svelte";
    import { appState } from "../state/app.svelte";
    import "../styles/grid.css";

    let painting = false;

    function pointerDown(id: number) {
        painting = true;
        if (appState.paintNode == 0) {
            grid.setStart(id);
        } else if (appState.paintNode == 1) {
            grid.setEnd(id);
        } else {
            grid.toggleWall(id);
        }
    }

    function pointerEnterCell(id: number) {
        if (painting) {
            if (appState.paintNode == 0) {
                grid.setStart(id);
            } else if (appState.paintNode == 1) {
                grid.setEnd(id);
            } else {
                grid.toggleWall(id);
            }
        }
    }
</script>

<div id="grid-container">
    <div id="grid" style:--cols={grid.cols} style:--rows={grid.rows}>
        {#each { length: grid.rows * grid.cols }, id}
            <div
                class="cell"
                class:wall={grid.walls.has(id)}
                class:start={id === grid.start}
                class:end={id === grid.end}
                onpointerdown={() => pointerDown(id)}
                onpointerenter={() => pointerEnterCell(id)}
                onpointerup={() => (painting = false)}
            ></div>
        {/each}
    </div>
</div>
