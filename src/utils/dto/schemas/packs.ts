import { z } from 'zod'

export const PackResponseSchema = z.object({
  name: z.string(),
  path: z.string().optional(),
  image: z.string().nullable().optional(),
  lastUpdated: z.string().nullable().optional(),
  fromSteamWorkshop: z.boolean().optional(),
})
