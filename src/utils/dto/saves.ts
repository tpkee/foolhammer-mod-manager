import type { SaveResponseDto } from '~/types/dto'

export function ensureSaveResponseDto(save: SaveResponseDto): SaveResponseDto {
  return {
    name: save.name ?? '',
    path: save.path ?? '',
    lastUpdated: save.lastUpdated ?? null,
    lastAccessed: save.lastAccessed ?? null,
  }
}
