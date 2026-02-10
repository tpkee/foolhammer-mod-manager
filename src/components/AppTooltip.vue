<template>
  <div ref="triggerRef" class="group/tooltip relative inline-block">
    <Teleport to="body">
      <transition name="tooltip-fade" mode="out-in">
        <div
          v-if="isEnabled && isVisible"
          ref="tooltipRef"
          class="fixed z-50 w-fit p-2 bg-gray-800 rounded border border-gray-700 shadow"
          :style="floatingStyles"
        >
          <slot name="content" />
        </div>
      </transition>
    </Teleport>

    <div
      :class="{
        'underline cursor-pointer': isEnabled,
      }"
    >
      <slot />
    </div>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  disable?: boolean
}>()

// Refs
const triggerRef = ref<HTMLElement | null>(null)
const tooltipRef = ref<HTMLElement | null>(null)
const isVisible = ref(false)

const isEnabled = computed(() => !props.disable)

// Floating position composable
const { floatingStyles, updatePosition } = useFloatingPosition(triggerRef, tooltipRef, {
  offset: 8,
  isVisible,
  preferredDirection: 'horizontal',
})

// Show tooltip on hover/focus
function showTooltip() {
  if (isEnabled.value) {
    isVisible.value = true
    nextTick(() => updatePosition())
  }
}

function hideTooltip() {
  isVisible.value = false
}

// Mouse events
useEventListener(triggerRef, 'mouseenter', showTooltip)
useEventListener(triggerRef, 'mouseleave', hideTooltip)
useEventListener(triggerRef, 'focus', showTooltip, { capture: true })
useEventListener(triggerRef, 'blur', hideTooltip, { capture: true })
</script>

<style scoped>
.tooltip-fade-enter-active,
.tooltip-fade-leave-active {
  transition: opacity 0.15s ease;
}

.tooltip-fade-enter-from,
.tooltip-fade-leave-to {
  opacity: 0;
}
</style>
