<template>
  <main>
    <NuxtRouteAnnouncer />
    <div class="flex">
      <div class="p-2.5 grow">
        <NuxtPage />
      </div>

      <app-sidebar class="relative overflow-hidden min-w-17.5" :games="listSupportedGames" :current-game="preferencesStore.currentGame" />
    </div>
  </main>
</template>

<script setup lang="ts">
const preferencesStore = usePreferencesStore()

const { data: userSettings, refresh: refreshUserSettings } = await useAsyncData<UserSettings>('user-settings', () => useTauriInvoke('get_state'))
const { data: listSupportedGames } = await useAsyncData<string[]>(`supported-games`, () => useTauriInvoke('get_supported_games'), {
  default: () => [],
})
const unlistenUserSettings = useTauriListener('update/user-settings', _e => refreshUserSettings())

watch(userSettings, (newSettings) => {
  preferencesStore.setSettings(newSettings ?? null)
  if (!preferencesStore.currentGame && newSettings) {
    preferencesStore.setCurrentGame(newSettings.gameId ?? newSettings.currentGame)
  }
}, { immediate: true })

watch(listSupportedGames, (games) => {
  if (!Array.isArray(games) || games.length === 0) {
    preferencesStore.setCurrentGame(null)
    return
  }

  if (!preferencesStore.currentGame) {
    preferencesStore.setCurrentGame(games[0]!)
  }
}, { immediate: true })

provide('currentGame', preferencesStore.currentGame)

onUnmounted(unlistenUserSettings)
</script>
