<template>
  <table-row :columns="columns">
    <template #name>
      <span class="font-medium truncate">{{ group.name }}</span>
    </template>

    <template #mods>
      <span class="text-sm text-gray-400">{{ group.mods?.length ?? 0 }} mod{{ (group.mods?.length ?? 0) !== 1 ? 's' : '' }}</span>
    </template>

    <template #actions>
      <app-options :options="getOptions" />
    </template>
  </table-row>

  <modal-rename-group
    ref="renameModalRef"
    :game-id="gameId"
    :group-id="group.id!"
    :current-name="group.name!"
    @save="emit('refresh')"
  />

  <modal-mod
    ref="modsModalRef"
    :mods="(gameStore.getGameMods as any)"
    title="Manage Group Mods"
    description="Select which mods belong to this group"
    :loading="isSavingMods"
    @save="onSaveGroupMods"
  />
</template>

<script lang="ts" setup>
import type { AppTableColumn } from '~/types/common/AppTable'
import type { GroupResponseDto } from '~/types/dto'

const props = defineProps<{
  columns: AppTableColumn[]
  group: GroupResponseDto
  gameId: string
}>()

const emit = defineEmits<{ refresh: [] }>()

const gameStore = useGameStore()

const renameModalRef = useTemplateRef('renameModalRef')
const modsModalRef = useTemplateRef('modsModalRef')
const isSavingMods = ref(false)

const getOptions = computed(() => [
  { icon: 'mi:edit', label: 'Rename', callback: openRenameModal },
  { icon: 'mi:layers', label: 'Manage Mods', callback: openModsModal },
  { icon: 'mi:delete', label: 'Delete', callback: deleteGroup },
])

function openRenameModal() {
  renameModalRef.value?.open()
}

function openModsModal() {
  modsModalRef.value?.open((props.group.mods ?? []).filter((m): m is string => m != null))
}

async function onSaveGroupMods(mods: string[]) {
  isSavingMods.value = true
  try {
    await useTauriInvoke('set_group_mods', {
      gameId: props.gameId,
      groupId: props.group.id!,
      mods,
    })
    await gameStore.fetchGame()
    modsModalRef.value?.close()
    emit('refresh')
  }
  catch (err) {
    console.error(err)
  }
  finally {
    isSavingMods.value = false
  }
}

async function deleteGroup() {
  try {
    await useTauriInvoke('delete_group', {
      gameId: props.gameId,
      groupId: props.group.id!,
    })
    await gameStore.fetchGame()
    emit('refresh')
  }
  catch (err) {
    console.error(err)
  }
}
</script>
