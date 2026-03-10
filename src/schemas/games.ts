import { z } from 'zod'
import { GroupResponseSchema } from './groups'
import { PackResponseSchema } from './packs'
import { ProfileResponseSchema } from './profiles'
import { SaveResponseSchema } from './saves'

export const GameResponseSchema = z.object({
  mods: z.array(PackResponseSchema).default([]),
  profiles: z.array(ProfileResponseSchema).default([]),
  groups: z.array(GroupResponseSchema).default([]),
  saves: z.array(SaveResponseSchema).default([]),
  defaultProfile: z.string().nullish(),
  gameId: z.string(),
  gamePath: z.string().nullish(),
  savesPath: z.string().nullish(),
  modsPath: z.string().nullish(),
  workshopPath: z.string().nullish(),
})
