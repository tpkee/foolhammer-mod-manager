import type { z } from 'zod'
import type { SaveResponseSchema } from '~/schemas'

export type SaveResponseDto = z.infer<typeof SaveResponseSchema>
