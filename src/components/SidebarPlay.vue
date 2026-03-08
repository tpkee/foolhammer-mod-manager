<template>
  <sidebar-button
    :label="getLabel"
    :tooltip="getTooltip"
    :disabled="variant === 'continue' && !currentSave"
    @click="handleClick"
  >
    <template v-if="variant === 'continue'" #menu="{ close }">
      <div class="py-1 min-w-44 max-w-72 w-fit overflow-y-scroll max-h-64">
        <div class="px-3 py-1.5 space-y-1.5">
          <p class="text-xs text-gray-400 uppercase tracking-wide">
            Select save
          </p>
          <app-input v-model="searchQuery" label="Search" sr-only-label placeholder="Search saves..." class="mb-3 block w-full" />
          <p v-if="filteredSaves.length === 0" class="text-sm text-gray-500">
            No saves found
          </p>
          <template v-else>
            <div v-for="save in filteredSaves" :key="save.path">
              <app-radio v-model="currentSave" :value="save.name" :label="save.name" />
              <span v-if="save.lastAccessed" class="block text-xs text-gray-500 pl-5.5">{{ d(save.lastAccessed) }}</span>
            </div>
          </template>
        </div>
      </div>
    </template>

    <div v-if="isLoading" class="size-10 flex items-center justify-center">
      <IconSvgSpinnersRingResize class="text-2xl" />
    </div>
    <IconMiStop v-else-if="gameStatus === 'success'" class="size-10" />
    <IconMiNext v-else-if="variant === 'continue'" class="size-10" />
    <IconMiPlay v-else class="size-10" />
  </sidebar-button>
</template>

<script lang="ts" setup>
import type { UnlistenFn } from '@tauri-apps/api/event'
import type { ProfileResponseDto, SaveResponseDto } from '~/types/dto'
import type { GameRunnerStatus } from '~/types/GameRunnerStatus'

const props = defineProps<{
  currentGame: string
  profile: ProfileResponseDto
  variant?: 'play' | 'continue'
  saves?: SaveResponseDto[]
}>()

// Reactive state
const gameStatus = ref<GameRunnerStatus>()
const currentSave = ref<Nullable<string>>()
const searchQuery = ref('')

// Non reactive state
const stopGameListener: Promise<UnlistenFn> = useTauriListener('game_closed', () => {
  gameStatus.value = undefined
})

// Composables
const { t, d } = useI18n()

// Computed
const isLoading = computed(() => gameStatus.value === 'start')
const getGameName = computed(() => t(`games.${props.currentGame}`))

const filteredSaves = computed(() => {
  const q = searchQuery.value.trim().toLowerCase()
  if (!q)
    return props.saves ?? []
  return (props.saves ?? []).filter(save => save.name!.toLowerCase().includes(q))
})

const getLabel = computed(() => {
  switch (gameStatus.value) {
    case 'start':
      return `Starting ${getGameName.value}...`
    case 'success':
      return `Stop ${getGameName.value}`
    default:
      return props.variant === 'continue'
        ? 'Continue'
        : `Play ${getGameName.value} with profile ${props.profile.name}`
  }
})

const getTooltip = computed(() => {
  if (props.variant === 'continue' && !gameStatus.value) {
    return currentSave.value ? `Continue: ${currentSave.value}` : 'Continue'
  }
  return getLabel.value
})

watch(() => props.saves, (arr) => {
  if (Array.isArray(arr) && arr.length > 0) {
    currentSave.value = arr.toSorted((a, b) => {
      const aTime = a.lastAccessed ? new Date(a.lastAccessed).getTime() : 0
      const bTime = b.lastAccessed ? new Date(b.lastAccessed).getTime() : 0
      return bTime - aTime
    })[0]?.name
  }
  else {
    currentSave.value = null
  }
}, { immediate: true })

async function handleClick() {
  if (gameStatus.value === 'success') {
    await stopGame()
    return
  }

  await playGame()
}

async function stopGame() {
  try {
    await useTauriInvoke('stop_game', {
      gameId: props.currentGame,
    })
    gameStatus.value = undefined
  }
  catch (e) {
    console.error('Failed to stop game:', e)
  }
}

async function playGame() {
  gameStatus.value = 'start'
  try {
    await useTauriInvoke('start_game', {
      gameId: props.currentGame,
      profileId: props.profile.id,
      saveName: props.variant === 'continue' && currentSave.value
        ? currentSave.value
        : undefined,
    })
    gameStatus.value = 'success'
  }
  catch (e) {
    gameStatus.value = 'error'
    console.error('Failed to start game:', e)
  }
}

onUnmounted(async () => {
  if (stopGameListener) {
    (await stopGameListener)()
  }
})
</script>
