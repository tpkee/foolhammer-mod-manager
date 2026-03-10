import { z } from 'zod'

export const SaveResponseSchema = z.object({
  name: z.string(),
  path: z.string(),
  lastUpdated: z.string().nullish(),
  lastAccessed: z.string().nullish(),
})
