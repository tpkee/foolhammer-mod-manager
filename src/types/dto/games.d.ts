import type { z } from 'zod'
import type { GameResponseSchema } from '~/schemas'

export type GameResponseDto = z.infer<typeof GameResponseSchema>
