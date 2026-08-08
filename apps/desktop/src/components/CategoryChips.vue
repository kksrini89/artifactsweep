<script setup lang="ts">
defineProps<{
  categories: string[];
  filterKind: string | "all";
  totalCount: number;
  countKind: (kind: string) => number;
}>();

const emit = defineEmits<{
  select: [kind: string | "all"];
}>();
</script>

<template>
  <div class="d-flex flex-wrap gap-2" role="group" aria-label="Filter by type">
    <button
      type="button"
      class="as-chip"
      :class="{ active: filterKind === 'all' }"
      @click="emit('select', 'all')"
    >
      All<span class="as-chip-n">{{ totalCount }}</span>
    </button>
    <button
      v-for="c in categories"
      :key="c"
      type="button"
      class="as-chip"
      :class="{ active: filterKind === c }"
      @click="emit('select', c)"
    >
      {{ c }}<span class="as-chip-n">{{ countKind(c) }}</span>
    </button>
  </div>
</template>
