import type { z } from 'zod'
import type { PackResponseSchema } from '~/schemas'

export type PackResponseDto = z.infer<typeof PackResponseSchema>
