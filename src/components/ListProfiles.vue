<template>
  <div class="space-y-2.5">
    <app-input v-model="search" placeholder="Search..." type="search" class="w-full" label="Search" />

    <div class="border overflow-hidden rounded border-gray-700">
      <div class="grid grid-cols-12 p-2.5 border-b border-gray-800 items-center gap-2.5 text-left [&>p]:whitespace-nowrap bg-gray-800">
        <div class="col-span-1" />
        <p class="col-span-6">
          Name
        </p>
        <p class="col-span-4">
          Active Mods
        </p>
        <div class="col-span-1" />
      </div>

      <div v-bind="containerProps" class="relative max-h-[80svh]">
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
              @refresh="emit('refresh')"
            />
            <hr class="h-px mx-2.5 border-gray-800 group-last:border-none select-none" aria-hidden="true">
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { ProfileResponseDto } from '~/types/dto/profiles'

const props = defineProps<{
  profiles: ProfileResponseDto[]
  gameId: string
}>()

const emit = defineEmits<{
  refresh: []
}>()

const gameStore = useGameStore()

const ITEM_HEIGHT = 56 // px

const search = ref('')

const filteredProfiles = computed(() => {
  const q = search.value.toLowerCase()
  return props.profiles.filter(p => p?.name?.toLowerCase().includes(q))
})

const { list: virtualizedList, containerProps, wrapperProps } = useVirtualList(
  filteredProfiles,
  { itemHeight: ITEM_HEIGHT },
)
</script>
