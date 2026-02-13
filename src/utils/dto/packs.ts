import type { ModRequestDto, PackResponseDto } from '~/types/dto'

export function packResponseToRequestDisabled(pack: PackResponseDto): ModRequestDto {
  return {
    name: pack.name,
    enabled: false,
    order: 0,
  }
}
