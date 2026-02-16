import type { ModRequestDto, PackResponseDto } from '~/types/dto'

export function packResponseToRequest(pack: PackResponseDto): ModRequestDto {
  if (!pack.name)
    throw new Error('Pack name is required to convert to ModRequestDto')

  return {
    name: pack.name,
    enabled: false,
    order: 0,
  }
}
