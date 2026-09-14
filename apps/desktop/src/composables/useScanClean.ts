import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { JunkEntry, JunkEntryDto, Phase, ToastMessage } from "../types/junk";
import { formatBytes } from "../utils/formatBytes";
import { kindFromPath } from "../utils/kind";

const LAST_PATH_KEY = "artifactsweep.lastPath";

function mapEntries(dtos: JunkEntryDto[]): JunkEntry[] {
  return dtos
    .map((d) => ({
      path: d.path,
      sizeBytes: d.size_bytes,
      kind: kindFromPath(d.path),
    }))
    .sort((a, b) => b.sizeBytes - a.sizeBytes);
}

function loadLastPath(): string {
  try {
    return localStorage.getItem(LAST_PATH_KEY) ?? "";
  } catch {
    return "";
  }
}

function saveLastPath(path: string) {
  try {
    if (path) localStorage.setItem(LAST_PATH_KEY, path);
  } catch {
    /* ignore */
  }
}

export function useScanClean() {
  const path = ref(loadLastPath());
  const entries = ref<JunkEntry[]>([]);
  const filterKind = ref<string | "all">("all");
  const busy = ref(false);
  const phase = ref<Phase>("idle");
  const showConfirm = ref(false);
  const toast = ref<ToastMessage | null>(null);
  const error = ref<string | null>(null);
  const lastScannedPath = ref("");
  const selectedPaths = ref(new Set<string>());

  let toastTimer: ReturnType<typeof setTimeout> | null = null;

  const filteredEntries = computed(() => {
    if (filterKind.value === "all") return entries.value;
    return entries.value.filter((e) => e.kind === filterKind.value);
  });

  const selectedEntries = computed(() =>
    filteredEntries.value.filter((e) => selectedPaths.value.has(e.path)),
  );

  const filteredTotal = computed(() =>
    filteredEntries.value.reduce((sum, e) => sum + e.sizeBytes, 0),
  );

  const selectedTotal = computed(() =>
    selectedEntries.value.reduce((sum, e) => sum + e.sizeBytes, 0),
  );

  const totalBytes = computed(() =>
    entries.value.reduce((sum, e) => sum + e.sizeBytes, 0),
  );

  const largest = computed(() => selectedEntries.value[0] ?? null);

  const categories = computed(() => {
    const set = new Set(entries.value.map((e) => e.kind));
    return [...set].sort((a, b) => a.localeCompare(b));
  });

  const categoryCount = computed(() => categories.value.length);

  const statusLabel = computed(() => {
    if (!busy.value) return "Ready";
    if (phase.value === "scan") return "Scanning…";
    if (phase.value === "clean") return "Working…";
    return "Working…";
  });

  const hasScanned = computed(() => {
    const root = path.value.trim();
    return root.length > 0 && root === lastScannedPath.value;
  });

  const canClean = computed(() => {
    const root = path.value.trim();
    return (
      !busy.value &&
      root.length > 0 &&
      root === lastScannedPath.value &&
      selectedEntries.value.length > 0
    );
  });

  function showToast(type: ToastMessage["type"], title: string, body: string) {
    toast.value = { type, title, body };
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      toast.value = null;
    }, 3600);
  }

  function dismissToast() {
    toast.value = null;
    if (toastTimer) clearTimeout(toastTimer);
  }

  function countKind(kind: string): number {
    return entries.value.filter((e) => e.kind === kind).length;
  }

  function sharePercent(sizeBytes: number): number {
    if (!filteredTotal.value) return 0;
    return Math.max(4, (sizeBytes / filteredTotal.value) * 100);
  }

  async function chooseFolder() {
    error.value = null;
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected == null) return;
    setPath(selected as string);
    saveLastPath(path.value);
  }

  function setPath(next: string) {
    const normalized = next;
    if (normalized === path.value) {
      return;
    }
    path.value = normalized;
    entries.value = [];
    filterKind.value = "all";
    showConfirm.value = false;
    error.value = null;
    lastScannedPath.value = "";
    selectedPaths.value = new Set();
  }

  function selectAll(list: JunkEntry[]) {
    selectedPaths.value = new Set(list.map((e) => e.path));
  }

  function togglePath(path: string) {
    const next = new Set(selectedPaths.value);
    if (next.has(path)) next.delete(path);
    else next.add(path);
    selectedPaths.value = next;
  }

  function toggleVisible(on: boolean) {
    const next = new Set(selectedPaths.value);
    for (const e of filteredEntries.value) {
      if (on) next.add(e.path);
      else next.delete(e.path);
    }
    selectedPaths.value = next;
  }

  async function scan() {
    const root = path.value.trim();
    if (!root) {
      error.value = "Choose or enter a folder path first.";
      showToast("err", "Path required", "Choose or enter a folder path first.");
      return;
    }

    busy.value = true;
    phase.value = "scan";
    error.value = null;
    showConfirm.value = false;

    try {
      saveLastPath(root);
      const result = await invoke<JunkEntryDto[]>("scan", { path: root });
      entries.value = mapEntries(result);
      lastScannedPath.value = root;
      filterKind.value = "all";
      selectAll(entries.value);
      showToast(
        "ok",
        "Scan complete",
        `${entries.value.length} folders · ${formatBytes(totalBytes.value)}`,
      );
    } catch (e) {
      entries.value = [];
      lastScannedPath.value = "";
      selectedPaths.value = new Set();
      error.value = String(e);
      showToast("err", "Scan failed", String(e));
    } finally {
      busy.value = false;
      phase.value = "idle";
    }
  }

  function openConfirm() {
    if(!canClean.value) return;
    showConfirm.value = true;
  }

  function closeConfirm() {
    showConfirm.value = false;
  }

  /** Live clean checked ∩ visible rows after modal confirm. */
  async function confirmClean() {
    if(!canClean.value) {
      showConfirm.value = false;
      return;
    }
    const targets = selectedEntries.value;
    if (!targets.length) {
      showConfirm.value = false;
      return;
    }

    const reclaimed = selectedTotal.value;
    const n = targets.length;
    const paths = targets.map((e) => e.path);

    busy.value = true;
    phase.value = "clean";
    error.value = null;
    showConfirm.value = false;

    try {
      const cleanedCount = await invoke<number>("clean", {
        paths,
        dryRun: false,
      });

      // Refresh scan results under the same root when possible.
      const root = path.value.trim();
      if (root) {
        const result = await invoke<JunkEntryDto[]>("scan", { path: root });
        entries.value = mapEntries(result);
        selectAll(entries.value);
      } else {
        entries.value = [];
        selectedPaths.value = new Set();
      }
      filterKind.value = "all";

      if (cleanedCount === n) {
        showToast(
          "ok",
          "Cleaned",
          `Removed ${cleanedCount} of ${n} folders · ${formatBytes(reclaimed)} reclaimed`,
        );
      } else if (cleanedCount === 0) {
        showToast("err", "Clean failed", `Removed 0 of ${n} folders. Nothing was deleted (check permissions or locks).`);
      } else {
        showToast("warn", "Partially cleaned", `Removed ${cleanedCount} of ${n} folders. Some paths failed - reclaimed size not estimated. List refreshed.`);
      }
    } catch (e) {
      error.value = String(e);
      showToast("err", "Clean failed", String(e));
    } finally {
      busy.value = false;
      phase.value = "idle";
    }
  }

  return {
    path,
    entries,
    filterKind,
    busy,
    phase,
    showConfirm,
    toast,
    error,
    filteredEntries,
    selectedPaths,
    selectedEntries,
    filteredTotal,
    selectedTotal,
    totalBytes,
    largest,
    categories,
    categoryCount,
    statusLabel,
    formatBytes,
    countKind,
    sharePercent,
    showToast,
    dismissToast,
    chooseFolder,
    setPath,
    scan,
    togglePath,
    toggleVisible,
    openConfirm,
    closeConfirm,
    confirmClean,
    canClean,
    hasScanned,
  };
}
