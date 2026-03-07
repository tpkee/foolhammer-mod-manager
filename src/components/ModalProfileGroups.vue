<template>
  <app-modal ref="modalRef" :close-on-backdrop="false">
    <form class="p-6 space-y-4" @submit.prevent="onSave" @reset.prevent="close">
      <div>
        <h2 class="text-lg font-semibold">
          Manage Groups
        </h2>
        <p class="text-sm text-gray-400">
          Select which groups this profile belongs to
        </p>
      </div>

      <div
        v-if="gameStore.getGroups.length"
        class="space-y-1 max-h-80 overflow-y-auto border border-gray-700 rounded p-2.5"
      >
        <div v-for="group in gameStore.getGroups" :key="group.id" class="py-0.5">
          <app-checkbox
            v-model="selectedGroupIds"
            :label="group.name"
            :value="group.id"
          />
          <p class="text-xs text-gray-500 ml-5">
            {{ group.mods?.length ?? 0 }} mod{{ (group.mods?.length ?? 0) !== 1 ? 's' : '' }}
          </p>
        </div>
      </div>
      <p v-else class="text-sm text-gray-400 italic">
        No groups available
      </p>

      <div class="flex gap-2 justify-end">
        <app-button class="px-4 py-2" type="reset" variant="secondary">
          Cancel
        </app-button>
        <app-button class="px-4 py-2" type="submit" :loading="isLoading">
          Save
        </app-button>
      </div>
    </form>
  </app-modal>
</template>

<script lang="ts" setup>
import type { ProfileResponseDto } from '~/types/dto'

const props = defineProps<{
  gameId: string
  profile: ProfileResponseDto
}>()

const emit = defineEmits<{
  save: []
}>()

const gameStore = useGameStore()
const modalRef = useTemplateRef('modalRef')

const selectedGroupIds = ref<string[]>([])
const isLoading = ref(false)

async function onSave() {
  isLoading.value = true
  try {
    await useTauriInvoke('set_groups_profile', {
      gameId: props.gameId,
      profileId: props.profile.id,
      groups: selectedGroupIds.value,
    })
    await gameStore.fetchGame()
    emit('save')
    close()
  }
  catch (err) {
    console.error(err)
  }
  finally {
    isLoading.value = false
  }
}

function open() {
  selectedGroupIds.value = (props.profile.groups ?? []).filter((g): g is string => g != null)
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

defineExpose({ open, close })
</script>
