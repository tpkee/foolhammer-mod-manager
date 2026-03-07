<template>
  <div class="space-y-2.5">
    <app-input v-model="search" placeholder="Search..." type="search" class="w-full block" label="Search" />

    <app-table :columns="columns" :list="filteredGroups">
      <template #default="{ columns: cols }">
        <div class="relative overflow-y-auto">
          <div
            v-for="(data, index) of filteredGroups"
            :key="index"
            class="group"
          >
            <item-group
              :group="data"
              :game-id="gameId"
              :columns="cols"
            />
            <div class="h-px mx-2.5 bg-gray-800 group-last:bg-transparent select-none" :aria-hidden="true" />
          </div>
        </div>
      </template>
    </app-table>
  </div>
</template>

<script lang="ts" setup>
import type { AppTableColumn } from '~/types/common/AppTable'
import type { GroupResponseDto } from '~/types/dto'

const props = defineProps<{
  groups: GroupResponseDto[]
  gameId: string
}>()

const columns: AppTableColumn[] = [
  { key: 'name', label: 'Name', span: 7 },
  { key: 'mods', label: 'Mods', span: 4 },
  { key: 'actions', label: '', span: 1, cellClass: 'justify-self-end' },
]

const search = ref('')

const filteredGroups = computed(() => {
  const q = search.value.toLowerCase()
  return props.groups.filter(g => g?.name?.toLowerCase().includes(q))
})
</script>
