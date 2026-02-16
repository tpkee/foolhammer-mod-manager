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
  currentProfile: Nullable<ProfileResponseDto>
}>()

const router = useRouter()

const getTooltip = computed(() => {
  return props.currentProfile ? `Current profile: ${props.currentProfile.name}` : 'No profile'
})

function navigateToProfiles() {
  router.push('/profiles')
}
</script>
