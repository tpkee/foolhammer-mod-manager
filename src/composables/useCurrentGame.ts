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

  watch(() => preferencesStore.currentGame, (gameId) => {
    if (gameId) {
      const defaultProfile = currentGameData.value?.defaultProfile
      preferencesStore.setCurrentProfile(defaultProfile ?? null)
    }
  }, {
    immediate: true,
  })

  const getProfiles = computed(() => {
    return currentGameData.value?.profiles ?? []
  })

  const getCurrentProfile = computed(() => {
    if (!currentGameData.value || !currentGameData.value.profiles?.length)
      return null

    if (preferencesStore.currentProfile) {
      const profile = getProfiles.value.find(p => p.name === preferencesStore.currentProfile)
      if (profile) {
        return profile ?? null
      }
    }
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
