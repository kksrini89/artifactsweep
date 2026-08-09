<script setup lang="ts">
import type { JunkEntry } from "../types/junk";
import { isHighRiskKind } from "../utils/kind";

defineProps<{
  entries: JunkEntry[];
  hasAnyResults: boolean;
  canClean: boolean;
  formatBytes: (n: number) => string;
  sharePercent: (sizeBytes: number) => number;
}>();

const emit = defineEmits<{
  clean: [];
}>();
</script>

<template>
  <div class="card as-card">
    <div
      class="d-flex flex-wrap align-items-center justify-content-between gap-2 px-3 py-3 border-bottom"
      style="border-color: var(--as-border) !important"
    >
      <h3 class="m-0" style="font-size: 13px; font-weight: 600">
        Artifact folders
      </h3>
      <div v-if="entries.length" class="d-flex flex-wrap gap-2">
        <button
          type="button"
          class="btn btn-danger btn-sm"
          :disabled="!canClean"
          @click="emit('clean')"
        >
          Clean
        </button>
      </div>
    </div>

    <div v-if="!hasAnyResults" class="as-empty">
      <strong>Nothing scanned yet</strong>
      Choose a project root and hit Scan folder. Results will appear with sizes
      and categories.
    </div>

    <div
      v-else-if="!entries.length"
      class="as-empty"
    >
      <strong>No folders in this filter</strong>
      Try another category chip or All.
    </div>

    <div v-else class="as-table-wrap">
      <table class="table table-sm table-hover align-middle mb-0">
        <thead>
          <tr>
            <th scope="col" style="width: 6.5rem">Size</th>
            <th scope="col" style="width: 7rem">Share</th>
            <th scope="col" style="width: 8rem">Type</th>
            <th scope="col">Location</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="e in entries" :key="e.path">
            <td class="as-size">{{ formatBytes(e.sizeBytes) }}</td>
            <td>
              <div class="as-share-bar" :title="`${sharePercent(e.sizeBytes).toFixed(0)}%`">
                <i :style="{ width: sharePercent(e.sizeBytes) + '%' }" />
              </div>
            </td>
            <td>
              <span class="badge as-kind">{{ e.kind }}</span>
              <span
                v-if="isHighRiskKind(e.kind)"
                class="badge as-risk ms-1"
                title="Name can appear outside build caches"
              >
                risk
              </span>
            </td>
            <td class="as-path text-truncate" :title="e.path">
              {{ e.path }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
