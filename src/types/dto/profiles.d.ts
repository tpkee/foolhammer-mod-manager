import type { ModRequestDto, ModResponseDto } from './mods'

export type ProfileResponseDto = RecursivePartial<{
  id: string
  name: string
  mods: ModResponseDto[]
  default: boolean
  manualMode: boolean
}>

export interface ProfileRequestDto {
  id: string
  gameId: string
  name: string
  default?: boolean | null
  manualMode?: boolean | null
  mods: ModRequestDto[]
}
