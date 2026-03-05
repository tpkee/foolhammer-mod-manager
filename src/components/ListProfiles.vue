<template>
  <div class="space-y-2">
    <app-accordion
      v-for="profile in getProfiles"
      :key="profile.id"
      :model-value="selectedProfiles.has(profile.id!)"
      :title="`${profile.name} (${profile.mods?.length})`"
      class="relative"
      @update:model-value="toggleSelection(profile.id!)"
    >
      <ul v-if="profile.mods?.length" class="grid grid-cols-3 gap-y-1">
        <li
          v-for="mod in profile.mods.filter(m => m && m.name)"
          :key="mod!.name"
        >
          <span class="truncate" :title="mod!.name">{{ mod!.name }}</span>
        </li>
      </ul>
      <p v-else class="text-sm  italic">
        No mods in this profile
      </p>
    </app-accordion>

    <app-accordion
      v-if="selectedProfiles.size"
      :title="`Combined Mods (${getUniqueMods.size})`"
    >
      <ul v-if="getUniqueMods.size" class="gap-y-1 grid grid-cols-3">
        <li
          v-for="modName in getUniqueMods"
          :key="modName"
        >
          <span class="truncate">{{ modName }}</span>
        </li>
      </ul>
    </app-accordion>
  </div>
</template>

<script lang="ts" setup>
import type { ProfileResponseDto } from '~/types/dto/profiles'

const props = defineProps<{
  profiles: ProfileResponseDto[]
}>()

const emit = defineEmits<{
  change: [Set<string>]
}>()

const selectedProfiles = ref<Set<string>>(new Set())

function toggleSelection(profileId: string) {
  if (selectedProfiles.value.has(profileId)) {
    selectedProfiles.value.delete(profileId)
  }
  else {
    selectedProfiles.value.add(profileId)
  }
}

const getProfiles = computed(() => {
  return props.profiles.filter(p => p && p.id)
})

const getUniqueMods = computed(() => {
  const allMods = new Set<string>()

  for (const profileId of selectedProfiles.value) {
    const profile = props.profiles.find(p => p.id === profileId)
    const mods = profile?.mods ?? []
    if (profile) {
      for (const mod of mods) {
        if (mod?.name) {
          allMods.add(mod.name)
        }
      }
    }
  }

  return allMods
})

watch(getUniqueMods, (newVal) => {
  emit('change', newVal)
}, { deep: true })
</script>
