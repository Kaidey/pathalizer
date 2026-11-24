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
    <!-- TODO: Maybe fix later -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        id="grid"
        style:--cols={grid.cols}
        style:--rows={grid.rows}
        onpointerleave={() => (painting = false)}
    >
        {#each { length: grid.rows * grid.cols }, id}
            <!-- class:<name>={expr} assigns the class <name> to the element if expr evals to true -->
            <div
                class="cell"
                class:wall={grid.walls.has(id)}
                class:start={id === grid.start}
                class:end={id === grid.end}
                class:path={grid.solution.has(id) && id != grid.end}
                class:visited={grid.visited.has(id) &&
                    !grid.solution.has(id) &&
                    id != grid.start &&
                    id != grid.end}
                onpointerdown={() => pointerDown(id)}
                onpointerenter={() => pointerEnterCell(id)}
                onpointerup={() => (painting = false)}
                style:--id={id}
            ></div>
        {/each}
    </div>
</div>
