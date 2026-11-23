<script>
    import "../styles/gridsizer.css";
    import { grid } from "../state/grid.svelte";

    let tempCols = $state(grid.cols);
    let tempRows = $state(grid.rows);

    let { onClose } = $props();
</script>

<div id="overlay">
    <div id="popup">
        <div class="labeled-input">
            <label for="col-picker">Columns: {tempCols}</label>
            <input id="col-picker" type="range" min="0" max="50" bind:value={tempCols} />
        </div>
        <div class="labeled-input">
            <label for="row-picker">Rows: {tempRows}</label>
            <input id="row-picker" type="range" min="0" max="50" bind:value={tempRows} />
        </div>
        <button
            onclick={() => {
                onClose();
                grid.generate(tempRows, tempCols);
            }}
        >
            Ok
        </button>
    </div>
</div>
