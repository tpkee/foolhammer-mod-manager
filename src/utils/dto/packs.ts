import type { ModRequestDto, PackResponseDto } from '~/types/dto'
import { ModRequestSchema } from './schemas'

export function packResponseToRequest(pack: PackResponseDto): ModRequestDto {
  return ModRequestSchema.parse({
    name: pack.name,
    enabled: false,
    order: 0,
  })
}
