import { z } from 'zod'

export const ModResponseSchema = z.object({
  name: z.string().optional(),
  path: z.string().nullable().optional(),
  enabled: z.boolean().optional(),
  groups: z.array(z.string()).nullish(),
  order: z.number().nullish(),
  canEnable: z.boolean().optional(),
  lastUpdated: z.string().nullable().optional(),
  fromSteamWorkshop: z.boolean().optional(),
  image: z.string().nullable().optional(),
  dependencies: z.array(z.tuple([z.boolean(), z.string()])).optional(),
})

export const ModRequestSchema = z.object({
  name: z.string(),
  enabled: z.boolean(),
  groups: z.array(z.string()).nullish(),
  order: z.number().nullish(),
})
