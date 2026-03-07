<!-- eslint-disable vue-a11y/no-static-element-interactions -->
<template>
  <table-row :columns="columns">
    <template #select>
      <div class="flex justify-center">
        <app-radio
          :model-value="gameStore.selectedProfile"
          :value="profile.id"
          label="Select profile"
          sr-only-label
          @update:model-value="switchProfile"
        />
      </div>
    </template>

    <template #name>
      <div class="flex items-center gap-2 min-w-0">
        <span class="font-medium truncate">{{ profile.name }}</span>
        <span v-if="isDefault" class="text-xs px-2 py-0.5 bg-purple-600 rounded shrink-0">Default</span>
      </div>
    </template>

    <template #activeMods>
      <span class="text-sm text-gray-400">{{ activeMods }}/{{ totalMods }}</span>
    </template>

    <template #actions>
      <app-options :options="getOptions" />
    </template>
  </table-row>

  <modal-rename-profile
    ref="modal"
    :game-id="gameId"
    :profile-id="profile.id!"
    :current-name="profile.name!"
  />

  <modal-merge-profile
    ref="mergeModal"
    :game-id="gameId"
    :profile="profile"
  />

  <modal-profile-groups
    ref="groupsModal"
    :game-id="gameId"
    :profile="profile"
    @save="emit('refresh')"
  />
</template>

<script lang="ts" setup>
import type { AppTableColumn } from '~/types/common/AppTable'
import type { ProfileResponseDto } from '~/types/dto/profiles'

interface Props {
  columns: AppTableColumn[]
  profile: ProfileResponseDto
  isActive: boolean
  gameId: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  refresh: []
}>()

const gameStore = useGameStore()

const modalRef = useTemplateRef('modal')
const mergeModalRef = useTemplateRef('mergeModal')
const groupsModalRef = useTemplateRef('groupsModal')

// Computed
const isDefault = computed(() => gameStore.currentGame?.defaultProfile && gameStore.currentGame?.defaultProfile === props.profile.id)
const activeMods = computed(() => props.profile.mods?.filter(m => m?.enabled).length ?? 0)
const totalMods = computed(() => props.profile.mods?.length ?? 0)
const getProfilesMinusCurrent = computed(() => gameStore.getProfiles.filter(p => p.id !== props.profile.id))
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
      hide: gameStore.getProfiles.length <= 1 || getProfilesMinusCurrent.value.every(p => !p.mods || p.mods.length === 0),
    },
    {
      icon: 'mi:group',
      label: 'Manage Groups',
      callback: openGroupsModal,
      hide: gameStore.getGroups.length === 0,
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

    emit('refresh')
  }
  catch (err) {
    console.error('Failed to set default profile:', err)
  }
}

function openEditModal() {
  modalRef.value?.open()
}

function openMergeModal() {
  mergeModalRef.value?.open()
}

function openGroupsModal() {
  groupsModalRef.value?.open()
}

async function switchProfile() {
  gameStore.setProfile(props.profile.id)
  emit('refresh')
}

async function deleteProfileItem() {
  try {
    await useTauriInvoke('delete_profile', {
      gameId: gameStore.selectedGame,
      profileId: props.profile.id,
    })
    gameStore.setProfile(null)
    emit('refresh')
  }
  catch (error) {
    console.error('Failed to delete profile:', error)
  }
}
</script>
