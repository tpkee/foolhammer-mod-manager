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
        <button
          v-for="(option, index) in options" :key="index"
          class="p-2 text-sm hover:bg-gray-700 flex items-center gap-2 w-full text-left cursor-pointer"
          @click="handleOption(option.callback, close)"
        >
          <nuxt-icon v-if="option.icon" :name="option.icon" class="size-4" />
          {{ option.label }}
        </button>
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
