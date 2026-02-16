<template>
  <div>
    <NuxtLoadingIndicator color="#8200db" />
    <NuxtRouteAnnouncer />
    <div class="flex">
      <div class="p-2.5 grow">
        <NuxtPage />
      </div>

      <app-sidebar
        class="relative overflow-hidden min-w-17.5"
        :games="listSupportedGames"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { GameResponseDto } from './types/dto'
import type { SettingsResponseDto } from './types/dto/settings'

// Stores
const settingsStore = useSettingsStore()
const gameStore = useGameStore()

// Reactive state
const { locale } = useI18n()

// Fetching
const { data: listSupportedGames } = await useAsyncData<string[]>(`supported-games`, () => useTauriInvoke('get_supported_games'), {
  default: () => [],
})

const { data: game, refresh } = await useAsyncData<Nullable<GameResponseDto>>(`game-${gameStore.selectedGame}`, () => {
  if (!gameStore.selectedGame)
    return Promise.resolve(null)

  return useTauriInvoke('get_game', { gameId: gameStore.selectedGame })
}, {
  default: () => null,
  watch: [() => gameStore.selectedGame],
})

const { data: userSettings, refresh: refreshUserSettings } = await useAsyncData<Nullable<SettingsResponseDto>>('user-settings', () => useTauriInvoke('get_user_settings'), {
  default: () => null,
  immediate: false,
})

// Watchers
watch(game, () => {
  gameStore.setGame(game.value)
}, { immediate: true })

watch(listSupportedGames, () => {
  refreshUserSettings()
}, { immediate: true })

watch(userSettings, (newSettings) => {
  settingsStore.setSettings(newSettings)

  if (gameStore.selectedGame != null && newSettings?.defaultGame) {
    gameStore.setGameId(newSettings.defaultGame)
  }
}, { immediate: true })

// Misc
const unlistenUserSettings = useTauriListener('update/user-settings', _e => refreshUserSettings())

useHeadSafe({
  htmlAttrs: {
    lang: locale.value,
  },
})
provide('refreshGame', refresh)

// Lifecycle hooks
onUnmounted(unlistenUserSettings)
</script>
