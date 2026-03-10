import { z } from 'zod'
import { GroupResponseSchema } from './groups'
import { PackResponseSchema } from './packs'
import { ProfileResponseSchema } from './profiles'
import { SaveResponseSchema } from './saves'

export const GameResponseSchema = z.object({
  mods: z.array(PackResponseSchema).default([]),
  profiles: z.array(ProfileResponseSchema).optional().default([]),
  groups: z.array(GroupResponseSchema).optional().default([]),
  saves: z.array(SaveResponseSchema).optional().default([]),
  defaultProfile: z.string().nullish(),
  gameId: z.string().optional(),
  gamePath: z.string().optional(),
  savesPath: z.string().nullish(),
  modsPath: z.string().optional(),
  workshopPath: z.string().nullish(),
})
