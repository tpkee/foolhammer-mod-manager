<template>
  <div class="space-y-2.5">
    <div class="flex items-end gap-2.5 w-full">
      <app-input v-model="filters.search" placeholder="Search by pack or name..." class="w-full" label="Search" />
      <div class="flex items-end gap-2.5">
        <app-select v-model="filters.sortBy" :options="sortOptions" label="Sort by" />
        <app-select v-model="filters.sortOrder" :options="orderOptions" label="Order " />
        <app-dropdown>
          <template #trigger="{ toggle }">
            <div>
              <app-button variant="secondary" @click="toggle">
                <span class="sr-only">Menu</span>
                <IconMiMenu class="size-5 align-middle block" />
              </app-button>
            </div>
          </template>
          <template #default="{ close }">
            <div class="py-1.5 min-w-48">
              <button
                v-if="refModalMod"
                v-show="getMissingMods.length"
                class="flex w-full items-center gap-2.5 px-3 py-2 text-sm text-left rounded hover:bg-gray-700 transition-colors cursor-pointer"
                @click="refModalMod.open(); close()"
              >
                <IconMiAdd class="size-4 shrink-0" />
                Add mods to profile
              </button>
              <button
                class="flex w-full items-center gap-2.5 px-3 py-2 text-sm text-left rounded hover:bg-gray-700 transition-colors cursor-pointer"
                @click="toggleAllMods(); close()"
              >
                <IconMiCheck v-if="!allModsEnabled" class="size-4 shrink-0" />
                <IconMiClose v-else class="size-4 shrink-0" />
                {{ allModsEnabled ? 'Disable' : 'Enable' }} all mods
              </button>
              <button
                v-if="profile"
                class="flex w-full items-center gap-2.5 px-3 py-2 text-sm text-left rounded hover:bg-gray-700 transition-colors cursor-pointer"
                @click="refModalProfileGroups?.open(); close()"
              >
                <IconMiFolder class="size-4 shrink-0" />
                Manage profile groups
              </button>
              <div class="my-1 mx-2 h-px bg-gray-700" />
              <button
                class="flex w-full items-center gap-2.5 px-3 py-2 text-sm text-left rounded transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                :class="profile?.manualMode ? 'text-red-400 hover:bg-red-950' : 'hover:bg-gray-700'"
                :disabled="isTogglingManualMode"
                @click="toggleManualMode(); close()"
              >
                <app-spinner v-if="isTogglingManualMode" class="size-4 shrink-0 animate-spin" />
                <IconMiEdit v-else class="size-4 shrink-0" />
                {{ profile?.manualMode ? 'Disable' : 'Enable' }} manual mode
              </button>
            </div>
          </template>
        </app-dropdown>
      </div>
    </div>

    <app-table :loading="loading" :columns="columns" :list="getList">
      <template #default="{ items, columns: cols }">
        <div ref="containerList" class="relative overflow-y-auto">
          <div
            v-for="data of items"
            :key="data.name"
            class="group"
          >
            <item-mod
              v-model:order="data.order"
              v-model:enabled="data.enabled!"
              :columns="cols"
              :name="data.name!"
              :last-updated="data.lastUpdated"
              :image="data.image"
              :can-enable="data.canEnable"
              :can-reorder="data.canEnable && profile?.manualMode"
              :can-drag="isDragEnabled"
              :errors="getModErrors.get(data.name!) ?? []"
              :groups="getModGroups.get(data.name!) ?? []"
              @status="changeStatus(data.name!, $event)"
              @order="changeOrder(data.name!, $event)"
              @refresh="emit('refresh')"
            />
            <div class="h-px mx-2.5 bg-gray-800 group-last:bg-transparent select-none" :aria-hidden="true" />
          </div>
        </div>
      </template>
    </app-table>
  </div>

  <modal-profile-groups
    v-if="profile"
    ref="modalProfileGroups"
    :game-id="gameId"
    :profile="profile"
  />

  <!-- TODO: when adding new mods ask for confirmation if there are edits, and allow the user to undo/save em before proceedin -->
  <modal-mod
    ref="modalMod"
    :mods="getMissingMods"
    :loading="isAddingMods"
    @save="onAddMods"
  />

  <mods-edit-bar
    :visible="hasEdits"
    :can-undo="canUndo"
    :can-redo="canRedo"
    :loading="isSaving"

    @cancel="cancel()"
    @save="saveEdits"
    @undo="undo()"
    @redo="redo()"
  />
</template>

<script lang="ts" setup>
import type Sortable from 'sortablejs'
import type { AppTableColumn } from '~/types/common/AppTable'
import type { ModResponseDto, ProfileResponseDto } from '~/types/dto'

// Props
const props = defineProps<{
  gameId: string
  list: ModResponseDto[]
  loading?: boolean
  profile: Nullable<ProfileResponseDto>
}>()

// Emits
const emit = defineEmits<{ refresh: [] }>()

const SPACE_PATTERN = / /g

// Store
const gameStore = useGameStore()

// Template refs
const refModalMod = useTemplateRef('modalMod')
const refModalProfileGroups = useTemplateRef('modalProfileGroups')
const refContainerList = useTemplateRef('containerList')

// Non-reactive state
const sortOptions = [
  { value: '', label: 'Sort by', disabled: true, selected: true },
  { value: 'order', label: 'Order' },
  { value: 'name', label: 'Name' },
  { value: 'lastUpdate', label: 'Last update' },
]
const orderOptions = [
  { value: 'asc', label: 'Asc', selected: true },
  { value: 'desc', label: 'Desc' },
]

// Reactive state
const filters = ref({ search: '', sortBy: 'order', sortOrder: 'desc' })
const localList = ref<ModResponseDto[]>([])
const isSaving = ref(false)
const isTogglingManualMode = ref(false)
const isAddingMods = ref(false)
const { snapshots, undo, redo, commit, cancel, canUndo, canRedo } = useHistory(localList)

// Computed
const hasEdits = computed(() => snapshots.value.length > 1)
const allModsEnabled = computed(() => localList.value.every(mod => mod.enabled))
const isDragEnabled = computed(() => !!(props.profile?.manualMode) && filters.value.sortBy === 'order')
const columns = computed<AppTableColumn[]>(() => [
  { key: 'order', label: 'Order', span: isDragEnabled.value ? 2 : 1, headerClass: isDragEnabled.value ? 'ml-9' : '' },
  { key: 'enabled', label: 'Enabled?', span: 1 },
  { key: 'pack', label: 'Pack', span: 5 },
  { key: 'lastUpdate', label: 'Last update', span: 3 },
  { key: 'groups', label: 'Groups', span: 1 },
  { key: 'actions', label: '' },
])
const getMissingMods = computed(() => {
  const profileModNames = new Set(localList.value.flatMap(mod => mod.name ? [mod.name] : []))
  return gameStore.getGameMods.filter(mod => mod.name && !profileModNames.has(mod.name))
})
const getList = computed(() => {
  const search = filters.value.search.toLowerCase()
  const { sortBy, sortOrder } = filters.value
  const dir = sortOrder === 'asc' ? -1 : 1

  return localList.value
    .filter((item) => {
      if (!item.name)
        return false
      const name = item.name.toLowerCase()
      return name.includes(search.replace(SPACE_PATTERN, '_')) || name.includes(search)
    })
    .sort((a, b) => {
      switch (sortBy) {
        case 'order': return ((a.order ?? 0) - (b.order ?? 0)) * dir
        case 'name': return a.name!.localeCompare(b.name!) * dir
        case 'lastUpdate': {
          if (!a.lastUpdated || !b.lastUpdated)
            return 0
          return (new Date(b.lastUpdated).getTime() - new Date(a.lastUpdated).getTime()) * dir
        }
        default: return 0
      }
    })
})
const getModErrors = useModErrors(localList)
const getModGroups = computed(() => {
  const map = new Map<string, string[]>()
  for (const group of gameStore.getGroups) {
    if (!group.name || !group.mods)
      continue
    for (const modName of group.mods) {
      if (!modName)
        continue
      const existing = map.get(modName) ?? []
      existing.push(group.name)
      map.set(modName, existing)
    }
  }
  return map
})

// Watchers
watch(() => props.list, (value) => {
  localList.value = value.map(m => ({ ...m }))
  commit()
}, { immediate: true, deep: true })

// Functions
async function toggleManualMode() {
  isTogglingManualMode.value = true
  try {
    await useTauriInvoke('toggle_manual_mode', { profileId: props.profile!.id, gameId: props.gameId })
    await gameStore.fetchGame()
  }
  catch (error) {
    console.error('Failed to toggle manual mode:', error)
  }
  finally {
    isTogglingManualMode.value = false
  }
}

function toggleAllMods() {
  const toggle = !localList.value.every(mod => mod.enabled)
  localList.value.forEach((mod: ModResponseDto) => {
    mod.enabled = toggle
  })
}

async function saveEdits() {
  isSaving.value = true
  try {
    await useTauriInvoke('set_profile_mods', { mods: localList.value, profileId: props.profile!.id, gameId: props.gameId })
    commit()
    await gameStore.fetchGame()
  }
  catch (error) {
    console.error('Failed to save edits:', error)
  }
  finally {
    isSaving.value = false
  }
}

async function onAddMods(mods: string[]) {
  isAddingMods.value = true
  try {
    await useTauriInvoke('add_profile_mods', {
      profileId: props.profile!.id,
      gameId: props.gameId,
      mods: mods.map(name => ({ name, order: null, enabled: true })),
    })
    await gameStore.fetchGame()
    refModalMod.value?.close()
  }
  catch (error) {
    console.error('Failed to add mods:', error)
  }
  finally {
    isAddingMods.value = false
  }
}

function changeStatus(name: string, value: boolean) {
  const mod = localList.value.find((m: ModResponseDto) => m.name === name)
  if (mod)
    mod.enabled = value
}

function changeOrder(name: string, value: number) {
  const mod = localList.value.find((m: ModResponseDto) => m.name === name)
  if (mod)
    mod.order = value
}

// Misc
useMultiDrag(refContainerList, {
  enabled: isDragEnabled,
  onDragEnd(event: Sortable.SortableEvent) {
    const orderedNames = getList.value.map(m => m.name!)

    const moves = event.oldIndicies?.length
      ? event.oldIndicies.map((old, i) => ({ oldIndex: old.index, newIndex: event.newIndicies![i]!.index }))
      : [{ oldIndex: event.oldIndex!, newIndex: event.newIndex! }]

    const movedNames = moves.map(m => orderedNames[m.oldIndex]!)

    for (const { oldIndex } of moves.toSorted((a, b) => b.oldIndex - a.oldIndex))
      orderedNames.splice(oldIndex, 1)

    const insertions = movedNames.map((name, i) => ({ name, newIndex: moves[i]!.newIndex }))
    for (const { name, newIndex } of insertions.sort((a, b) => a.newIndex - b.newIndex))
      orderedNames.splice(newIndex, 0, name)

    orderedNames.forEach((name, i) => {
      const mod = localList.value.find(m => m.name === name)
      if (mod)
        mod.order = i + 1
    })
  },
})
</script>

<style>
.drag-highlight {
  border-left: 2px solid var(--color-purple-600);
}

.sortable-fallback {
  display: none !important;
}

.is-dragging,
.is-dragging * {
  cursor: grabbing !important;
}

.is-dragging input,
.is-dragging button,
.is-dragging label,
.is-dragging a,
.is-dragging [role='checkbox'] {
  pointer-events: none;
  user-select: none;
}
</style>
