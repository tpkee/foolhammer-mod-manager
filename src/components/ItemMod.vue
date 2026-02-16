<template>
  <div
    class="grid grid-cols-12 p-2.5 items-center gap-2.5 text-left"
  >
    <div class="flex items-center gap-2.5 col-span-2">
      <button class="cursor-grab active:cursor-move text-left disabled:pointer-events-none" :disabled="!canReorder">
        <nuxt-icon name="mi:reorder" class="size-6" />
        <span class="sr-only">Drag</span>
      </button>
      <app-input :model-value="order" :disabled="!canReorder" label="Order number" sr-only-label type="number" :min="1" class="w-20" @update:model-value="emit('order', $event)" />
    </div>
    <div>
      <app-tooltip :content="name" :disabled="canEnable">
        <app-checkbox :model-value="enabled" label="Is enabled?" sr-only-label :disabled="!canEnable" @update:model-value="emit('status', $event)" />
        <template #content>
          It's not possible to enable this mod, probably the file doesn't exist in the disk anymore
        </template>
      </app-tooltip>
    </div>
    <div class="flex items-center gap-2.5 col-span-5">
      <div class="size-9 rounded">
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
    <app-options class="justify-self-end" :options="getOptions" />
  </div>
</template>

<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core'
import { profileResponseToRequest } from '~/utils/dto'

// Props
const props = defineProps<{
  name: string
  enabled: boolean
  order: Nullable<number>
  image?: Nullable<string>
  lastUpdated: Nullable<string>
  canEnable?: boolean
  canReorder?: boolean
}>()

// Emits
const emit = defineEmits<{
  status: [value: boolean]
  order: [value: number]
  refresh: []
}>()

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
    {
      icon: 'mi:close',
      label: 'Delete from disk',
      callback: () => {
        console.log('Delete from disk')
      },
    },
  ]
})

async function deleteFromProfile() {
  const profile = gameStore.getProfile
  const profileMods = gameStore.getProfileMods

  if (!profile)
    return

  try {
    const updatedMods = profileMods.filter(m => m.name !== props.name)

    const profileRequest = profileResponseToRequest(
      { ...profile, mods: updatedMods },
      gameStore.selectedGame!,
    )

    await useTauriInvoke('update_profile', { payload: profileRequest })
    emit('refresh')
  }
  catch (err) {
    console.error('Failed to delete mod from profile:', err)
  }
}
</script>
