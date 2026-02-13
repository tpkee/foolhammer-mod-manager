<template>
  <sidebar-button
    label="Profile"
    :tooltip="getTooltip"
    :is-active="false"
    @click="openProfilesList"
  >
    <nuxt-icon name="mi:user" class="size-10" />
  </sidebar-button>

  <modal-list-profiles
    ref="modalListProfilesRef"
    :game-id="gameId"
    :current-profile="profileName"
    :profiles="allProfileNames"
    @profile-switch="handleProfileSwitch"
    @profile-deleted="emit('onProfileDeleted')"
  />
</template>

<script lang="ts" setup>
const props = defineProps<{
  gameId: string
  profileName: string
  isDefault: boolean
  allProfileNames: string[]
}>()

const emit = defineEmits<{
  onProfileUpdated: []
  onProfileDeleted: []
}>()

const modalListProfilesRef = useTemplateRef('modalListProfilesRef')

const getTooltip = computed(() => {
  return `Current profile: ${props.profileName}`
})

function openProfilesList() {
  modalListProfilesRef.value?.open()
}

async function handleProfileSwitch(newProfileName: string) {
  try {
    await useTauriInvoke('update_profile', {
      gameId: props.gameId,
      name: newProfileName,
      default: false,
      manualMode: false,
      mods: [],
    })
    emit('onProfileUpdated')
  }
  catch (error) {
    console.error('Failed to switch profile:', error)
  }
}
</script>
