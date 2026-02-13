import type { GameResponseDto } from '~/types/dto'

export function ensureGameResponseDto(value: GameResponseDto): GameResponseDto {
  return {
    mods: value.mods ?? [],
    profiles: value.profiles ?? [],
    gameId: value.gameId,
    gamePath: value.gamePath,
    savesPath: value.savesPath ?? null,
    modsPath: value.modsPath,
    workshopPath: value.workshopPath ?? null,
  }
}
