<template>
  <Teleport to="body">
    <transition name="modal-fade" mode="out-in">
      <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center">
        <!-- Backdrop -->
        <div
          role="button"
          tabindex="0"
          class="absolute inset-0 bg-black/50"
          @click="closeOnBackdrop"
          @keydown.escape="closeOnBackdrop"
        />

        <!-- Modal -->
        <div
          ref="modalRef"
          class="relative z-10 bg-gray-800 rounded shadow-lg max-h-[90vh] overflow-y-auto w-[90vw]"
        >
          <slot :close="close" />
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<script lang="ts" setup>
const props = withDefaults(
  defineProps<{
    /** Allow closing by clicking the backdrop */
    closeOnBackdrop?: boolean
  }>(),
  {
    closeOnBackdrop: true,
  },
)

const modalRef = useTemplateRef('modalRef')
const isOpen = ref(false)

function open() {
  isOpen.value = true
}

function close() {
  isOpen.value = false
}

function closeOnBackdrop() {
  if (props.closeOnBackdrop) {
    close()
  }
}

// Close on escape key
useEventListener('keydown', (e: KeyboardEvent) => {
  if (e.key === 'Escape' && isOpen.value) {
    close()
  }
})

// Expose for parent components
defineExpose({ open, close })
</script>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.2s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

.modal-fade-enter-active > div:nth-child(2),
.modal-fade-leave-active > div:nth-child(2) {
  transition: transform 0.2s ease;
}

.modal-fade-enter-from > div:nth-child(2),
.modal-fade-leave-to > div:nth-child(2) {
  transform: scale(0.95);
}
</style>
