<template>
  <app-modal ref="modalRef" :close-on-backdrop="false">
    <form class="p-6 space-y-4" @submit.prevent="onSave" @reset.prevent="close">
      <div>
        <h2 class="text-lg font-semibold">
          Add group to profiles
        </h2>
        <p class="text-sm text-gray-400">
          Select which profiles to add <strong class="text-gray-200">{{ group.name }}</strong>'s mods to
        </p>
      </div>

      <div class="border border-gray-700 rounded px-2.5 divide-y divide-gray-800">
        <div
          v-for="profile in profiles"
          :key="profile.id"
          class="flex items-center justify-between gap-3 py-2 px-1"
        >
          <app-checkbox
            v-model="selectedProfileIds"
            :label="profile.name ?? ''"
            :value="profile.id"
          />
          <span class="text-xs text-gray-400 shrink-0">
            <template v-if="getNewModCount(profile) === 0">
              All mods already added
            </template>
            <template v-else>
              {{ getNewModCount(profile) }} mod{{ getNewModCount(profile) === 1 ? '' : 's' }} to add
            </template>
          </span>
        </div>

        <p v-if="profiles.length === 0" class="text-sm text-gray-400 italic p-3">
          No profiles available
        </p>
      </div>

      <div class="flex gap-2 justify-end">
        <app-button type="reset" variant="secondary">
          Cancel
        </app-button>
        <app-button
          type="submit"
          :disabled="selectedProfileIds.length === 0"
          :loading="isLoading"
        >
          Add to profiles
        </app-button>
      </div>
    </form>
  </app-modal>
</template>

<script lang="ts" setup>
import type { GroupResponseDto, ProfileResponseDto } from '~/types/dto'

const props = defineProps<{
  group: GroupResponseDto
  gameId: string
}>()

const emit = defineEmits<{
  done: []
}>()

const gameStore = useGameStore()
const modalRef = useTemplateRef('modalRef')

const selectedProfileIds = ref<(string | undefined)[]>([])
const isLoading = ref(false)

const profiles = computed<ProfileResponseDto[]>(() => gameStore.getProfiles)

function getNewModCount(profile: ProfileResponseDto): number {
  const existing = new Set((profile.mods ?? []).map(m => m?.name).filter(Boolean))
  return (props.group.mods ?? []).filter(m => m && !existing.has(m)).length
}

function open() {
  selectedProfileIds.value = []
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

async function onSave() {
  if (isLoading.value)
    return

  isLoading.value = true
  try {
    await Promise.all(
      selectedProfileIds.value.map(async (profileId) => {
        if (!profileId)
          return

        const profile = profiles.value.find(p => p.id === profileId)
        if (!profile)
          return

        const existing = new Set((profile.mods ?? []).map(m => m?.name).filter(Boolean))
        const newMods = (props.group.mods ?? [])
          .filter((m): m is string => !!m && !existing.has(m))
          .map(name => ({ name, enabled: true, order: null }))

        if (newMods.length === 0)
          return

        await useTauriInvoke('add_profile_mods', {
          gameId: props.gameId,
          profileId,
          mods: newMods,
        })
      }),
    )

    await gameStore.fetchGame()
    emit('done')
    close()
  }
  catch (err) {
    console.error('Failed to add group to profiles:', err)
  }
  finally {
    isLoading.value = false
  }
}

defineExpose({ open, close })
</script>
