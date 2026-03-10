import type { z } from 'zod'
import type { SettingsResponseSchema } from '~/schemas'

export type SettingsResponseDto = z.infer<typeof SettingsResponseSchema>
