<template>
  <sidebar-button
    label="Profile"
    :tooltip="getTooltip"
    :is-active="false"
    @click="navigateToProfiles"
  >
    <nuxt-icon name="mi:user" class="size-10" />
  </sidebar-button>
</template>

<script lang="ts" setup>
import type { ProfileResponseDto } from '~/types/dto'

const props = defineProps<{
  gameId: string
  profiles: ProfileResponseDto[]
}>()

const router = useRouter()

const currentProfile = computed(() => {
  if (!props.profiles.length)
    return null

  const defaultProfile = props.profiles.find(p => p.default)
  return defaultProfile ?? props.profiles[0]
})

const getTooltip = computed(() => {
  return currentProfile.value ? `Current profile: ${currentProfile.value.name}` : 'No profile'
})

function navigateToProfiles() {
  router.push('/profiles')
}
</script>
