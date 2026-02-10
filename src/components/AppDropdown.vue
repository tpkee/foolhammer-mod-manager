<template>
  <div ref="triggerRef" class="relative inline-block">
    <slot name="trigger" :toggle="toggle" :is-open="isOpen" />

    <Teleport to="body">
      <transition name="dropdown-fade" mode="out-in">
        <div
          v-if="isOpen"
          ref="dropdownRef"
          class="fixed z-50 bg-gray-800 rounded shadow-lg"
          :style="floatingStyles"
        >
          <slot :close="close" />
        </div>
      </transition>
    </Teleport>
  </div>
</template>

<script lang="ts" setup>
// Props
const props = withDefaults(defineProps<{
  /** Offset in pixels from the trigger */
  offset?: number
}>(), {
  offset: 8,
})

// Refs
const triggerRef = ref<HTMLElement | null>(null)
const dropdownRef = ref<HTMLElement | null>(null)
const isOpen = ref(false)

// Floating position composable
const { floatingStyles, updatePosition } = useFloatingPosition(triggerRef, dropdownRef, {
  offset: props.offset,
  isVisible: isOpen,
})

// Functions
function toggle() {
  isOpen.value ? close() : open()
}

function open() {
  isOpen.value = true
  nextTick(() => updatePosition())
}

function close() {
  isOpen.value = false
}

// Close on click outside
onClickOutside(dropdownRef, () => close(), { ignore: [triggerRef] })

// Close on escape
useEventListener('keydown', (e: KeyboardEvent) => {
  if (e.key === 'Escape' && isOpen.value) {
    close()
  }
})

// Expose for parent components
defineExpose({ open, close, toggle })
</script>

<style scoped>
.dropdown-fade-enter-active,
.dropdown-fade-leave-active {
  transition: opacity 0.15s ease;
}

.dropdown-fade-enter-from,
.dropdown-fade-leave-to {
  opacity: 0;
}
</style>
