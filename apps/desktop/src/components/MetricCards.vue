<script setup lang="ts">
import { computed } from "vue";
import type { JunkEntry } from "../types/junk";

const props = defineProps<{
  filteredTotal: number;
  filteredCount: number;
  largest: JunkEntry | null;
  categoryCount: number;
  totalBytes: number;
  formatBytes: (n: number) => string;
}>();

const circumference = 2 * Math.PI * 30;

const ringPct = computed(() => {
  if (!props.totalBytes) return 0;
  return Math.min(100, (props.filteredTotal / props.totalBytes) * 100);
});

const dashOffset = computed(
  () => circumference * (1 - ringPct.value / 100),
);

const ringLabel = computed(() =>
  props.filteredCount ? `${Math.round(ringPct.value)}%` : "—",
);
</script>

<template>
  <div class="row g-3">
    <div class="col-12 col-lg-5">
      <div class="card as-card h-100">
        <div class="card-body d-flex align-items-center gap-3">
          <div class="as-ring-wrap" aria-hidden="true">
            <svg width="72" height="72" viewBox="0 0 72 72">
              <circle
                cx="36"
                cy="36"
                r="30"
                fill="none"
                stroke="#1a2a1a"
                stroke-width="7"
              />
              <circle
                cx="36"
                cy="36"
                r="30"
                fill="none"
                stroke="#3ddc84"
                stroke-width="7"
                stroke-linecap="round"
                :stroke-dasharray="circumference"
                :stroke-dashoffset="dashOffset"
                style="transition: stroke-dashoffset 0.5s ease"
              />
            </svg>
            <div class="as-ring-center">{{ ringLabel }}</div>
          </div>
          <div class="min-w-0">
            <div class="as-label">Reclaimable</div>
            <p class="as-metric-value">{{ formatBytes(filteredTotal) }}</p>
            <p class="as-metric-sub">
              {{ filteredCount }} folder{{ filteredCount === 1 ? "" : "s" }}
              match filters
            </p>
          </div>
        </div>
      </div>
    </div>

    <div class="col-6 col-lg-3">
      <div class="card as-card h-100">
        <div class="card-body">
          <div class="as-label">Largest hit</div>
          <p class="as-metric-value amber">
            {{ largest ? formatBytes(largest.sizeBytes) : "—" }}
          </p>
          <p class="as-metric-sub">
            {{ largest ? largest.kind : "Scan to see results" }}
          </p>
        </div>
      </div>
    </div>

    <div class="col-6 col-lg-4">
      <div class="card as-card h-100">
        <div class="card-body">
          <div class="as-label">Categories</div>
          <p class="as-metric-value">{{ categoryCount || "—" }}</p>
          <p class="as-metric-sub">Artifact types in results</p>
        </div>
      </div>
    </div>
  </div>
</template>
