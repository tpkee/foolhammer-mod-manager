import { z } from 'zod'

export const SaveResponseSchema = z.object({
  name: z.string().optional().default(''),
  path: z.string().optional().default(''),
  lastUpdated: z.string().nullish().default(null),
  lastAccessed: z.string().nullish().default(null),
})
