import type { GameResponseDto, GroupResponseDto, ModResponseDto, PackResponseDto, ProfileResponseDto, SaveResponseDto } from '~/types/dto'
import { GameResponseSchema } from '~/schemas'

export const useGameStore = defineStore('gameStore', () => {
  const selectedGame = ref<Nullable<string>>(null)
  const selectedProfile = ref<Nullable<string>>(null)
  const currentGame = ref<Nullable<GameResponseDto>>(null)

  const fetchStatus = ref<'pending' | 'success' | 'error'>('pending')

  // Getters
  const getProfiles = computed<ProfileResponseDto[]>(() =>
    transformToNonNullable(currentGame.value?.profiles),
  )

  const getProfile = computed<Nullable<ProfileResponseDto>>(() =>
    getProfiles.value.find(profile => profile?.id && profile.id === selectedProfile.value),
  )

  const getProfileMods = computed<ModResponseDto[]>(() =>
    transformToNonNullable(getProfile.value?.mods),
  )

  const getGameMods = computed<PackResponseDto[]>(() =>
    transformToNonNullable(currentGame.value?.mods),
  )

  const getSaves = computed<SaveResponseDto[]>(() =>
    transformToNonNullable(currentGame.value?.saves),
  )

  const getGroups = computed<GroupResponseDto[]>(() =>
    transformToNonNullable(currentGame.value?.groups),
  )

  // Actions
  function setProfile(profileId: Nullable<string>) {
    selectedProfile.value = profileId
  }

  function setGame(game: Nullable<GameResponseDto>) {
    currentGame.value = game

    if (!game) {
      selectedProfile.value = null
      return
    }

    setProfile(selectedProfile.value ?? game.defaultProfile ?? null)
  }

  async function fetchGame() {
    if (!selectedGame.value) {
      setGame(null)
      return
    }

    fetchStatus.value = 'pending'

    try {
      const raw = await useTauriInvoke<Nullable<GameResponseDto>>('get_game', {
        gameId: selectedGame.value,
      })
      const game = raw ? GameResponseSchema.parse(raw) : null
      setGame(game)
      fetchStatus.value = 'success'
      return game
    }
    catch (error) {
      console.error('Failed to fetch game data:', error)
      fetchStatus.value = 'error'
      setGame(null)
      throw error
    }
  }

  function setGameId(gameId: Nullable<string>) {
    selectedGame.value = gameId
  }

  watch(selectedGame, (gameId) => {
    if (gameId) {
      fetchGame()
    }
    else {
      setGame(null)
    }
  })

  return {
    selectedGame,
    selectedProfile,
    currentGame,
    fetchStatus,
    getProfiles,
    getProfile,
    getProfileMods,
    getGameMods,
    getSaves,
    getGroups,
    setGameId,
    setProfile,
    setGame,
    fetchGame,
  }
})

function transformToNonNullable<T>(array: Nullable<T[]>): NonNullable<T>[] {
  if (!array)
    return []
  return array.filter(item => item != null)
}
