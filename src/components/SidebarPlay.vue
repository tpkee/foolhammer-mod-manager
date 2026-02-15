<template>
  <sidebar-button
    :label="getLabel"
    :tooltip="getLabel"
    @click="handleClick"
  >
    <div v-if="isLoading" class="size-10 flex items-center justify-center">
      <nuxt-icon name="svg-spinners:ring-resize" class="text-2xl" />
    </div>
    <nuxt-icon v-else-if="gameStatus === 'success'" name="mi:stop" class="size-10" />
    <nuxt-icon v-else name="mi:play" class="size-10" />
  </sidebar-button>
</template>

<script lang="ts" setup>
import type { UnlistenFn } from '@tauri-apps/api/event'
import type { GameRunnerStatus } from '~/types/GameRunnerStatus'

const props = defineProps<{
  currentGame: string
  profileName: string
}>()

// Reactive state
const gameStatus = ref<GameRunnerStatus>()

// Non reactive state
let launchGameListener: Nullable<Promise<UnlistenFn>> = null
const stopGameListener: Promise<UnlistenFn> = useTauriListener('game_closed', () => {
  gameStatus.value = undefined
})

// Composables
const { t } = useI18n()

// Computed
const isLoading = computed(() => gameStatus.value === 'start')
const getGameName = computed(() => t(`games.${props.currentGame}`))
const getLabel = computed(() => {
  switch (gameStatus.value) {
    case 'start':
      return `Starting ${getGameName.value}...`
    case 'success':
      return `Stop ${getGameName.value}`
    default:
      return `Play ${getGameName.value} with profile ${props.profileName}`
  }
})

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
  }
  catch (e) {
    console.error('Failed to stop game:', e)
  }
}

async function playGame() {
  try {
    initGameListener()

    await useTauriInvoke('start_game', {
      gameId: props.currentGame,
      profileName: props.profileName,
    })
  }
  catch (e) {
    console.error('Failed to start game:', e)
  }
}

function initGameListener() {
  gameStatus.value = undefined
  launchGameListener = useTauriListener<GameRunnerStatus>('game_launch', (status) => {
    switch (status.payload) {
      case 'start':
        gameStatus.value = 'start'
        break
      case 'error':
        // TODO: should trigger a notification with the error message
        gameStatus.value = 'error'
        break
      case 'success':
        gameStatus.value = 'success'
        break
    }
  })
}

onUnmounted(async () => {
  if (launchGameListener) {
    (await launchGameListener)()
  }

  if (stopGameListener) {
    (await stopGameListener)()
  }
})
</script>
