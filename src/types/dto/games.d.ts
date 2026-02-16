import type { PackResponseDto } from './packs'
import type { ProfileResponseDto } from './profiles'

export type GameResponseDto = RecursivePartial<{
  mods: PackResponseDto[]
  profiles: ProfileResponseDto[]
  defaultProfile: Nullable<string>
  gameId: string
  gamePath: string
  savesPath: Nullable<string>
  modsPath: string
  workshopPath: Nullable<string>
}>
