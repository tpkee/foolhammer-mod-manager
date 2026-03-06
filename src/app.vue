<template>
  <div>
    <NuxtLoadingIndicator color="#8200db" />
    <NuxtRouteAnnouncer />
    <div class="flex">
      <div class="p-2.5 grow">
        <NuxtPage />
      </div>

      <div
        class="shrink"
      >
        <app-sidebar
          class="sticky top-0 overflow-hidden"
          :games="listSupportedGames"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SettingsResponseDto } from '~/types/dto'

// Stores
const settingsStore = useSettingsStore()
const gameStore = useGameStore()

// Reactive state
const { locale } = useI18n()

// Fetching
const { data: listSupportedGames } = await useAsyncData<string[]>(`supported-games`, () => useTauriInvoke('get_supported_games'), {
  default: () => [],
})

const { data: userSettings, refresh: refreshUserSettings } = await useAsyncData<Nullable<SettingsResponseDto>>('user-settings', () => useTauriInvoke('get_user_settings'), {
  default: () => null,
})

// Non reactive state
const unlistenUserSettings = useTauriListener('update_user_settings', _e => refreshUserSettings())
const unlistenRefreshGame = useTauriListener('refresh_game', _e => gameStore.fetchGame())

// Watchers
watch(userSettings, (newSettings) => {
  settingsStore.setSettings(newSettings)

  if (!gameStore.selectedGame && newSettings?.defaultGame) {
    gameStore.setGameId(newSettings.defaultGame)
  }
}, { immediate: true })

// Misc
useHeadSafe({
  htmlAttrs: {
    lang: locale.value,
  },
})

// Lifecycle hooks
onBeforeUnmount(() => {
  const unlisten = (promise: Promise<() => void>) => promise.then(unlisten => unlisten())

  unlisten(unlistenUserSettings)
  unlisten(unlistenRefreshGame)
})
</script>
