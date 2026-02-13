<template>
  <app-modal ref="modal" :close-on-backdrop="false">
    <div class="p-6 space-y-6">
      <div>
        <h2 class="text-lg font-semibold">
          Rename Profile
        </h2>
        <p class="text-sm text-gray-400">
          {{ currentName }}
        </p>
      </div>

      <form @submit.prevent="handleSubmit">
        <div class="space-y-2.5">
          <app-input
            v-model="form.name"
            label="Profile Name *"
            type="text"
            placeholder="Enter profile name"
            required
            class="w-full"
            @keydown.enter="handleSubmit"
          />
          <p v-if="error" class="text-sm text-red-500">
            {{ error }}
          </p>
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
            :disabled="!form.name.trim() || form.name === currentName"
          >
            Save
          </app-button>
        </div>
      </form>
    </div>
  </app-modal>
</template>

<script lang="ts" setup>
interface ProfileForm {
  name: string
}

const props = defineProps<{
  gameId: string
  currentName: string
}>()

const emit = defineEmits<{
  save: [newName: string]
}>()

const modalRef = useTemplateRef('modal')

const { getProfiles } = useCurrentGame()

const form = ref<ProfileForm>({
  name: '',
})

const error = ref<string>('')

function validateName(name: string): string {
  if (!name.trim()) {
    return 'Profile name cannot be empty'
  }

  if (name.toLowerCase() === props.currentName.toLowerCase()) {
    return ''
  }

  const normalizedName = name.toLowerCase()
  const existsWithDifferentCase = getProfiles.value.some(
    existing => existing.name.toLowerCase() === normalizedName && existing.name.toLowerCase() !== props.currentName.toLowerCase(),
  )

  if (existsWithDifferentCase) {
    return 'A profile with this name already exists'
  }

  return ''
}

function handleSubmit() {
  const validationError = validateName(form.value.name)
  if (validationError) {
    error.value = validationError
    return
  }

  emit('save', form.value.name)
  modalRef.value?.close()
}

function resetForm() {
  form.value = {
    name: props.currentName,
  }
  error.value = ''
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
