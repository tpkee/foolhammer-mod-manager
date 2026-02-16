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
// Stores
const preferencesStore = usePreferencesStore()
const gameStore = useGameStore()

// Reactive state
const { locale } = useI18n()

// Fetching
const { data: userSettings, refresh: refreshUserSettings } = await useAsyncData<Nullable<RecursivePartial<UserSettings>>>('user-settings', () => useTauriInvoke('get_user_settings'), {
  default: () => null,
})

const { data: listSupportedGames } = await useAsyncData<string[]>(`supported-games`, () => useTauriInvoke('get_supported_games'), {
  default: () => [],
})

// Watchers
watch(listSupportedGames, (games) => {
  if (!Array.isArray(games) || games.length === 0) {
    preferencesStore.setCurrentGame(null)
    return
  }

  if (!preferencesStore.currentGame) {
    preferencesStore.setCurrentGame(games[0]!)
  }
}, { immediate: true })

watch(userSettings, (newSettings) => {
  preferencesStore.setSettings(newSettings)

  if (gameStore.selectedGame != null && newSettings?.defaultGame) {
    preferencesStore.setCurrentGame(newSettings.defaultGame)
  }
}, { immediate: true })

// Misc
const unlistenUserSettings = useTauriListener('update/user-settings', _e => refreshUserSettings())

useHeadSafe({
  htmlAttrs: {
    lang: locale.value,
  },
})

// Lifecycle hooks
onUnmounted(unlistenUserSettings)
</script>
