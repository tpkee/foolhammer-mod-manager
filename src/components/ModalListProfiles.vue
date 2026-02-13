<template>
  <app-modal ref="modalRef" :close-on-backdrop="false">
    <div class="p-6 space-y-6 min-w-96">
      <div>
        <h2 class="text-lg font-semibold">
          Profiles
        </h2>
        <p class="text-sm text-gray-400">
          Select a profile or manage them
        </p>
      </div>

      <div class="space-y-2 max-h-96 overflow-y-auto">
        <button
          v-for="profile in profiles"
          :key="profile.name"
          class="flex items-center justify-between p-3 bg-gray-800 border border-gray-700 rounded hover:border-purple-600 transition-colors group w-full cursor-pointer"
          :class="{
            'border-purple-600! hover:border-purple-400': profile.name === currentProfile,
          }"
          @click="switchProfile(profile.name); close()"
        >
          {{ profile.name }}

          <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
            <app-button
              type="button"
              variant="secondary"
              class="px-2 py-1 text-sm"
              @click.stop="editProfile(profile.name)"
            >
              Edit
            </app-button>
            <app-button
              v-if="!profile.default"
              type="button"
              variant="secondary"
              class="px-2 py-1 text-sm"
              @click.stop="setAsDefault(profile.name)"
            >
              Set Default
            </app-button>
            <app-button
              v-if="profile.name !== 'default'"
              type="button"
              variant="tertiary"
              class="px-2 py-1 text-sm"
              @click.stop="deleteProfileItem(profile.name)"
            >
              Delete
            </app-button>
          </div>
        </button>
      </div>

      <div class="flex gap-2 justify-end pt-4 w-full">
        <app-button
          type="button"
          class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded transition-colors"
          variant="secondary"
          @click="close"
        >
          Close
        </app-button>
      </div>
    </div>
  </app-modal>

  <modal-profile
    ref="modalEdit"
    :game-id="gameId"
    :current-name="selectedProfileForEdit"
    :existing-profile-names="otherProfileNames"
    @save="handleProfileRenamed"
  />
</template>

<script lang="ts" setup>
import type { ProfileResponseDto } from '~/types/dto'

const props = defineProps<{
  gameId: string
  currentProfile: string
  profiles: ProfileResponseDto[]
}>()

const emit = defineEmits<{
  profileSwitch: [profileName: string]
  profileDeleted: []
  profileRenamed: []
  profileSetDefault: []
}>()

const modalRef = ref()
const editModalRef = useTemplateRef('modalEdit')
const selectedProfileForEdit = ref<string>('')

const otherProfileNames = computed(() =>
  props.profiles.map(p => p.name).filter(name => name !== selectedProfileForEdit.value),
)

async function switchProfile(profileName: string) {
  if (profileName === props.currentProfile) {
    return
  }

  try {
    // Switch to profile by updating the game's current profile setting
    await useTauriInvoke('update_profile', {
      gameId: props.gameId,
      name: profileName,
      default: false,
      manualMode: false,
      mods: [],
    })
    emit('profileSwitch', profileName)
  }
  catch (error) {
    console.error('Failed to switch profile:', error)
  }
}

function editProfile(profileName: string) {
  selectedProfileForEdit.value = profileName
  editModalRef.value?.open()
}

async function handleProfileRenamed(newName: string) {
  const oldName = selectedProfileForEdit.value

  try {
    await useTauriInvoke('update_profile', {
      gameId: props.gameId,
      oldName,
      name: newName,
      default: false,
      manualMode: false,
      mods: [],
    })
    emit('profileRenamed')
  }
  catch (error) {
    console.error('Failed to rename profile:', error)
  }
}

async function setAsDefault(profileName: string) {
  try {
    await useTauriInvoke('update_profile', {
      gameId: props.gameId,
      name: profileName,
      default: true,
      manualMode: false,
      mods: [],
    })
    emit('profileSetDefault')
  }
  catch (error) {
    console.error('Failed to set profile as default:', error)
  }
}

async function deleteProfileItem(profileName: string) {
  if (profileName === 'default') {
    return
  }

  const confirmed = confirm(`Are you sure you want to delete the profile "${profileName}"?`)
  if (!confirmed) {
    return
  }

  try {
    await useTauriInvoke('delete_profile', {
      game_id: props.gameId,
      profile_name: profileName,
    })
    emit('profileDeleted')
  }
  catch (error) {
    console.error('Failed to delete profile:', error)
  }
}

function open() {
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

defineExpose({ open, close })
</script>
