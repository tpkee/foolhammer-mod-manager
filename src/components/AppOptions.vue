<template>
  <app-dropdown>
    <template #trigger="{ toggle }">
      <button class="cursor-pointer" @click="toggle">
        <span class="sr-only">
          Toggle dropdown
        </span>

        <nuxt-icon name="mi:options-vertical" class="size-6" />
      </button>
    </template>

    <template #default="{ close }">
      <div class="w-48">
        <item-option
          v-for="(option, index) in options" :key="index"
          class="p-2 text-sm"
          :icon="option.icon"
          :label="option.label"
          @click="handleOption(option.callback, close)"
        />
      </div>
    </template>
  </app-dropdown>
</template>

<script lang="ts" setup>
// Props
defineProps<{
  options: Array<{
    icon?: string
    label: string
    callback?: () => void
  }>
}>()

// Functions
function handleOption(callback: (() => void) | undefined, close: () => void) {
  callback?.()
  close()
}
</script>
