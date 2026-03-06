<template>
  <app-dropdown ref="dropdown">
    <template #trigger>
      <app-button variant="secondary" class="cursor-pointer flex items-center justify-center" @click="toggle">
        <span class="sr-only">
          Toggle dropdown
        </span>

        <nuxt-icon name="mi:options-vertical" class="size-6" />
      </app-button>
    </template>

    <template #default>
      <div class="w-48">
        <item-option
          v-for="(option, index) in getOpts" :key="index"
          class="p-2 text-sm"
          :icon="option.icon"
          :label="option.label"
          @click="handleOption(option.callback)"
        />
      </div>
    </template>
  </app-dropdown>
</template>

<script lang="ts" setup>
// Props
const props = defineProps<{
  options: {
    icon?: string
    label: string
    callback?: () => void
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

function handleOption(callback: (() => void) | undefined) {
  callback?.()
  close()
}

defineExpose({
  open,
  close,
  toggle,
})
</script>
