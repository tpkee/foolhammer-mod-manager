<template>
  <app-modal ref="modalRef" :close-on-backdrop="false">
    <div class="p-6 space-y-6">
      <div>
        <h2 class="text-lg font-semibold">
          Rename Group
        </h2>
        <p class="text-sm text-gray-400">
          {{ currentName }}
        </p>
      </div>

      <form @submit.prevent="handleSubmit">
        <div class="space-y-2.5">
          <app-input
            v-model="newName"
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
            :disabled="!newName.trim() || newName === currentName || !!error"
            :loading="isLoading"
          >
            Save
          </app-button>
        </div>
      </form>
    </div>
  </app-modal>
</template>

<script lang="ts" setup>
const props = defineProps<{
  gameId: string
  groupId: string
  currentName: string
}>()

const emit = defineEmits<{
  save: []
}>()

const gameStore = useGameStore()
const modalRef = useTemplateRef('modalRef')

const newName = ref('')
const isLoading = ref(false)

const error = computed(() => {
  const normalized = newName.value.toLowerCase().trim()
  if (!normalized)
    return ''
  if (normalized === props.currentName.toLowerCase())
    return ''
  if (gameStore.getGroups.some(g => g.name?.toLowerCase() === normalized))
    return 'A group with this name already exists'
  return ''
})

async function handleSubmit() {
  if (error.value || !newName.value.trim() || newName.value === props.currentName)
    return

  isLoading.value = true
  try {
    await useTauriInvoke('rename_group', {
      gameId: props.gameId,
      groupId: props.groupId,
      newName: newName.value.trim(),
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
  newName.value = props.currentName
  modalRef.value?.open()
}

function close() {
  modalRef.value?.close()
}

defineExpose({ open, close })
</script>
