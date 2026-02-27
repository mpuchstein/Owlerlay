import type {Duration} from "../../../shared/time/duration";
import type {EmptyPayload} from "../../../shared/payloads/empty";

export type CountdownState = "Idle" | "Running" | "Paused" | "Finished";

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
    start_epoch: Date | null;
    target_epoch: Date | null;
}

export type CountdownCommand =
    | "countdown_create"
    | "countdown_delete"
    | "countdown_list"
    | "countdown_start"
    | "countdown_pause"
    | "countdown_resume"
    | "countdown_reset"
    | "countdown_snapshot";

export type CountdownPayload = EmptyPayload | CountdownIdPayload | CountdownCreatePayload;

export type CountdownIdPayload = {
    id: number;
};

export type CountdownCreatePayload = {
    label: string;
    duration: number;
}