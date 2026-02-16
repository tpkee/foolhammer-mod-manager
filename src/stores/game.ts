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
    getProfile(): Nullable<ProfileResponseDto> {
      return this.getProfiles.find(profile => profile?.name && profile?.name === this.selectedProfile)
    },
    // returns the mods for the active profile
    getProfileMods(): ModResponseDto[] {
      return transformToNonNullable(this.getProfile?.mods)
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

      if (!game) {
        this.selectedProfile = null
        return
      }

      this.setProfile(this.selectedProfile ?? game.defaultProfile ?? null)
    },
  },
})

function transformToNonNullable<T>(array: Nullable<T[]>): NonNullable<T>[] {
  if (!array)
    return []
  return array.filter(item => item != null)
}
