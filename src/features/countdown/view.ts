import type {CountdownSnapshot} from "./model/countdown.types";
import {formatDuration} from "../../shared/time/duration";

export type CountdownView = {
    onStart: (handler: () => void) => void;
    onResume: (handler: () => void) => void;
    onPause: (handler: () => void) => void;
    onReset: (handler: () => void) => void;
    onRefresh: (handler: () => void) => void;
    setSnapshot: (snapshot: CountdownSnapshot) => void;
    setError: (message: string) => void;
};

function makeButton(label: string): HTMLButtonElement {
    const button = document.createElement("button");
    button.id = "cd-btn-" + label.toLowerCase();
    button.className = "countdown-btn"
    button.type = "button";
    button.textContent = label;
    return button;
}

export function createCountdownView(container: HTMLElement): CountdownView {
    const panel = document.createElement("section");
    panel.className = "countdown-panel";

    const title = document.createElement("h2");
    title.textContent = "Countdown Controls";

    const actions = document.createElement("div");
    actions.className = "countdown-actions";

    const startButton = makeButton("Start");
    const resumeButton = makeButton("Resume");
    const pauseButton = makeButton("Pause");
    const resetButton = makeButton("Reset");
    const refreshButton = makeButton("Refresh");

    actions.append(startButton, resumeButton, pauseButton, resetButton, refreshButton);

    const summary = document.createElement("p");
    summary.className = "countdown-summary";
    summary.textContent = "Waiting for snapshot...";

    const error = document.createElement("p");
    error.className = "countdown-error";

    const snapshot = document.createElement("pre");
    snapshot.className = "countdown-snapshot";

    panel.append(title, actions, summary, error, snapshot);
    container.appendChild(panel);

    return {
        onStart(handler) {
            startButton.addEventListener("click", handler);
        },
        onResume(handler) {
            resumeButton.addEventListener("click", handler);
        },
        onPause(handler) {
            pauseButton.addEventListener("click", handler);
        },
        onReset(handler) {
            resetButton.addEventListener("click", handler);
        },
        onRefresh(handler) {
            refreshButton.addEventListener("click", handler);
        },
        setSnapshot(value) {
            error.textContent = "";
            summary.textContent = `State: ${value.state} | Remaining: ${formatDuration(value.duration)}`;
            snapshot.textContent = JSON.stringify(value, null, 2);
        },
        setError(message) {
            error.textContent = message;
        },
    };
}
