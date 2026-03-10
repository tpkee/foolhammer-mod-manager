import type { ProfileRequestDto, ProfileResponseDto } from '~/types/dto'
import { ProfileRequestSchema } from '../index'
import { modResponseToRequest } from './mods'

export function profileResponseToRequest(
  profile: ProfileResponseDto,
  gameId: string,
): ProfileRequestDto {
  return ProfileRequestSchema.parse({
    gameId,
    name: profile.name,
    manualMode: profile.manualMode,
    mods: profile.mods.map(modResponseToRequest),
  })
}
