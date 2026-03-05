import type { ModRequestDto, ModResponseDto } from '~/types/dto'

export function modResponseToRequest(mod: ModResponseDto): ModRequestDto {
  if (!mod.name) {
    throw new Error('Mod name is required')
  }
  return {
    name: mod.name,
    enabled: !!mod.enabled,
    order: mod.order,
  }
}
