import {invokeCommand} from "../../shared/tauri/invoke";
import type {CountdownCommand, CountdownSnapshot, CountdownSnapshotDto, Duration} from "./types";
import {millisToDuration} from "./helper.ts";

export async function fetchCountdownSnapshot(): Promise<CountdownSnapshot> {
    let temp = await invokeCommand<CountdownSnapshotDto>("countdown_snapshot");
    let duration: Duration = millisToDuration(temp.duration);
    return {
        id: temp.id,
        label: temp.label,
        duration: duration,
        state: temp.state,
        start_epoch_ms: temp.start_epoch_ms ? new Date(temp.start_epoch_ms) : null,
        target_epoch_ms: temp.target_epoch_ms ? new Date(temp.target_epoch_ms) : null,
    };
}

async function invokeCountdownCommand(command: CountdownCommand): Promise<void> {
    await invokeCommand<void>(command);
}

export async function startCountdown(): Promise<void> {
    await invokeCountdownCommand("countdown_start");
}

export async function pauseCountdown(): Promise<void> {
    await invokeCountdownCommand("countdown_pause");
}

export async function resumeCountdown(): Promise<void> {
    await invokeCountdownCommand("countdown_resume");
}

export async function resetCountdown(): Promise<void> {
    await invokeCountdownCommand("countdown_reset");
}
