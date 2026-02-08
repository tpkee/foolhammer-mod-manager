<template>
  <div class="grid grid-cols-12 p-2.5 items-center gap-2.5 text-left">
    <div class="flex items-center gap-2.5 col-span-2">
      <button class="cursor-grab active:cursor-move text-left">
        <nuxt-icon name="mi:reorder" class="size-6" />
      </button>
      <app-input v-model.number="order" label="Order number" sr-only-label type="number" :min="1" class="w-20" />
    </div>
    <app-checkbox v-model="isEnabled" label="Is enabled?" sr-only-label />
    <div class="flex items-center gap-2.5 col-span-3">
      <div class="size-9 rounded">
        <img v-if="getImage" :src="getImage" alt="" class="size-10 rounded-[inherit] object-contain">
        <div v-else class="size-[inherit] rounded-[inherit] bg-gray-700" />
      </div>
      <app-tooltip class="relative">
        <template #content>
          <p class="whitespace-nowrap">
            {{ name }}
          </p>
        </template>
        <p class="truncate" :title="pack">
          {{ pack }}
        </p>
      </app-tooltip>
    </div>
    <time :datetime="getLastUpdate" class="text-xs text-gray-500 col-span-5">
      {{ getLastUpdate }}
    </time>

    <app-options class="justify-self-end" :options="getOptions" />
  </div>
</template>

<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core'

// Props
const props = defineProps<{
  name: string
  image?: string
  pack: string
  lastUpdate: string | Date | number
}>()

// Model
const isEnabled = defineModel('enabled', { type: Boolean, required: true })
const order = defineModel('order', { type: Number, required: true })

// Computed
const getLastUpdate = computed(() => {
  const date = new Date(props.lastUpdate)
  return date.toLocaleDateString()
})
const getImage = computed(() => {
  return props.image ? convertFileSrc(props.image) : null
})
const getOptions = computed(() => {
  return [
    {
      icon: 'mi:delete',
      label: 'Delete from profile',
      callback: () => {
        console.log('Delete from profile')
      },
    },
    {
      icon: 'mi:close',
      label: 'Delete from disk',
      callback: () => {
        console.log('Delete from disk')
      },
    },
  ]
})
</script>
