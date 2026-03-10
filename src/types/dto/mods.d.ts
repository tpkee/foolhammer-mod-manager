import type { z } from 'zod'
import type { ModRequestSchema, ModResponseSchema } from '~/schemas'

export type ModResponseDto = z.infer<typeof ModResponseSchema>
export type ModRequestDto = z.infer<typeof ModRequestSchema>
