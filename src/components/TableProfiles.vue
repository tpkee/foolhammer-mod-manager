<template>
  <div class="space-y-2.5">
    <app-input v-model="search" placeholder="Search..." type="search" class="w-full block" label="Search" />

    <app-table :columns="columns" :list="filteredProfiles">
      <template #default="{ columns: cols }">
        <div v-bind="containerProps" class="relative overflow-y-auto">
          <div v-bind="wrapperProps">
            <div
              v-for="({ data }, index) of virtualizedList"
              :key="index"
              class="group"
              :style="{ height: `${ITEM_HEIGHT}px` }"
            >
              <item-profile
                :profile="data"
                :is-active="data.id === gameStore.selectedProfile"
                :game-id="gameId"
                :columns="cols"
                @refresh="emit('refresh')"
              />
              <div class="h-px mx-2.5 bg-gray-800 group-last:bg-transparent select-none" :aria-hidden="true" />
            </div>
          </div>
        </div>
      </template>
    </app-table>
  </div>
</template>

<script lang="ts" setup>
import type { AppTableColumn } from '~/types/common/AppTable'
import type { ProfileResponseDto } from '~/types/dto/profiles'

// Props
const props = defineProps<{
  profiles: ProfileResponseDto[]
  gameId: string
}>()

// Emits
const emit = defineEmits<{ refresh: [] }>()

// Stores
const gameStore = useGameStore()

// Non-reactive state
const ITEM_HEIGHT = 56 // px
const columns: AppTableColumn[] = [
  { key: 'select', label: '', span: 1 },
  { key: 'name', label: 'Name', span: 6 },
  { key: 'activeMods', label: 'Active Mods', span: 4 },
  { key: 'actions', label: '', span: 1, cellClass: 'justify-self-end' },
]

// Reactive state
const search = ref('')

// Computed
const filteredProfiles = computed(() => {
  const q = search.value.toLowerCase()
  return props.profiles.filter(p => p?.name?.toLowerCase().includes(q))
})

// Reactive state
const { list: virtualizedList, containerProps, wrapperProps } = useVirtualList(
  filteredProfiles,
  { itemHeight: ITEM_HEIGHT },
)
</script>
