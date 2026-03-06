import type { PackResponseDto } from './packs'
import type { ProfileResponseDto } from './profiles'
import type { SaveResponseDto } from './saves'

export type GameResponseDto = RecursivePartial<{
  mods: PackResponseDto[]
  profiles: ProfileResponseDto[]
  saves: SaveResponseDto[]
  defaultProfile: Nullable<string>
  gameId: string
  gamePath: string
  savesPath: Nullable<string>
  modsPath: string
  workshopPath: Nullable<string>
}>
