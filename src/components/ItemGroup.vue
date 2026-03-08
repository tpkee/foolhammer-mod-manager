<template>
  <table-row :columns="columns">
    <template #name>
      <span class="font-medium truncate">{{ group.name }}</span>
    </template>

    <template #mods>
      <span class="text-sm text-gray-400">{{ group.mods?.length ?? 0 }}</span>
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
  />

  <modal-mod
    ref="modsModalRef"
    :mods="(gameStore.getGameMods as any)"
    title="Manage Group Mods"
    description="Select which mods belong to this group"
    :loading="isSavingMods"
    @save="onSaveGroupMods"
  />

  <modal-add-group-to-profiles
    ref="addToProfilesModalRef"
    :group="group"
    :game-id="gameId"
  />
</template>

<script lang="ts" setup>
import type { AppTableColumn } from '~/types/common/AppTable'
import type { GroupResponseDto } from '~/types/dto'

import IconMiAdd from '~icons/mi/add'
import IconMiDelete from '~icons/mi/delete'
import IconMiEdit from '~icons/mi/edit'
import IconMiLayers from '~icons/mi/layers'

const props = defineProps<{
  columns: AppTableColumn[]
  group: GroupResponseDto
  gameId: string
}>()

const gameStore = useGameStore()

const renameModalRef = useTemplateRef('renameModalRef')
const modsModalRef = useTemplateRef('modsModalRef')
const addToProfilesModalRef = useTemplateRef('addToProfilesModalRef')
const isSavingMods = ref(false)

const getOptions = computed(() => [
  { icon: IconMiEdit, label: 'Rename', callback: openRenameModal },
  { icon: IconMiLayers, label: 'Manage Mods', callback: openModsModal },
  { icon: IconMiAdd, label: 'Add to profile', callback: () => addToProfilesModalRef.value?.open() },
  { icon: IconMiDelete, label: 'Delete', callback: deleteGroup },
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
  }
  catch (err) {
    console.error(err)
  }
}
</script>
