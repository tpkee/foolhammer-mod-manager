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
    :current-profile="currentProfile?.name ?? ''"
    :profiles="profiles"
    @profile-switch="handleProfileSwitch"
    @profile-deleted="emit('refresh')"
    @profile-renamed="emit('refresh')"
    @profile-set-default="emit('refresh')"
  />
</template>

<script lang="ts" setup>
import type { ProfileResponseDto } from '~/types/dto'

const props = defineProps<{
  gameId: string
  profiles: ProfileResponseDto[]
}>()

const emit = defineEmits<{
  refresh: []
}>()

const modalListProfilesRef = useTemplateRef('modalListProfilesRef')

const currentProfile = computed(() => {
  if (!props.profiles.length)
    return null

  const defaultProfile = props.profiles.find(p => p.default)
  return defaultProfile ?? props.profiles[0]
})

const getTooltip = computed(() => {
  return currentProfile.value ? `Current profile: ${currentProfile.value.name}` : 'No profile'
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
    emit('refresh')
  }
  catch (error) {
    console.error('Failed to switch profile:', error)
  }
}
</script>
