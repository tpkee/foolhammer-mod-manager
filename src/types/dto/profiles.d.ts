import type { ModRequestDto, ModResponseDto } from './mods'

export type ProfileResponseDto = RecursivePartial<{
  id: string
  name: string
  mods: ModResponseDto[]
  default: boolean
  manualMode: boolean
  groups: string[]
}>

export interface ProfileRequestDto {
  gameId: string
  name: string
  default?: boolean | null
  manualMode?: boolean | null
  mods: ModRequestDto[]
}
