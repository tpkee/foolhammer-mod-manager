<template>
  <app-dropdown ref="dropdown">
    <template #trigger>
      <app-button variant="secondary" class="cursor-pointer flex items-center justify-center" @click="toggle">
        <span class="sr-only">
          Toggle dropdown
        </span>

        <IconMiOptionsVertical class="size-6" />
      </app-button>
    </template>

    <template #default>
      <div class="w-48">
        <item-option
          v-for="(option, index) in getOpts" :key="index"
          class="p-2 text-sm"
          :icon="option.icon"
          :label="option.label"
          :callback="wrapCallback(option.callback)"
        />
      </div>
    </template>
  </app-dropdown>
</template>

<script lang="ts" setup>
import type { FunctionalComponent, SVGAttributes } from 'vue'

// Props
const props = defineProps<{
  options: {
    icon?: FunctionalComponent<SVGAttributes>
    label: string
    callback?: () => void | Promise<void>
    hide?: boolean
  }[]
}>()

// Template refs
const refDropdown = useTemplateRef('dropdown')

// Computed
const getOpts = computed(() => props.options.filter(opt => !opt.hide))

// Functions
function open() {
  refDropdown.value?.open()
}

function close() {
  refDropdown.value?.close()
}

function toggle() {
  refDropdown.value?.toggle()
}

function wrapCallback(callback: (() => void | Promise<void>) | undefined) {
  return async () => {
    await callback?.()
    close()
  }
}

defineExpose({
  open,
  close,
  toggle,
})
</script>
