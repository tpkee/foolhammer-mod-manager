<template>
  <app-modal ref="modalRef" :close-on-backdrop="false">
    <div class="p-6 space-y-6">
      <div>
        <h2 class="text-lg font-semibold">
          Create New Group
        </h2>
        <p class="text-sm text-gray-400">
          Enter a name for the new group
        </p>
      </div>

      <form @submit.prevent="handleSubmit">
        <div class="space-y-2.5">
          <app-input
            v-model="name"
            label="Group Name *"
            type="text"
            placeholder="Enter group name"
            required
            class="w-full"
          />
          <p v-if="error" class="text-sm text-red-500">
            {{ error }}
          </p>
        </div>

        <div class="flex gap-2 justify-end pt-4 w-full">
          <app-button
            type="button"
            variant="secondary"
            class="px-4 py-2"
            @click="close"
          >
            Cancel
          </app-button>
          <app-button
            type="submit"
            class="px-4 py-2"
            :disabled="!!error || !name.trim()"
            :loading="isLoading"
          >
            Create
          </app-button>
        </div>
      </form>
    </div>
  </app-modal>
</template>

<script lang="ts" setup>
const props = defineProps<{
  gameId: string
}>()

const emit = defineEmits<{
  created: []
}>()

const gameStore = useGameStore()
const modalRef = useTemplateRef('modalRef')

const name = ref('')
const isLoading = ref(false)

const error = computed(() => {
  const normalized = name.value.toLowerCase().trim()
  if (!normalized)
    return ''
  if (gameStore.getGroups.some(g => g.name?.toLowerCase() === normalized))
    return 'A group with this name already exists'
  return ''
})

async function handleSubmit() {
  if (error.value || !name.value.trim())
    return

  isLoading.value = true
  try {
    await useTauriInvoke('create_group', {
      payload: { gameId: props.gameId, name: name.value.trim(), mods: [] },
    })
    await gameStore.fetchGame()
    emit('created')
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
  name.value = ''
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

defineExpose({ open, close })
</script>
