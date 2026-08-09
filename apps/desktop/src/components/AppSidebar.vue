<script setup lang="ts">
defineProps<{
  version?: string;
}>();
</script>

<template>
  <aside class="as-sidebar d-flex flex-column flex-shrink-0 p-3">
    <div class="d-flex align-items-center gap-2 px-1 mb-4">
      <div class="as-brand-mark" aria-hidden="true">AS</div>
      <div>
        <p class="as-brand-title">ArtifactSweep</p>
      </div>
    </div>

    <nav class="d-flex flex-column gap-1" aria-label="Main">
      <button type="button" class="as-nav-btn active" aria-current="page">
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          aria-hidden="true"
        >
          <path d="M3 12h4l3-9 4 18 3-9h4" />
        </svg>
        Clean
      </button>
      <button type="button" class="as-nav-btn" disabled title="Coming soon">
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          aria-hidden="true"
        >
          <circle cx="12" cy="12" r="3" />
          <path
            d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"
          />
        </svg>
        Settings
      </button>
      <button type="button" class="as-nav-btn" disabled title="Coming soon">
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          aria-hidden="true"
        >
          <path d="M12 8v4l3 3" />
          <circle cx="12" cy="12" r="9" />
        </svg>
        History
      </button>
    </nav>

    <div class="as-sidebar-foot">
      Scan &amp; reclaim build artifacts
      <template v-if="version">
        <br />
        v{{ version }}
      </template>
    </div>
  </aside>
</template>
