<script setup lang="ts">
import type { ToastMessage } from "../types/junk";

defineProps<{
  toast: ToastMessage | null;
}>();

const emit = defineEmits<{
  dismiss: [];
}>();
</script>

<template>
  <div
    v-if="toast"
    class="as-toast"
    :class="toast.type"
    role="status"
    @click="emit('dismiss')"
  >
    <strong>{{ toast.title }}</strong>
    {{ toast.body }}
  </div>
</template>
