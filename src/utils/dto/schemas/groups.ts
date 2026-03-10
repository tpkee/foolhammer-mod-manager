import { z } from 'zod'

export const GroupResponseSchema = z.object({
  id: z.string().optional(),
  name: z.string().optional(),
  mods: z.array(z.string()).optional(),
})

export const GroupRequestSchema = z.object({
  id: z.string().nullish(),
  gameId: z.string(),
  name: z.string(),
  mods: z.array(z.string()),
})
