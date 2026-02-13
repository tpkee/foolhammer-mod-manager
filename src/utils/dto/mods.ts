import type { ModRequestDto, ModResponseDto } from '~/types/dto'

export function modResponseToRequest(mod: ModResponseDto): ModRequestDto {
  return {
    name: mod.name,
    enabled: mod.enabled,
    order: mod.order,
  }
}

export function modRequestToResponse(
  mod: ModRequestDto,
  overrides: Partial<ModResponseDto> = {},
): ModResponseDto {
  return {
    name: mod.name,
    enabled: mod.enabled,
    order: mod.order,
    path: overrides.path ?? null,
    canEnable: overrides.canEnable ?? false,
    lastUpdated: overrides.lastUpdated ?? null,
    fromSteamWorkshop: overrides.fromSteamWorkshop ?? false,
    image: overrides.image ?? null,
  }
}
