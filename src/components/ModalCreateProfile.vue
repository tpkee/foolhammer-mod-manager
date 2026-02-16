<template>
  <app-modal ref="modal" :close-on-backdrop="false">
    <div class="p-6 space-y-6">
      <div>
        <h2 class="text-lg font-semibold">
          Create New Profile
        </h2>
        <p class="text-sm text-gray-400">
          Enter a name for the new profile
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
          />
          <p v-if="getErrors" class="text-sm text-red-500">
            {{ getErrors }}
          </p>

          <div class="pt-2">
            <app-checkbox
              v-model="form.default"
              label="Set as default profile"
            />
          </div>
        </div>

        <div v-if="gameStore.getProfiles.length" class="pt-4">
          <p class="text-sm  mb-2">
            Copy mods from existing profiles
          </p>
          <div class="max-h-60 overflow-y-auto">
            <list-profiles :profiles="gameStore.getProfiles" @change="updateSelection" />
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
            :disabled="!!getErrors"
          >
            Create
          </app-button>
        </div>
      </form>
    </div>
  </app-modal>
</template>

<script lang="ts" setup>
interface ProfileForm {
  name: string
  default: boolean
}

const props = defineProps<{
  gameId: string
}>()

const emit = defineEmits<{
  created: []
}>()

const gameStore = useGameStore()

const modalRef = useTemplateRef('modal')

const listMods = ref<Set<string>>(new Set())

const form = ref<ProfileForm>({
  name: '',
  default: false,
})

// Computed
const getErrors = computed(() => {
  const normalizedName = form.value.name.toLowerCase()
  const existsWithDifferentCase = gameStore.getProfiles.some(
    existing => existing?.name && existing.name.toLowerCase() === normalizedName,
  )

  if (existsWithDifferentCase) {
    return 'A profile with this name already exists'
  }

  return ''
})

async function handleSubmit() {
  if (getErrors.value) {
    return
  }

  try {
    await useTauriInvoke('create_profile', {
      payload: {
        gameId: props.gameId,
        name: form.value.name,
        default: form.value.default,
        manualMode: false,
        mods: (Array.from(listMods.value) ?? []).map((name: string, index: number) => ({
          name,
          enabled: false,
          order: index + 1,
        })),
      },
    })

    emit('created')
    clearNuxtData(gameStore.getDataKey)
    modalRef.value?.close()
  }
  catch (err) {
    console.error('Failed to create profile:', err)
  }
}

function resetForm() {
  form.value = {
    name: '',
    default: false,
  }
}

function open() {
  resetForm()
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

function updateSelection(selected: Set<string>) {
  listMods.value = selected
}

defineExpose({ open, close })
</script>
