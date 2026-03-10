import { z } from 'zod'

export const PackResponseSchema = z.object({
  name: z.string(),
  path: z.string(),
  image: z.string().nullable(),
  lastUpdated: z.string().nullable(),
  fromSteamWorkshop: z.boolean(),
})
