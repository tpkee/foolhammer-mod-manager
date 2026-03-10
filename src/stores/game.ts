import type { GameResponseDto, GroupResponseDto, ModResponseDto, PackResponseDto, ProfileResponseDto, SaveResponseDto } from '~/types/dto'
import { GameResponseSchema } from '~/schemas'

export const useGameStore = defineStore('gameStore', () => {
  const selectedGame = ref<Nullable<string>>(null)
  const selectedProfile = ref<Nullable<string>>(null)
  const currentGame = ref<Nullable<GameResponseDto>>(null)

  const fetchStatus = ref<'pending' | 'success' | 'error'>('pending')

  // Getters
  const getProfiles = computed<ProfileResponseDto[]>(() =>
    currentGame.value?.profiles ?? [],
  )

  const getProfile = computed<Nullable<ProfileResponseDto>>(() =>
    getProfiles.value.find(profile => profile.id === selectedProfile.value) ?? null,
  )

  const getProfileMods = computed<ModResponseDto[]>(() =>
    getProfile.value?.mods ?? [],
  )

  const getGameMods = computed<PackResponseDto[]>(() =>
    currentGame.value?.mods ?? [],
  )

  const getSaves = computed<SaveResponseDto[]>(() =>
    currentGame.value?.saves ?? [],
  )

  const getGroups = computed<GroupResponseDto[]>(() =>
    currentGame.value?.groups ?? [],
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
      console.log('Fetched game data:', raw)
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
