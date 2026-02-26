import { invokeCommand } from "../../shared/tauri/invoke";
import type { CountdownCommand, CountdownSnapshotDto } from "./types";

export async function fetchCountdownSnapshot(): Promise<CountdownSnapshotDto> {
  return invokeCommand<CountdownSnapshotDto>("countdown_snapshot");
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
