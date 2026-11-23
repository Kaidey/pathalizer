class AppState {
    paintNode = $state(-1);

    switchPaintEntity(node: number) {
        this.paintNode = node;
    }

    getAvailableNodes() {
        return [
            { value: 0, label: "Start" },
            { value: 1, label: "End" },
            { value: 2, label: "Wall" }
        ]
    }
}

export const appState = new AppState();
