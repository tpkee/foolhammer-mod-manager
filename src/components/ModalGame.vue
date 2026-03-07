<template>
  <app-modal ref="modalRef" :close-on-backdrop="false">
    <div class="p-6 space-y-6">
      <div>
        <h2 class="text-lg font-semibold">
          {{ t(`games.${gameId}`) }} Settings
        </h2>
        <p class="text-sm text-gray-400">
          Configure paths for the game
        </p>
      </div>

      <form @submit.prevent="handleSubmit" @reset.prevent="close">
        <div class="grid grid-cols-2 gap-x-5 gap-y-2.5 items-center">
          <div class="flex gap-2 items-end">
            <app-input
              v-model="form.gamePath"
              label="Game Installation Path *"
              type="text"
              placeholder="Select game installation directory"
              required
              class="w-full"
            />
            <app-button
              type="button"
              @click="pickPath('gamePath')"
            >
              Browse
            </app-button>
          </div>

          <div class="flex gap-2 items-end">
            <app-input
              v-model="form.savesPath"
              label="Game Saves Path"
              type="text"
              placeholder="Select game saves directory (optional)"
              class="w-full"
            />
            <app-button
              type="button"
              @click="pickPath('savesPath')"
            >
              Browse
            </app-button>
          </div>

          <div class="flex gap-2 items-end">
            <app-input
              v-model="form.modsPath"
              label="Mods Path"
              type="text"
              placeholder="Select mods directory (optional)"
              class="w-full"
            />
            <app-button
              type="button"
              @click="pickPath('modsPath')"
            >
              Browse
            </app-button>
          </div>
        </div>

        <div class="flex gap-2 justify-end pt-4 w-full">
          <app-button
            type="button"
            class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded transition-colors"
            variant="secondary"
            @click="close"
          >
            Cancel
          </app-button>
          <app-button
            type="submit"
            class="px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            :disabled="!form.gamePath"
            :loading="isLoading"
          >
            Save Settings
          </app-button>
        </div>
      </form>
    </div>
  </app-modal>
</template>

<script lang="ts" setup>
import { open as openDialog } from '@tauri-apps/plugin-dialog'

interface GameSettings {
  gamePath: Nullable<string>
  savesPath: Nullable<string>
  modsPath: Nullable<string>
}

const { gameId } = defineProps<{
  gameId: string
}>()

const emit = defineEmits<{
  save: []
}>()

// TODO: big todo. I forgot to add a way to load existing settings for inactive games, so this will only work for the current game.

// stores
const gameStore = useGameStore()

const { t } = useI18n()

const modalRef = ref()
const isLoading = ref(false)

const form = ref<GameSettings>({
  gamePath: null,
  savesPath: null,
  modsPath: null,
})

async function pickPath(field: keyof GameSettings) {
  try {
    const selected = await openDialog({
      multiple: false,
      directory: true,
    })

    form.value[field] = selected

    if (field === 'gamePath') {
      form.value.modsPath = `${selected}/data`
    }
  }
  catch (error) {
    console.error('Failed to pick path:', error)
  }
}

async function handleSubmit() {
  isLoading.value = true
  try {
    await useTauriInvoke('update_game', {
      gameId,
      payload: {
        gamePath: form.value.gamePath,
        savesPath: form.value.savesPath,
        modsPath: form.value.modsPath,
      },
    })
    await gameStore.fetchGame()
    emit('save')

    modalRef.value?.close()
  }
  catch (error) {
    console.error('Failed to save game settings:', error)
  }
  finally {
    isLoading.value = false
  }
}

function resetForm() {
  form.value = {
    gamePath: gameStore.currentGame?.gamePath ?? null,
    savesPath: gameStore.currentGame?.savesPath ?? null,
    modsPath: gameStore.currentGame?.modsPath ?? null,
  }
}

function open() {
  resetForm()
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

defineExpose({ open, close })
</script>
