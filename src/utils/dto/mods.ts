import type { ModRequestDto, ModResponseDto } from '~/types/dto'
import { ModRequestSchema } from './schemas'

export function modResponseToRequest(mod: ModResponseDto): ModRequestDto {
  return ModRequestSchema.parse({
    name: mod.name,
    enabled: !!mod.enabled,
    order: mod.order,
  })
}
