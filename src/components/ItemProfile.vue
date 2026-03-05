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

const refreshGame = inject('refreshGame') as () => void

// Computed
const isDefault = computed(() => gameStore.currentGame?.defaultProfile && gameStore.currentGame?.defaultProfile === props.profile.id)
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
      hide: gameStore.getProfiles.length <= 1,
    },
    {
      icon: 'mi:delete',
      label: 'Delete',
      callback: deleteProfileItem,
      // hide: isDefault.value,
    },
  ]

  if (!isDefault.value) {
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
      profileId: props.profile.id,
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
      profileId: props.profile.id,
      newName,
    })
    refreshGame()
  }
  catch (err) {
    console.error('Failed to rename profile:', err)
  }
}

function openEditModal() {
  modalRef.value?.open()
}

function openMergeModal() {
  mergeModalRef.value?.open()
}

async function switchProfile() {
  if (props.profile.id === gameStore.selectedProfile)
    return

  gameStore.setProfile(props.profile.id)
  refreshGame()
}

async function deleteProfileItem() {
  try {
    await useTauriInvoke('delete_profile', {
      gameId: gameStore.selectedGame,
      profileId: props.profile.id,
    })
    if (gameStore.selectedProfile === props.profile.id) {
      const defaultProfile = gameStore.getProfiles.find(p => p.default && p.id !== props.profile.id)
      gameStore.setProfile(defaultProfile?.id ?? null)
    }
    refreshGame()
  }
  catch (error) {
    console.error('Failed to delete profile:', error)
  }
}
</script>
