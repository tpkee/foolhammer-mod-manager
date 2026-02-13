import type { PackResponseDto } from './packs'
import type { ProfileResponseDto } from './profiles'

export interface GameResponseDto {
  mods: PackResponseDto[]
  profiles: ProfileResponseDto[]
  gameId: string
  gamePath: string
  savesPath: string | null
  modsPath: string
  workshopPath: string | null
}
