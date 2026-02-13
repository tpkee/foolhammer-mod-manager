<template>
  <div class="space-y-2">
    <app-accordion
      v-for="profile in profiles"
      :key="profile.name"
      class="relative"
    >
      <template #title>
        <app-checkbox
          :model-value="isSelected(profile.name)"
          :label="`Select ${profile.name}`"
          sr-only-label
          @update:model-value="toggleSelection(profile.name)"
        />
        <span>{{ profile.name }} ({{ profile.mods.length }})</span>
      </template>
      <ul v-if="profile.mods.length" class="grid grid-cols-3 gap-y-1">
        <li
          v-for="mod in profile.mods"
          :key="mod.name"
        >
          <span class="truncate" :title="mod.name">{{ mod.name }}</span>
        </li>
      </ul>
      <p v-else class="text-sm  italic">
        No mods in this profile
      </p>
    </app-accordion>

    <app-accordion
      v-if="selectedProfiles.length"
    >
      <template #title>
        <span>Combined Mods ({{ uniqueMods.length }})</span>
      </template>
      <ul v-if="uniqueMods.length" class="gap-y-1 grid grid-cols-3">
        <li
          v-for="modName in uniqueMods"
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

const selectedProfiles = ref<string[]>([])

function isSelected(profileName: string): boolean {
  return selectedProfiles.value.includes(profileName)
}

function toggleSelection(profileName: string) {
  const index = selectedProfiles.value.indexOf(profileName)
  if (index === -1) {
    selectedProfiles.value.push(profileName)
  }
  else {
    selectedProfiles.value.splice(index, 1)
  }
}

const uniqueMods = computed(() => {
  const allMods = new Set<string>()

  for (const profileName of selectedProfiles.value) {
    const profile = props.profiles.find(p => p.name === profileName)
    if (profile) {
      for (const mod of profile.mods) {
        allMods.add(mod.name)
      }
    }
  }

  return [...allMods].sort((a, b) => a.localeCompare(b))
})

defineExpose({ selectedProfiles, uniqueMods })
</script>
