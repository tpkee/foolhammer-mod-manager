<template>
  <main>
    <NuxtRouteAnnouncer />
    <div class="flex">
      <div class="p-2.5 grow">
        <NuxtPage />
      </div>

      <app-sidebar class="relative overflow-hidden" :games="listSupportedGames" :current-game="currentGame" />
    </div>
  </main>
</template>

<script setup lang="ts">
const settingsStore = useSettingsStore()
const currentGame = ref<Nullable<string>>(null)

const { data: userSettings, refresh: refreshUserSettings } = await useAsyncData<UserSettings>('user-settings', () => useTauriInvoke('get_state'))
const { data: listSupportedGames } = await useAsyncData<string[]>(`supported-games`, () => useTauriInvoke('get_supported_games'), {
  default: () => [],
})
const unlistenUserSettings = useTauriListener('update/user-settings', _e => refreshUserSettings())

watch(userSettings, (newSettings) => {
  if (newSettings) {
    settingsStore.setSettings(newSettings)
    if (!currentGame.value) {
      currentGame.value = newSettings.gameId ?? newSettings.currentGame
    }
  }
}, { immediate: true })

watch(listSupportedGames, (games) => {
  if (!Array.isArray(games) || games.length === 0) {
    currentGame.value = null
    return
  }

  if (!currentGame.value) {
    currentGame.value = games[0]!
  }
}, { immediate: true })

provide('currentGame', currentGame)

onUnmounted(unlistenUserSettings)
</script>
