/** Derive artifact folder kind from absolute path (basename). */
export function kindFromPath(path: string): string {
  const normalized = path.replace(/[/\\]+$/, "");
  const parts = normalized.split(/[/\\]/).filter(Boolean);
  return parts[parts.length - 1] ?? path;
}

/** Kinds that often collide with real source folders. */
const HIGH_RISK = new Set(["bin", "obj", "tmp", "temp", "out", "build"]);

export function isHighRiskKind(kind: string): boolean {
  return HIGH_RISK.has(kind);
}
