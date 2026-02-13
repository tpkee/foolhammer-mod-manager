import type { ModRequestDto, ModResponseDto } from './mods'

export interface ProfileResponseDto {
  name: string
  mods: ModResponseDto[]
  default: boolean
  manualMode: boolean
}

export interface ProfileRequestDto {
  gameId: string
  name: string
  default?: boolean | null
  manualMode?: boolean | null
  mods: ModRequestDto[]
}
