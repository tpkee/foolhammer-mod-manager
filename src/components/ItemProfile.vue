<template>
  <div>
    <button
      class="flex items-center justify-between p-4 bg-gray-800 border border-gray-700 rounded hover:border-purple-600 transition-colors group w-full cursor-pointer"
      :class="{
        'border-purple-600 bg-gray-750': isActive,
      }"
      @click="switchProfile"
    >
      <div class="flex items-center gap-3">
        <span class="text-lg font-medium">{{ profile.name }}</span>
        <span v-if="profile.default" class="text-xs px-2 py-0.5 bg-purple-600 rounded">Default</span>
      </div>

      <app-options :options="getOptions" />
    </button>

    <modal-rename-profile
      ref="modal"
      :game-id="gameId"
      :current-name="profile.name!"
      @save="handleRename"
    />

    <modal-merge-profile
      ref="mergeModal"
      :game-id="gameId"
      :profile="profile"
      @merged="$emit('merged')"
    />
  </div>
</template>

<script lang="ts" setup>
import type { ProfileResponseDto } from '~/types/dto/profiles'

interface Props {
  profile: ProfileResponseDto
  isActive: boolean
  gameId: string
}

const props = defineProps<Props>()

defineEmits<{
  merged: []
}>()

const gameStore = useGameStore()

const modalRef = useTemplateRef('modal')
const mergeModalRef = useTemplateRef('mergeModal')

const getOptions = computed(() => {
  const opts = [
    {
      icon: 'mi:edit',
      label: 'Rename',
      callback: openEditModal,
    },
    {
      icon: 'mi:layers',
      label: 'Merge from profiles',
      callback: openMergeModal,
    },
    {
      icon: 'mi:delete',
      label: 'Delete',
      callback: () => props.profile.name ? deleteProfileItem(props.profile.name) : {},
      hide: props.profile.name === 'default',
    },
  ]

  if (!props.profile.default) {
    opts.unshift({
      icon: 'mi:heart',
      label: 'Set as Default',
      callback: handleSetDefault,
    })
  }

  return opts
})

async function handleSetDefault() {
  try {
    await useTauriInvoke('set_default_profile', {
      gameId: props.gameId,
      profileName: props.profile.name,
    })

    refreshGame()
  }
  catch (err) {
    console.error('Failed to set default profile:', err)
  }
}

async function handleRename(newName: string) {
  try {
    await useTauriInvoke('rename_profile', {
      gameId: props.gameId,
      oldName: props.profile.name,
      newName,
    })
    refreshGame()
  }
  catch (err) {
    console.error('Failed to rename profile:', err)
  }
}

function refreshGame() { // invalidate the fetched state, this should trigger a refetch. hopefully.
  clearNuxtData(gameStore.getDataKey)
}

function openEditModal() {
  modalRef.value?.open()
}

function openMergeModal() {
  mergeModalRef.value?.open()
}

async function switchProfile() {
  if (props.profile.name === gameStore.selectedProfile)
    return

  gameStore.setProfile(props.profile.name)
  refreshGame()
}

async function deleteProfileItem(profileName: string) {
  try {
    await useTauriInvoke('delete_profile', {
      gameId: gameStore.currentGame,
      profileName,
    })
    if (gameStore.selectedProfile === profileName) {
      const defaultProfile = gameStore.getProfiles.find(p => p.default && p.name !== profileName)
      gameStore.setProfile(defaultProfile?.name ?? null)
    }
    refreshGame()
  }
  catch (error) {
    console.error('Failed to delete profile:', error)
  }
}
</script>
