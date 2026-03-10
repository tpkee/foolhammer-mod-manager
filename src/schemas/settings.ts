import { z } from 'zod'

export const SettingsResponseSchema = z.object({
  defaultGame: z.string(),
})
