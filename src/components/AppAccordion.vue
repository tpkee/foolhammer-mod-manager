<template>
  <div class="relative">
    <button
      type="button"
      class="w-full flex items-center justify-between gap-2 px-2 py-1.5 text-left border border-gray-700 rounded rounded-b-none"
      :class="{ 'opacity-60 cursor-not-allowed': disabled, 'border-b-transparent': isOpen }"
      :aria-expanded="isOpen"
      :aria-controls="contentId"
      :disabled="disabled"
      @click="toggle"
    >
      <div class="text-sm font-medium flex items-center gap-1">
        <slot name="title" />
      </div>
      <nuxt-icon name="mi:caret-down" class="size-4 transition-transform" :class="{ 'rotate-180': isOpen }" />
    </button>

    <div
      :id="contentId"
      ref="contentRef"
      class="overflow-hidden transition-[max-height,opacity] duration-200 ease  w-full z-999 border rounded-b border-gray-700 -mt-px pt-2.5"
      :style="contentStyles"
      role="region"
      :aria-hidden="!isOpen"
    >
      <div class="px-2 pb-2">
        <slot />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  disabled?: boolean
}>()

const contentRef = ref<HTMLElement | null>(null)
const contentId = useId()
const contentHeight = ref(0)

const isOpen = ref(false)

const contentStyles = computed(() => ({
  maxHeight: isOpen.value ? `${contentHeight.value}px` : '0px',
  opacity: isOpen.value ? '1' : '0',
}))

function updateContentHeight() {
  contentHeight.value = contentRef.value?.scrollHeight ?? 0
}

function open() {
  if (props.disabled) {
    return
  }
  isOpen.value = true
  nextTick(updateContentHeight)
}

function close() {
  isOpen.value = false
}

function toggle() {
  if (isOpen.value) {
    close()
    return
  }
  open()
}

watch(isOpen, (openState) => {
  if (openState) {
    nextTick(updateContentHeight)
  }
  else {
    contentHeight.value = 0
  }
})

onMounted(() => {
  if (isOpen.value) {
    updateContentHeight()
  }
})

onUpdated(() => {
  if (isOpen.value) {
    updateContentHeight()
  }
})

defineExpose({ open, close, toggle })
</script>
