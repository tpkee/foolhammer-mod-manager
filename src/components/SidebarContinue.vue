<template>
  <sidebar-button
    label="Continue"
    :tooltip="currentSave ? `Continue: ${currentSave}` : 'Continue'"
    :disabled="!currentSave"
    @click="handleContinue"
  >
    <template #menu="{ close }">
      <div class="py-1 min-w-44 max-w-72 w-fit overflow-y-scroll max-h-64">
        <div class="px-3 py-1.5 space-y-1.5">
          <p class="text-xs text-gray-400 uppercase tracking-wide">
            Select save
          </p>
          <app-input v-model="searchQuery" label="Search" sr-only-label placeholder="Search saves..." class="mb-3 block w-full" />

          <p v-if="getSaves.length === 0" class="text-sm text-gray-500">
            No saves found
          </p>
          <template v-else>
            <div
              v-for="save in getSaves"
              :key="save.path"
            >
              <app-radio
                v-model="currentSave"
                :value="save.name"
                :label="save.name"
              />
              <span v-if="save.lastAccessed" class="block text-xs text-gray-500 pl-5.5">{{ d(save.lastAccessed) }}</span>
            </div>
          </template>
        </div>
      </div>
    </template>

    <nuxt-icon name="mi:next" class="size-10" />
  </sidebar-button>
</template>

<script lang="ts" setup>
import type { SaveResponseDto } from '~/types/dto'

const props = defineProps<{
  saves: SaveResponseDto[]
}>()

const { d } = useI18n()

const currentSave = ref<Nullable<string>>()
const searchQuery = ref('')

const getSaves = computed(() => {
  const q = searchQuery.value.trim().toLowerCase()
  if (!q)
    return props.saves
  return props.saves.filter(save => save.name!.toLowerCase().includes(q))
})

watch(() => props.saves, (arr) => {
  if (arr) {
    currentSave.value = props.saves.toSorted((a, b) => {
      const aTime = a.lastAccessed ? new Date(a.lastAccessed).getTime() : 0
      const bTime = b.lastAccessed ? new Date(b.lastAccessed).getTime() : 0
      return bTime - aTime
    })[0]?.name
  }
  else {
    currentSave.value = null
  }
}, { immediate: true })

function handleContinue() {
  console.log('Continue with save:', currentSave.value)
}
</script>
