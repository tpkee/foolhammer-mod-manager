<template>
  <app-modal ref="modal" :close-on-backdrop="false">
    <div class="p-6 space-y-6">
      <div>
        <h2 class="text-lg font-semibold">
          Merge Into Profile
        </h2>
        <p class="text-sm text-gray-400">
          Copy mods from other profiles into <strong>{{ profileName }}</strong>
        </p>
      </div>

      <div v-if="otherProfiles.length" class="max-h-80 overflow-y-auto">
        <list-profiles :profiles="otherProfiles" @change="updateSelection" />
      </div>
      <p v-else class="text-sm italic text-gray-400">
        No other profiles available to merge from
      </p>

      <div class="flex gap-2 justify-end pt-2 w-full">
        <app-button
          type="button"
          class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded transition-colors"
          variant="secondary"
          @click="close"
        >
          Cancel
        </app-button>
        <app-button
          type="button"
          class="px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="!hasSelection"
          @click="handleMerge"
        >
          Merge
        </app-button>
      </div>
    </div>
  </app-modal>
</template>

<script lang="ts" setup>
import type { ProfileResponseDto } from '~/types/dto/profiles'

const props = defineProps<{
  gameId: string
  profileName: string
  profile: ProfileResponseDto
}>()

const emit = defineEmits<{
  merged: []
}>()

const { getProfiles, refreshGame } = useCurrentGame()

const modalRef = useTemplateRef('modal')
const listMods = ref<Set<string>>(new Set())

const otherProfiles = computed(() =>
  getProfiles.value.filter(p => p.name !== props.profileName),
)

const hasSelection = computed(() => listMods.value.size > 0)

async function handleMerge() {
  if (!listMods.value.size)
    return

  const existingMods = props.profile.mods
  const existingNames = new Set(existingMods.map(m => m.name))

  let nextOrder = existingMods.length
    ? Math.max(...existingMods.map(m => m.order)) + 1
    : 1

  const mods = []

  for (const mod of existingMods) {
    mods.push({
      name: mod.name,
      enabled: mod.enabled,
      order: mod.order,
    })
  }

  for (const name of listMods.value) {
    if (!existingNames.has(name)) {
      mods.push({
        name,
        enabled: true,
        order: nextOrder++,
      })
    }
  }

  try {
    await useTauriInvoke('update_profile', {
      payload: {
        gameId: props.gameId,
        name: props.profileName,
        default: props.profile.default,
        manualMode: props.profile.manualMode,
        mods,
      },
    })

    emit('merged')
    await refreshGame()
    close()
  }
  catch (err) {
    console.error('Failed to merge profiles:', err)
  }
}

function open() {
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

function updateSelection(selected: Set<string>) {
  listMods.value = selected
}

defineExpose({ open, close })
</script>
