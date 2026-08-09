<script setup lang="ts">
defineProps<{
  path: string;
  busy: boolean;
  /** True only while a scan invoke is in flight. */
  scanning?: boolean;
}>();

const emit = defineEmits<{
  "update:path": [value: string];
  browse: [];
  scan: [];
}>();

function onEnter(e: KeyboardEvent) {
  if (e.key === "Enter") {
    e.preventDefault();
    emit("scan");
  }
}
</script>

<template>
  <div class="card as-card">
    <div class="card-body">
      <label class="as-label" for="root-path">Root path</label>
      <div class="d-flex flex-wrap gap-2 align-items-stretch">
        <input
          id="root-path"
          type="text"
          class="form-control flex-grow-1"
          style="min-width: 12rem"
          :value="path"
          :disabled="busy"
          placeholder="~/projects or choose a folder"
          spellcheck="false"
          autocomplete="off"
          @input="
            emit(
              'update:path',
              ($event.target as HTMLInputElement).value,
            )
          "
          @keydown="onEnter"
        />
        <button
          type="button"
          class="btn btn-outline-secondary"
          :disabled="busy"
          @click="emit('browse')"
        >
          Browse…
        </button>
        <button
          type="button"
          class="btn btn-primary"
          :disabled="busy"
          @click="emit('scan')"
        >
          {{ scanning ? "Scanning…" : "Scan folder" }}
        </button>
      </div>
    </div>
  </div>
</template>
