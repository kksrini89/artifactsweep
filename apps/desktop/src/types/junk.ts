/** Shape returned by Tauri `scan` (serde from Rust JunkEntry). */
export type JunkEntryDto = {
  path: string;
  size_bytes: number;
};

/** Frontend entry with derived artifact kind. */
export type JunkEntry = {
  path: string;
  sizeBytes: number;
  kind: string;
};

export type ToastMessage = {
  type: "ok" | "warn" | "err";
  title: string;
  body: string;
};

export type Phase = "idle" | "scan" | "clean";
