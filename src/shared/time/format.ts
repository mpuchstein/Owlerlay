type DurationLike = {
  secs?: number;
  nanos?: number;
};

export function formatDuration(value: DurationLike): string {
  const totalSeconds = Math.max(0, Math.floor(value.secs ?? 0));
  const hours = Math.floor(totalSeconds / 3600)
    .toString()
    .padStart(2, "0");
  const minutes = Math.floor((totalSeconds % 3600) / 60)
    .toString()
    .padStart(2, "0");
  const seconds = (totalSeconds % 60).toString().padStart(2, "0");

  return `${hours}:${minutes}:${seconds}`;
}
