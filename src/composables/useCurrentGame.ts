import type { GameResponseDto } from '~/types/dto'

export function useCurrentGame() {
  const preferencesStore = usePreferencesStore()

  const { data: currentGameData, refresh: refreshGame } = useAsyncData<GameResponseDto>(
    () => `${preferencesStore.currentGame}-game`,
    () => useTauriInvoke('get_game', { gameId: preferencesStore.currentGame }),
    {
      watch: [() => preferencesStore.currentGame],
      immediate: true,
    },
  )

  watch(() => preferencesStore.currentGame, (newGameId) => {
    if (!newGameId) {
      const defaultProfile = currentGameData.value?.profiles.find(p => p.default)
      if (defaultProfile) {
        preferencesStore.setCurrentProfile(defaultProfile.name)
      }
      else {
        preferencesStore.setCurrentProfile(null)
      }
    }
  })

  const getCurrentProfile = computed(() => {
    if (!currentGameData.value || !currentGameData.value.profiles?.length)
      return null

    const defaultProfile = currentGameData.value.profiles.find(p => p.name === preferencesStore.currentProfile)
    return defaultProfile ?? currentGameData.value.profiles[0]
  })

  const getProfiles = computed(() => {
    return currentGameData.value?.profiles ?? []
  })

  const getMods = computed(() => {
    return getCurrentProfile.value?.mods ?? []
  })

  return {
    currentGameData,
    getCurrentProfile,
    getProfiles,
    refreshGame,
    getMods,
  }
}
