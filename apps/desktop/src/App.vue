<script setup lang="ts">
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

type JunkEntry = {
  path: string;
  size_bytes: number;
}

const rootPath = ref<string | null>(null);
const entries = ref<JunkEntry[]>([]);
const busy = ref(false);
const error = ref<string | null>(null);
const selectedPaths = ref<string[]>([]);
const status = ref<string | null>(null);

const totalBytes = computed(() =>
  entries.value.reduce((sum, e) => sum + e.size_bytes, 0),
);

function formatSize(bytes: number): string {
  const units = ["B", "KiB", "MiB", "GiB"];
  let n = bytes;
  let i = 0;
  while (n >= 1024 && i < units.length - 1) {
    n /= 1024;
    i++;
  }
  return `${n.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

async function chooseFolder() {
  error.value = null;
  status.value = null;
  const selected = await open({
    directory: true,
    multiple: false
  });

  if (selected == null) {
    return;
  }

  rootPath.value = selected as string;
  entries.value = [];
  selectedPaths.value = [];

}

async function runScan() {
  if (!rootPath.value) {
    error.value = "Choose a folder first.";
    return;
  }
  busy.value = true;
  error.value = null;
  status.value = null;
  try {
    entries.value = await invoke<JunkEntry[]>("scan", {
      path: rootPath.value,
    });
    status.value = null;
  } catch (e) {
    error.value = String(e);
    entries.value = [];
  } finally {
    busy.value = false;
  }
}

function onSelectedPath(path: string, checked: boolean) {
  if(checked) {
    if(!selectedPaths.value.includes(path)) {
      selectedPaths.value = [...selectedPaths.value, path];
    }
  } else {
    selectedPaths.value = selectedPaths.value.filter((p) => p !== path);
  }
}

async function runClean() {
  console.log(`Clean action is performed...`);
  busy.value = true;
  error.value = null;
  try {
    const cleanedCount = await invoke<number>('clean', {
      paths: selectedPaths.value,
      dryRun: false,
    });
    // list refresh after cleaned the selected paths,
    selectedPaths.value = [];
    status.value = `Removed ${cleanedCount} folder(s).`;
    if(rootPath.value) {
      entries.value = await invoke<JunkEntry[]>('scan', {
        path: rootPath.value,
      });
    } else {
      entries.value = [];
    }
  } catch (err) {
    error.value = String(err);
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <main class="wrap">
    <h1>ArtifactSweep</h1>

    <div class="row">
      <button type="button" @click="chooseFolder" :disabled="busy">
        Choose folder
      </button>
      <button
        type="button"
        @click="runScan"
        :disabled="busy || !rootPath"
      >
        {{ busy ? "Scanning…" : "Scan" }}
      </button>
    </div>

    <p class="path" v-if="rootPath">Folder: {{ rootPath }}</p>
    <p class="error" v-if="error">{{ error }}</p>
    <p class="status" v-if="status">{{ status }}</p>

    <p v-if="entries.length">
      Found {{ entries.length }} folder(s). Total: {{ formatSize(totalBytes) }}
    </p>
    <p v-else-if="rootPath && !busy && !error">No results yet — click Scan.</p>

    <div class="column" v-if="entries.length">
      <button 
        type="button" 
        @click="runClean"
        :disabled="busy || selectedPaths.length === 0">
        {{ busy ? "Working..." : "Clean" }}
      </button>
      <table>
        <thead>
          <tr>
            <th></th>
            <th>Size</th>
            <th>Path</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(e, i) in entries" :key="i">
            <td>
              <input 
              type="checkbox"
              :checked="selectedPaths.includes(e.path)"
              :disabled="busy"
              @change="onSelectedPath(e.path, ($event.target as HTMLInputElement).checked)"
              >
            </td>
            <td class="size">{{ formatSize(e.size_bytes) }}</td>
            <td class="path-cell">{{ e.path }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </main>
</template>

<style scoped>
.wrap {
  max-width: 960px;
  margin: 0 auto;
  padding: 1.5rem;
  font-family: system-ui, sans-serif;
}
.row {
  display: flex;
  gap: 0.75rem;
  margin: 1rem 0;
}

.column {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin: 1rem 0;
}
button {
  padding: 0.5rem 1rem;
  cursor: pointer;
}
button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.path {
  word-break: break-all;
  color: #333;
}
.error {
  color: #b00020;
}
table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 1rem;
}
th,
td {
  border-bottom: 1px solid #ddd;
  padding: 0.5rem;
  text-align: left;
}
.size {
  width: 7rem;
  white-space: nowrap;
}
.path-cell {
  word-break: break-all;
  font-family: ui-monospace, monospace;
  font-size: 0.9rem;
}

.status {
  color: #0a7a32;
}
</style>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>