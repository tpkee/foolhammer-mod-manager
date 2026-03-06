<template>
  <table-row :columns="columns">
    <template #order>
      <div class="flex items-center">
        <span
          class="drag-handle cursor-grab active:cursor-move overflow-hidden transition-all duration-300 ease-in-out"
          :class="canDrag ? 'max-w-8 opacity-100 mr-2.5' : 'max-w-0 opacity-0 pointer-events-none'"
        >
          <nuxt-icon name="mi:reorder" class="size-6" />
          <span class="sr-only">Drag</span>
        </span>
        <app-input
          v-if="canReorder"
          v-model="order"
          label="Order number"
          sr-only-label
          type="number"
          :min="1"
          class="w-20"
        />
        <strong v-else class="inline-flex items-center gap-1 tabular-nums text-gray-400">
          <span class="text-gray-600 text-xs select-none font-light" aria-hidden>#</span>{{ order }}
        </strong>
      </div>
    </template>

    <template #enabled>
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
    </template>

    <template #pack>
      <div class="flex items-center gap-2.5 min-w-0">
        <div class="size-9 shrink-0 rounded">
          <img v-if="getImage" :src="getImage" alt="" class="size-10 rounded-[inherit] object-contain">
          <div v-else class="size-[inherit] rounded-[inherit] bg-gray-700" />
        </div>
        <p class="truncate" :title="name">
          {{ name }}
        </p>
      </div>
    </template>

    <template #lastUpdate>
      <span class="text-xs text-gray-400">
        <time v-if="getLastUpdate" :datetime="getLastUpdate">{{ getLastUpdate }}</time>
      </span>
    </template>

    <template #actions>
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
    </template>
  </table-row>
</template>

<script lang="ts" setup>
import type { AppTableColumn } from '~/types/common/AppTable'
import { convertFileSrc } from '@tauri-apps/api/core'

const props = defineProps<{
  columns: AppTableColumn[]
  name: string
  enabled: boolean
  order: Nullable<number>
  image?: Nullable<string>
  lastUpdated: Nullable<string>
  canEnable?: boolean
  canReorder?: boolean
  canDrag?: boolean
  errors?: ModError[]
}>()

const emit = defineEmits<{
  status: [value: boolean]
  order: [value: number]
  refresh: []
}>()

const order = defineModel<Nullable<number>>('order', { required: true })
const enabled = defineModel<Nullable<boolean>>('enabled', { default: false })

const gameStore = useGameStore()

const getLastUpdate = computed(() => {
  if (!props.lastUpdated)
    return ''
  return new Date(props.lastUpdated).toLocaleDateString()
})

const getImage = computed(() => props.image ? convertFileSrc(props.image) : null)

const getOptions = computed(() => [
  { icon: 'mi:delete', label: 'Delete from profile', callback: deleteFromProfile },
])

async function deleteFromProfile() {
  try {
    await useTauriInvoke('remove_profile_mods', {
      gameId: gameStore.selectedGame,
      profileId: gameStore.selectedProfile,
      mods: [props.name],
    })
    emit('refresh')
  }
  catch (err) {
    console.error('Failed to delete mod from profile:', err)
  }
}
</script>
