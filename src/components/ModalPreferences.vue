<template>
  <app-modal ref="modalRef" :close-on-backdrop="false">
    <div class="p-6 space-y-6">
      <div>
        <h2 class="text-lg font-semibold">
          Preferences
        </h2>
        <p class="text-sm text-gray-400">
          Configure Steam installation and library paths
        </p>
      </div>

      <form @submit.prevent="handleSubmit" @reset.prevent="close">
        <div class="grid grid-cols-1 gap-y-2.5">
          <div class="flex gap-2 items-end">
            <app-input
              v-model="form.steamPath"
              label="Steam Installation Path"
              type="text"
              placeholder="e.g. C:\Program Files (x86)\Steam"
              class="w-full"
            />
            <app-button
              type="button"
              @click="pickPath('steamPath')"
            >
              Browse
            </app-button>
          </div>

          <div class="flex gap-2 items-end">
            <app-input
              v-model="form.steamLibraryPath"
              label="Steam Library Path"
              type="text"
              placeholder="e.g. E:\SteamLibrary"
              class="w-full"
            />
            <app-button
              type="button"
              @click="pickPath('steamLibraryPath')"
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
            class="px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded transition-colors"
            :loading="isLoading"
          >
            Save Preferences
          </app-button>
        </div>
      </form>
    </div>
  </app-modal>
</template>

<script lang="ts" setup>
import { open as openDialog } from '@tauri-apps/plugin-dialog'

interface PreferencesForm {
  steamPath: Nullable<string>
  steamLibraryPath: Nullable<string>
}

const emit = defineEmits<{
  save: []
}>()

const settingsStore = useSettingsStore()

const modalRef = ref()
const isLoading = ref(false)

const form = ref<PreferencesForm>({
  steamPath: null,
  steamLibraryPath: null,
})

async function pickPath(field: keyof PreferencesForm) {
  try {
    const selected = await openDialog({
      multiple: false,
      directory: true,
    })

    form.value[field] = selected
  }
  catch (error) {
    console.error('Failed to pick path:', error)
  }
}

async function handleSubmit() {
  isLoading.value = true
  try {
    await useTauriInvoke('update_settings', {
      payload: {
        steamPath: form.value.steamPath,
        steamLibraryPath: form.value.steamLibraryPath,
      },
    })
    emit('save')
    modalRef.value?.close()
  }
  catch (error) {
    console.error('Failed to save preferences:', error)
  }
  finally {
    isLoading.value = false
  }
}

async function resetForm() {
  try {
    const settings = await useTauriInvoke<{
      steamPath: Nullable<string>
      steamLibraryPath: Nullable<string>
    }>('get_user_settings')

    form.value = {
      steamPath: settings.steamPath ?? null,
      steamLibraryPath: settings.steamLibraryPath ?? null,
    }
  }
  catch (error) {
    console.error('Failed to load preferences:', error)
    form.value = {
      steamPath: settingsStore.settings?.steamPath ?? null,
      steamLibraryPath: settingsStore.settings?.steamLibraryPath ?? null,
    }
  }
}

async function open() {
  await resetForm()
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

defineExpose({ open, close })
</script>
