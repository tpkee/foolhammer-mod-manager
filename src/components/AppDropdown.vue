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

// Floating position
const floatingStyles = ref<Record<string, string>>({
  top: '0px',
  left: '0px',
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

function updatePosition() {
  const trigger = triggerRef.value
  const dropdown = dropdownRef.value
  if (!trigger || !dropdown)
    return

  const triggerRect = trigger.getBoundingClientRect()
  const dropdownRect = dropdown.getBoundingClientRect()
  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight

  // Measure available space in each direction
  const spaceBelow = viewportHeight - triggerRect.bottom - props.offset
  const spaceAbove = triggerRect.top - props.offset
  const spaceRight = viewportWidth - triggerRect.left
  const spaceLeft = triggerRect.right

  // Vertical: prefer below, flip to above if not enough room
  let top: number
  if (spaceBelow >= dropdownRect.height || spaceBelow >= spaceAbove) {
    top = triggerRect.bottom + props.offset
  }
  else {
    top = triggerRect.top - dropdownRect.height - props.offset
  }

  // Horizontal: prefer aligning start (left edge), flip to end (right edge) if not enough room
  let left: number
  if (spaceRight >= dropdownRect.width || spaceRight >= spaceLeft) {
    left = triggerRect.left
  }
  else {
    left = triggerRect.right - dropdownRect.width
  }

  // Clamp to viewport bounds
  top = Math.max(0, Math.min(top, viewportHeight - dropdownRect.height))
  left = Math.max(0, Math.min(left, viewportWidth - dropdownRect.width))

  floatingStyles.value = {
    top: `${top}px`,
    left: `${left}px`,
  }
}

// Close on click outside
onClickOutside(dropdownRef, () => close(), { ignore: [triggerRef] })

// Close on escape
useEventListener('keydown', (e: KeyboardEvent) => {
  if (e.key === 'Escape' && isOpen.value) {
    close()
  }
})

// Reposition on scroll (any ancestor) and resize
useEventListener('scroll', () => {
  if (isOpen.value)
    updatePosition()
}, { capture: true, passive: true })

useEventListener('resize', () => {
  if (isOpen.value)
    updatePosition()
}, { passive: true })

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
