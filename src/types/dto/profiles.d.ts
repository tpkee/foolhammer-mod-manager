import type { z } from 'zod'
import type { ProfileRequestSchema, ProfileResponseSchema } from '~/schemas'

export type ProfileResponseDto = z.infer<typeof ProfileResponseSchema>
export type ProfileRequestDto = z.infer<typeof ProfileRequestSchema>
