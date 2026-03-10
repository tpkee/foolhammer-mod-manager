import { z } from 'zod'

export const ModResponseSchema = z.object({
  name: z.string(),
  path: z.string().nullish(),
  enabled: z.boolean(),
  groups: z.array(z.string()).nullish(),
  order: z.number().nullish(),
  canEnable: z.boolean(),
  lastUpdated: z.string().nullish(),
  fromSteamWorkshop: z.boolean().nullish(),
  image: z.string().nullish(),
  dependencies: z.array(z.tuple([z.boolean(), z.string()])),
})

export const ModRequestSchema = z.object({
  name: z.string(),
  enabled: z.boolean(),
  groups: z.array(z.string()).nullish(),
  order: z.number().nullish(),
})
