<template>
  <div class="space-y-4">
    <!-- Breadcrumb -->
    <div class="flex items-center gap-2">
      <app-button variant="secondary" class="px-3 py-1.5" @click="goBack">
        <div class="flex items-center gap-2">
          <nuxt-icon name="mi:arrow-left" class="size-5" />
          <span>Back</span>
        </div>
      </app-button>
    </div>

    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold">
          Profiles
        </h1>
        <p class="text-sm text-gray-400">
          Manage your game profiles
        </p>
      </div>
      <app-button class="px-4 py-2" @click="openCreateModal">
        Create New Profile
      </app-button>
    </div>

    <!-- Profiles List -->
    <div class="space-y-2 max-h-[calc(100vh-12rem)] overflow-y-auto">
      <button
        v-for="profile in getProfiles"
        :key="profile.name"
        class="flex items-center justify-between p-4 bg-gray-800 border border-gray-700 rounded hover:border-purple-600 transition-colors group w-full cursor-pointer"
        :class="{
          'border-purple-600 bg-gray-750': profile.name === currentProfile,
        }"
        @click="switchProfile(profile.name)"
      >
        <div class="flex items-center gap-3">
          <span class="text-lg font-medium">{{ profile.name }}</span>
          <span v-if="profile.default" class="text-xs px-2 py-0.5 bg-purple-600 rounded">Default</span>
          <span v-if="profile.name === currentProfile" class="text-xs text-purple-400">(current)</span>
        </div>

        <div class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
          <app-button
            type="button"
            variant="secondary"
            class="px-3 py-1.5 text-sm"
            @click.stop="editProfile(profile.name)"
          >
            Edit
          </app-button>
          <app-button
            v-if="!profile.default"
            type="button"
            variant="secondary"
            class="px-3 py-1.5 text-sm"
            @click.stop="setAsDefault(profile.name)"
          >
            Set Default
          </app-button>
          <app-button
            v-if="profile.name !== 'default'"
            type="button"
            variant="tertiary"
            class="px-3 py-1.5 text-sm"
            @click.stop="deleteProfileItem(profile.name)"
          >
            Delete
          </app-button>
        </div>
      </button>
    </div>
  </div>

  <modal-profile
    ref="editModalRef"
    :game-id="currentGameId"
    :current-name="selectedProfileForEdit"
    :existing-profile-names="otherProfileNames"
    @save="handleProfileRenamed"
  />

  <modal-create-profile
    ref="createModalRef"
    :game-id="currentGameId"
    :existing-profile-names="getProfiles.map(p => p.name)"
    @created="handleProfileCreated"
  />
</template>

<script lang="ts" setup>
const router = useRouter()
const preferencesStore = usePreferencesStore()
const { currentGameData, getProfiles, refreshGame } = useCurrentGame()

const editModalRef = useTemplateRef('editModalRef')
const createModalRef = useTemplateRef('createModalRef')
const selectedProfileForEdit = ref<string>('')

const currentGameId = computed(() => preferencesStore.currentGame ?? '')
const currentProfile = computed(() => {
  if (!getProfiles.value.length)
    return ''

  const defaultProfile = getProfiles.value.find(p => p.default)
  return defaultProfile?.name ?? getProfiles.value[0]?.name ?? ''
})

const otherProfileNames = computed(() =>
  getProfiles.value.map(p => p.name).filter(name => name !== selectedProfileForEdit.value),
)

function openCreateModal() {
  createModalRef.value?.open()
}

async function handleProfileCreated() {
  await refreshGame()
}

function goBack() {
  router.back()
}

async function switchProfile(profileName: string) {
  if (profileName === currentProfile.value)
    return

  try {
    await useTauriInvoke('update_profile', {
      gameId: currentGameId.value,
      name: profileName,
      default: false,
      manualMode: false,
      mods: [],
    })
    await refreshGame()
    router.push('/')
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
      gameId: currentGameId.value,
      oldName,
      name: newName,
      default: false,
      manualMode: false,
      mods: [],
    })
    await refreshGame()
  }
  catch (error) {
    console.error('Failed to rename profile:', error)
  }
}

async function setAsDefault(profileName: string) {
  try {
    await useTauriInvoke('update_profile', {
      gameId: currentGameId.value,
      name: profileName,
      default: true,
      manualMode: false,
      mods: [],
    })
    await refreshGame()
  }
  catch (error) {
    console.error('Failed to set profile as default:', error)
  }
}

async function deleteProfileItem(profileName: string) {
  if (profileName === 'default')
    return

  const confirmed = confirm(`Are you sure you want to delete the profile "${profileName}"?`)
  if (!confirmed)
    return

  try {
    await useTauriInvoke('delete_profile', {
      game_id: currentGameId.value,
      profile_name: profileName,
    })
    await refreshGame()
  }
  catch (error) {
    console.error('Failed to delete profile:', error)
  }
}
</script>
