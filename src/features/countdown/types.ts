import type {Duration} from "../../shared/time/duration";

export type CountdownState = "Idle" | "Running" | "Paused" | "Finished";

export type CountdownSnapshot = {
    id: number;
    label: string;
    duration: Duration;
    state: CountdownState;
    start_epoch: Date | null;
    target_epoch: Date | null;
}

export type CountdownCommand =
    | "countdown_start"
    | "countdown_pause"
    | "countdown_resume"
    | "countdown_reset";
