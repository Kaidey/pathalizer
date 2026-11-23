import { SvelteSet } from "svelte/reactivity";

class GridState {
    rows = $state(0);
    cols = $state(0);
    start = $state(-1);
    end = $state(-1);
    walls = new SvelteSet<number>();

    generate(rows: number, cols: number) {
        this.rows = rows;
        this.cols = cols;
        this.walls.clear();
        this.start = -1;
        this.end = -1;

        console.log("Walls" + this.walls);
    }

    toggleWall(id: number) {
        if (id === this.start || id === this.end) {
            return;
        }

        // If deleting the wall failed because it wasn't a wall yet
        if (!this.walls.delete(id)) {
            this.walls.add(id);
        }
    }

    setStart(id: number) {
        if (this.walls.has(id)) {
            return;
        }

        if (this.end === id) {
            this.end = -1
        }

        this.start = id;
    }

    setEnd(id: number) {
        if (this.walls.has(id)) {
            return;
        }

        if (this.start === id) {
            this.start = -1
        }

        this.end = id;
    }

}

export const grid = new GridState();
