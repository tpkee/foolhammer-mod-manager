import type { GameResponseDto, ModResponseDto, PackResponseDto, ProfileResponseDto } from '~/types/dto'

interface GameStore {
  selectedGame: Nullable<string> // TODO: Potentially refactor this, there shouldn't be a need to store the gameId separately from the game object (maybe for background updates and stuff? idk)
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
      return this.getProfiles.find(profile => profile?.id && profile.id === this.selectedProfile)
    },
    // returns the mods for the active profile
    getProfileMods(): ModResponseDto[] {
      return transformToNonNullable(this.getProfile?.mods)
    },
    // returns all the mods available for the current game
    getGameMods(state): PackResponseDto[] {
      return transformToNonNullable(state.currentGame?.mods)
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
    setProfile(profileId: Nullable<string>) {
      this.selectedProfile = profileId
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
