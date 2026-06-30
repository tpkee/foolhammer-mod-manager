import { z } from 'zod'

export const SettingsResponseSchema = z.object({
  defaultGame: z.string().optional(),
  steamPath: z.string().nullable().optional(),
  steamLibraryPath: z.string().nullable().optional(),
  invertModNames: z.boolean().optional(),
})
