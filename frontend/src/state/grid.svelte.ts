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

    toAdjacencyList() {

        let total_nodes = this.rows * this.cols;
        let row_count = 0;
        let adjacency = new Array();

        for (let i = 0; i < total_nodes - 1; i++) {

            // If remainder is 0, we're at the start of a new row
            // this.cols = 5 -> 0 % 5 = 0, 5 % 5 = 0, 10 % 5 = 0, ...
            if (Math.floor(i / this.cols) != row_count) {
                row_count++
            }

            let adjacents = new Array();

            if (!this.walls.has(i)) {
                // Up -> i + this.cols
                // If Up is not over upper grid bounds
                if (i + this.cols <= total_nodes - 1) {
                    // If Up is not a wall
                    if (!this.walls.has(i + this.cols)) {
                        adjacents.push({ target: i + this.cols, weight: 1 });
                    }
                }

                // Down -> i - this.cols
                // If Down is not over lower grid bounds
                if (i - this.cols > 0) {
                    // If Down is not a wall
                    if (!this.walls.has(i - this.cols)) {
                        adjacents.push({ target: i - this.cols, weight: 1 });
                    }
                }

                // Right -> i + 1
                // If we're not at the end of a row and not over upper grid bounds
                if (Math.floor((i + 1) / this.cols) == row_count && i + 1 <= total_nodes - 1) {
                    // If Down is not a wall
                    if (!this.walls.has(i + 1)) {
                        adjacents.push({ target: i + 1, weight: 1 });
                    }
                }

                // Left -> i - 1
                // If we're not at the start of a row and not over lower grid bounds
                if (Math.floor((i - 1) / this.cols) == row_count && i - 1 >= 0) {
                    // If Down is not a wall
                    if (!this.walls.has(i - 1)) {
                        adjacents.push({ target: i - 1, weight: 1 });
                    }
                }
            }

            adjacency.push(adjacents);
        }

        return adjacency;
    }
}

export const grid = new GridState();
