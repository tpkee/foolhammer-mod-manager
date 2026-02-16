import type { GameResponseDto, ModResponseDto, PackResponseDto, ProfileResponseDto } from '~/types/dto'

interface GameStore {
  selectedGame: Nullable<string>
  selectedProfile: Nullable<string>
  currentGame: Nullable<GameResponseDto>
}
export const useGameStore = defineStore('gameStore', {
  state: (): GameStore => ({
    selectedGame: null,
    selectedProfile: null,
    currentGame: null,
  }),
  getters: {
    getProfiles(state): ProfileResponseDto[] {
      return transformToNonNullable(state.currentGame?.profiles)
    },
    getProfile(state): Nullable<ProfileResponseDto> {
      return transformToNonNullable(state.currentGame?.profiles).find(profile => profile?.name && profile?.name === state.selectedProfile)
    },
    // returns the mods for the active profile
    getProfileMods(state): ModResponseDto[] {
      const activeProfile = state.currentGame?.profiles?.find(profile => profile?.name && profile?.name === state.selectedProfile)
      return transformToNonNullable(activeProfile?.mods)
    },
    // returns all the mods available for the current game
    getGameMods(state): PackResponseDto[] {
      return transformToNonNullable(state.currentGame?.mods)
    },
    getDataKey(state): string {
      return `game-${state.selectedGame}`
    },
  },
  actions: {
    setGameId(gameId: Nullable<string>) {
      if (!gameId) {
        this.setGame(null)
        return
      }
      this.selectedGame = gameId
    },
    setProfile(profileName: Nullable<string>) {
      this.selectedProfile = profileName
    },
    setGame(game: Nullable<GameResponseDto>) {
      this.currentGame = game
      // we have to set the profile
      if (!game || !game.defaultProfile) {
        this.setProfile(null)
        return
      }

      this.setProfile(game.defaultProfile)
    },
  },
})

function transformToNonNullable<T>(array: Nullable<T[]>): NonNullable<T>[] {
  if (!array)
    return []
  return array.filter(item => item != null)
}
