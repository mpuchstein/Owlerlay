export type CountdownState = "Idle" | "Running" | "Paused" | "Finished";

export type CountdownDuration = {
  secs: number;
  nanos: number;
};

export type CountdownSnapshotDto = {
  id: number;
  label: string;
  duration: CountdownDuration;
  state: CountdownState;
  start_epoch_ms: number | null;
  target_epoch_ms: number | null;
};

export type CountdownCommand =
  | "countdown_start"
  | "countdown_pause"
  | "countdown_resume"
  | "countdown_reset";
