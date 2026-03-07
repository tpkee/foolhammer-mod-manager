import type { GameResponseDto } from '~/types/dto'
import { ensureSaveResponseDto } from './saves'

export function ensureGameResponseDto(value: GameResponseDto): GameResponseDto {
  return {
    mods: value.mods ?? [],
    profiles: value.profiles ?? [],
    groups: value.groups ?? [],
    saves: (value.saves ?? []).filter(s => s != null).map(ensureSaveResponseDto),
    gameId: value.gameId,
    gamePath: value.gamePath,
    savesPath: value.savesPath ?? null,
    modsPath: value.modsPath,
    workshopPath: value.workshopPath ?? null,
  }
}
