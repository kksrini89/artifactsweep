<script setup lang="ts">
import packageJson from "../../package.json";
import { useScanClean } from "../composables/useScanClean";
import AppSidebar from "../components/AppSidebar.vue";
import MetricCards from "../components/MetricCards.vue";
import PathScanner from "../components/PathScanner.vue";
import CategoryChips from "../components/CategoryChips.vue";
import ResultsTable from "../components/ResultsTable.vue";
import ConfirmCleanModal from "../components/ConfirmCleanModal.vue";
import AppToast from "../components/AppToast.vue";

const {
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
  selectedTotal,
  totalBytes,
  largest,
  categories,
  categoryCount,
  statusLabel,
  formatBytes,
  countKind,
  sharePercent,
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
} = useScanClean();

const appVersion = packageJson.version;

</script>

<template>
  <div class="as-app d-flex overflow-hidden">
    <AppSidebar :version="appVersion" />

    <main class="as-main flex-grow-1 d-flex flex-column overflow-hidden">
      <header
        class="d-flex flex-wrap align-items-start justify-content-between gap-3 px-4 pt-4 pb-2"
      >
        <div>
          <h1 class="as-page-title">Reclaim space</h1>
          <p class="as-page-sub">
            Find generated build artifacts under a folder, then clean with
            confidence.
          </p>
        </div>
        <span class="as-status-pill" :class="{ busy }">
          <span v-if="busy" class="as-spinner" aria-hidden="true" />
          {{ statusLabel }}
        </span>
      </header>

      <div class="as-main-scroll flex-grow-1 overflow-auto px-4 pb-4">
        <div class="d-flex flex-column gap-3">
          <MetricCards
            :filtered-total="selectedTotal"
            :filtered-count="selectedEntries.length"
            :largest="largest"
            :category-count="categoryCount"
            :total-bytes="totalBytes"
            :format-bytes="formatBytes"
          />

          <PathScanner
            :path="path"
            :busy="busy"
            :scanning="phase === 'scan'"
            @update:path="setPath"
            @browse="chooseFolder"
            @scan="scan"
          />

          <div
            v-if="error"
            class="alert alert-danger mb-0"
            role="alert"
          >
            {{ error }}
          </div>

          <CategoryChips
            v-if="entries.length"
            :categories="categories"
            :filter-kind="filterKind"
            :total-count="entries.length"
            :count-kind="countKind"
            @select="filterKind = $event"
          />

          <ResultsTable
            :entries="filteredEntries"
            :selected-paths="selectedPaths"
            :has-scanned="hasScanned"
            :has-raw-results="entries.length > 0"
            :can-clean="canClean"
            :format-bytes="formatBytes"
            :share-percent="sharePercent"
            @clean="openConfirm"
            @toggle="togglePath"
            @toggle-visible="toggleVisible"
          />

          <p
            v-if="entries.length"
            class="mb-0 small"
            style="color: var(--as-dim)"
          >
            Clean deletes
            <strong style="color: var(--as-text)">{{ selectedEntries.length }}</strong>
            selected folder{{ selectedEntries.length === 1 ? "" : "s" }}.
            Uncheck to keep.
          </p>
        </div>
      </div>
    </main>

    <ConfirmCleanModal
      :open="showConfirm"
      :count="selectedEntries.length"
      :size-label="formatBytes(selectedTotal)"
      :root-path="path"
      :busy="busy"
      @cancel="closeConfirm"
      @confirm="confirmClean"
    />

    <AppToast :toast="toast" @dismiss="dismissToast" />
  </div>
</template>
