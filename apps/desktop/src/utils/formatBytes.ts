/** Format bytes similar to human_bytes / CLI style (e.g. 1.7 GB). */
export function formatBytes(bytes: number): string {
  if (!Number.isFinite(bytes) || bytes < 0) return "0 B";
  if (bytes < 1024) return `${Math.round(bytes)} B`;

  const units = ["KB", "MB", "GB", "TB"] as const;
  let value = bytes;
  let unitIndex = -1;

  do {
    value /= 1024;
    unitIndex += 1;
  } while (value >= 1024 && unitIndex < units.length - 1);

  const digits = value >= 10 || unitIndex === 0 ? 1 : 2;
  return `${value.toFixed(digits)} ${units[unitIndex]}`;
}
