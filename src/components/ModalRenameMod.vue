<template>
  <app-modal ref="modal" :close-on-backdrop="false">
    <div class="p-6 space-y-6">
      <div>
        <h2 class="text-lg font-semibold">
          Rename Mod
        </h2>
        <p class="text-sm text-gray-400">
          {{ name }}
        </p>
      </div>

      <form @submit.prevent="handleSubmit">
        <app-input
          v-model="form.customName"
          label="Custom Name"
          type="text"
          placeholder="Leave empty to use the pack name"
          class="w-full"
          @keydown.enter="handleSubmit"
        />

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
            :loading="isLoading"
            :disabled="form.customName.trim() === (currentCustomName ?? '')"
          >
            Save
          </app-button>
        </div>
      </form>
    </div>
  </app-modal>
</template>

<script lang="ts" setup>
interface ModForm {
  customName: string
}

const props = defineProps<{
  gameId: string
  name: string
  currentCustomName?: Nullable<string>
}>()

const emit = defineEmits<{
  save: []
}>()

const modalRef = useTemplateRef('modal')

const gameStore = useGameStore()

const form = ref<ModForm>({
  customName: '',
})
const isLoading = ref(false)

async function handleSubmit() {
  isLoading.value = true

  try {
    const customName = form.value.customName.trim()
    await useTauriInvoke('rename_mod', {
      gameId: props.gameId,
      name: props.name,
      customName: customName || null,
    })

    await gameStore.fetchGame()

    emit('save')
    modalRef.value?.close()
  }
  catch (err) {
    console.error('Failed to rename mod:', err)
  }
  finally {
    isLoading.value = false
  }
}

function resetForm() {
  form.value = {
    customName: props.currentCustomName ?? '',
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
