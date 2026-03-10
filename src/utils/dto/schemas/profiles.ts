import { z } from 'zod'
import { ModRequestSchema, ModResponseSchema } from './mods'

export const ProfileResponseSchema = z.object({
  id: z.string().optional(),
  name: z.string().optional(),
  mods: z.array(ModResponseSchema).optional(),
  default: z.boolean().optional(),
  manualMode: z.boolean().optional(),
  groups: z.array(z.string()).optional(),
})

export const ProfileRequestSchema = z.object({
  gameId: z.string(),
  name: z.string(),
  default: z.boolean().nullish(),
  manualMode: z.boolean().nullish(),
  mods: z.array(ModRequestSchema),
})
