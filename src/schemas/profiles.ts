import { z } from 'zod'
import { ModRequestSchema, ModResponseSchema } from './mods'

export const ProfileResponseSchema = z.object({
  id: z.string(),
  name: z.string(),
  mods: z.array(ModResponseSchema),
  manualMode: z.boolean(),
  groups: z.array(z.string()),
})

export const ProfileRequestSchema = z.object({
  gameId: z.string(),
  name: z.string(),
  default: z.boolean().nullish(),
  manualMode: z.boolean().nullish(),
  mods: z.array(ModRequestSchema),
})
