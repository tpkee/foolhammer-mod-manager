import { z } from 'zod'

export const GroupResponseSchema = z.object({
  id: z.string(),
  name: z.string(),
  mods: z.array(z.string()),
})

export const GroupRequestSchema = z.object({
  id: z.string().nullish(),
  gameId: z.string(),
  name: z.string(),
  mods: z.array(z.string()),
})
