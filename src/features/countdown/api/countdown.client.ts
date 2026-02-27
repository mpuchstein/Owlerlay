import {invokeCommand} from "../../../shared/tauri/invoke";
import type {CountdownCommand, CountdownSnapshot, CountdownState} from "../model/countdown.types";
import {type Duration, millisToDuration} from "../../../shared/time/duration";

type CountdownSnapshotDto = {
    id: number;
    label: string;
    duration: number;
    state: CountdownState;
    start_epoch_ms: number | null;
    target_epoch_ms: number | null;
};

export async function fetchCountdownSnapshot(): Promise<CountdownSnapshot> {
    let temp = await invokeCommand<CountdownSnapshotDto>("countdown_snapshot");
    const duration: Duration = millisToDuration(temp.duration);
    return {
        id: temp.id,
        label: temp.label,
        duration: duration,
        state: temp.state,
        start_epoch: temp.start_epoch_ms !== null ? new Date(temp.start_epoch_ms) : null,
        target_epoch: temp.target_epoch_ms !== null ? new Date(temp.target_epoch_ms) : null,
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
