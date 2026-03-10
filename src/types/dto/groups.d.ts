import type { z } from 'zod'
import type { GroupRequestSchema, GroupResponseSchema } from '~/schemas'

export type GroupResponseDto = z.infer<typeof GroupResponseSchema>
export type GroupRequestDto = z.infer<typeof GroupRequestSchema>
