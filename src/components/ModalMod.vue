<template>
  <app-modal ref="modalRef" :close-on-backdrop="false">
    <div class="p-6 space-y-4">
      <div>
        <h2 class="text-lg font-semibold">
          Add Mods
        </h2>
        <p class="text-sm text-gray-400">
          Select one or more mods to add to this profile
        </p>
      </div>

      <app-input v-model="search" class="w-full" label="Search" placeholder="Search mods..." />

      <div class="grid md:grid-cols-2 gap-3">
        <div class="border border-gray-700 rounded px-2.5">
          <p class="px-3 py-2 border-b border-gray-800 text-sm text-gray-300">
            Available mods ({{ filteredMods.length }})
          </p>

          <div v-if="filteredMods.length" class="max-h-80 overflow-y-auto">
            <div v-for="(mod, index) in filteredMods" :key="index" class="px-2.5 py-0.5 w-full">
              <app-checkbox v-model="selectedMods" :label="mod.name" :value="mod.name" />
            </div>
          </div>
          <p v-else class="text-sm text-gray-400 italic p-3">
            No mods available to add
          </p>
        </div>

        <div class="border border-gray-700 rounded">
          <p class="px-3 py-2 border-b border-gray-800 text-sm text-gray-300">
            Selected mods ({{ selectedMods.size }})
          </p>

          <div v-if="selectedMods.size" class="max-h-80 overflow-y-auto">
            <button
              v-for="(mod, index) in selectedMods" :key="index"
              class="w-full flex items-center justify-between gap-2.5 px-2.5 py-0.5 border-b border-gray-800 last:border-b-0 cursor-pointer hover:bg-gray-800/70 transition-colors"
              type="button" @click="selectedMods.delete(mod)"
            >
              <p :title="mod" class="truncate text-left">
                {{ mod }}
              </p>
              <nuxt-icon class="size-4 text-gray-400" name="mi:close" />
            </button>
          </div>
          <p v-else class="text-sm text-gray-400 italic p-3">
            No mods selected yet. Select mods from the left to add them to this profile.
          </p>
        </div>
      </div>

      <div class="flex gap-2 justify-end">
        <app-button class="px-4 py-2" type="button" variant="secondary" @click="close">
          Cancel
        </app-button>

        <app-button :disabled="!selectedMods.size" class="px-4 py-2" type="button" @click="addMods">
          Add selected
        </app-button>
      </div>
    </div>
  </app-modal>
</template>

<script lang="ts" setup>
import type { ModResponseDto } from '~/types/dto'

const props = defineProps<{
  mods: ModResponseDto[]
}>()

const emit = defineEmits<{
  add: [mods: ModResponseDto[]]
}>()

const modalRef = useTemplateRef('modalRef')

const search = ref('')
const selectedMods = ref<Set<string>>(new Set())

const filteredMods = computed(() => {
  const query = search.value.toLowerCase().trim()

  return props.mods.filter((mod) => {
    if (!mod.name)
      return false

    if (!query)
      return true

    return mod.name.toLowerCase().includes(query)
  })
})

function open() {
  search.value = ''
  selectedMods.value.clear()
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

function addMods() {
  if (!selectedMods.value.size)
    return

  // emit('add', selectedMods.value) TODO: handle this thing
  close()
}

defineExpose({ open, close })
</script>
