export type CountdownState = "Idle" | "Running" | "Paused" | "Finished";

export type Duration = {
    hours: number;
    minutes: number;
    seconds: number;
    millis: number;
}

export type CountdownSnapshotDto = {
    id: number;
    label: string;
    duration: number;
    state: CountdownState;
    start_epoch_ms: number | null;
    target_epoch_ms: number | null;
};

export type CountdownSnapshot = {
    id: number;
    label: string;
    duration: Duration;
    state: CountdownState;
    start_epoch_ms: Date | null;
    target_epoch_ms: Date | null;
}

export type CountdownCommand =
    | "countdown_start"
    | "countdown_pause"
    | "countdown_resume"
    | "countdown_reset";
