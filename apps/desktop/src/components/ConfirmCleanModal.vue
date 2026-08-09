<script setup lang="ts">
import { nextTick, onUnmounted, ref, watch } from "vue";

const props = defineProps<{
  open: boolean;
  count: number;
  sizeLabel: string;
  rootPath: string;
  busy: boolean;
}>();

const emit = defineEmits<{
  cancel: [];
  confirm: [];
}>();

const confirmBtn = ref<HTMLButtonElement | null>(null);

function onKeydown(e: KeyboardEvent) {
  if (!props.open) return;
  if (e.key === "Escape") {
    e.preventDefault();
    if (!props.busy) emit("cancel");
  }
}

watch(
  () => props.open,
  async (isOpen) => {
    if (isOpen) {
      window.addEventListener("keydown", onKeydown);
      await nextTick();
      confirmBtn.value?.focus();
    } else {
      window.removeEventListener("keydown", onKeydown);
    }
  },
);

onUnmounted(() => {
  window.removeEventListener("keydown", onKeydown);
});
</script>

<template>
  <div
    v-if="open"
    class="as-modal-backdrop"
    role="presentation"
    @click.self="!busy && emit('cancel')"
  >
    <div
      class="as-modal"
      role="dialog"
      aria-modal="true"
      aria-labelledby="confirm-clean-title"
    >
      <div class="as-modal-header">
        <h3 id="confirm-clean-title">
          Delete {{ count }} folder{{ count === 1 ? "" : "s" }}?
        </h3>
      </div>
      <div class="as-modal-body">
        This will permanently remove
        <strong>{{ sizeLabel }}</strong>
        of build artifacts
        <template v-if="rootPath">
          under <strong>{{ rootPath }}</strong>
        </template>
        . Source projects stay; only matched artifact folders in the current
        filter are removed.
      </div>
      <div class="as-modal-footer">
        <button
          type="button"
          class="btn btn-outline-secondary btn-sm"
          :disabled="busy"
          @click="emit('cancel')"
        >
          Cancel
        </button>
        <button
          ref="confirmBtn"
          type="button"
          class="btn btn-danger btn-sm"
          :disabled="busy"
          @click="emit('confirm')"
        >
          {{ busy ? "Removing…" : "Delete now" }}
        </button>
      </div>
    </div>
  </div>
</template>
