<template>
  <div class="grid grid-cols-12 p-2.5 items-center gap-2.5 text-left">
    <div class="flex items-center gap-2.5 col-span-2">
      <button class="cursor-grab active:cursor-move text-left disabled:pointer-events-none" :disabled="!canReorder">
        <nuxt-icon name="mi:reorder" class="size-6" />
        <span class="sr-only">Drag</span>
      </button>
      <app-input
        v-model="order"
        :disabled="!canReorder"
        label="Order number"
        sr-only-label
        type="number"
        :min="1"
        class="w-20"
      />
    </div>
    <div>
      <app-tooltip :content="name" :disabled="canEnable">
        <app-checkbox
          v-model="enabled"
          label="Is enabled?"
          sr-only-label
          :disabled="!canEnable"
        />
        <template #content>
          It's not possible to enable this mod, probably the file doesn't exist in the disk anymore
        </template>
      </app-tooltip>
    </div>
    <div class="flex items-center gap-2.5 col-span-5 min-w-0">
      <div class="size-9 shrink-0 rounded">
        <img v-if="getImage" :src="getImage" alt="" class="size-10 rounded-[inherit] object-contain">
        <div v-else class="size-[inherit] rounded-[inherit] bg-gray-700" />
      </div>
      <p class="truncate" :title="name">
        {{ name }}
      </p>
    </div>
    <div class="text-xs text-gray-400 col-span-3">
      <time v-if="getLastUpdate" :datetime="getLastUpdate">
        {{ getLastUpdate }}
      </time>
    </div>
    <div class="flex items-center justify-end gap-1.5">
      <app-tooltip v-if="errors && errors.length > 0">
        <nuxt-icon name="mi:warning" class="size-5 shrink-0 text-red-400 align-middle block" />
        <template #content>
          <ul class="text-xs space-y-1 max-w-56 list-disc">
            <li
              v-for="error in errors"
              :key="error.type"
              class="flex items-start gap-1.5"
            >
              {{ error.message }}
            </li>
          </ul>
        </template>
      </app-tooltip>
      <app-options :options="getOptions" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core'

// Props
const props = defineProps<{
  name: string
  enabled: boolean
  order: Nullable<number>
  image?: Nullable<string>
  lastUpdated: Nullable<string>
  canEnable?: boolean
  canReorder?: boolean
  errors?: ModError[]
}>()

// Emits
const emit = defineEmits<{
  status: [value: boolean]
  order: [value: number]
  refresh: []
}>()

const order = defineModel<Nullable<number>>('order', { required: true })
const enabled = defineModel<Nullable<boolean>>('enabled', { default: false })

// Store
const gameStore = useGameStore()

// Computed
const getLastUpdate = computed(() => {
  if (!props.lastUpdated)
    return ''
  const date = new Date(props.lastUpdated)
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
      callback: deleteFromProfile,
    },
    // {
    //   icon: 'mi:close',
    //   label: 'Delete from disk',
    //   callback: () => {
    //     console.log('Delete from disk')
    //   },
    // },
  ]
})

async function deleteFromProfile() {
  try {
    await useTauriInvoke('remove_profile_mods', { gameId: gameStore.selectedGame, profileId: gameStore.selectedProfile, mods: [props.name] })
    emit('refresh')
  }
  catch (err) {
    console.error('Failed to delete mod from profile:', err)
  }
}
</script>
