<template>
  <div class="fixed bottom-6 left-1/2 z-50 w-[min(92vw,560px)] -translate-x-1/2">
    <transition name="float-panel">
      <div v-if="visible" class="rounded-lg border border-gray-700 bg-gray-900/95 p-3 shadow-lg backdrop-blur">
        <div class="flex flex-wrap items-center gap-2.5">
          <p class="text-sm text-gray-300 mr-auto">
            Unsaved changes
          </p>
          <app-button variant="secondary" @click="$emit('cancel')">
            Cancel
          </app-button>
          <app-button @click="$emit('save')">
            Save
          </app-button>
          <app-button variant="secondary" :disabled="!canUndo" @click="$emit('undo')">
            Undo
          </app-button>
          <app-button variant="secondary" :disabled="!canRedo" @click="$emit('redo')">
            Redo
          </app-button>
        </div>
      </div>
    </transition>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  visible: boolean
  canUndo: boolean
  canRedo: boolean
}>()
</script>

<style scoped>
.float-panel-enter-active,
.float-panel-leave-active {
  transition:
    transform 200ms ease,
    opacity 200ms ease;
}

.float-panel-enter-from,
.float-panel-leave-to {
  transform: translateY(16px);
  opacity: 0;
}

.float-panel-enter-to,
.float-panel-leave-from {
  transform: translateY(0);
  opacity: 1;
}
</style>
