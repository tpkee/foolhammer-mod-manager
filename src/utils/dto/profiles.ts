import type { ProfileRequestDto, ProfileResponseDto } from '~/types/dto'
import { modResponseToRequest } from './mods'

export function profileResponseToRequest(
  profile: ProfileResponseDto,
  gameId: string,
): ProfileRequestDto {
  if (!profile.id || !profile.name) {
    throw new Error('Profile ID and name are required')
  }

  const modsRequest = (profile?.mods ?? [])
    .filter(m => m != null)
    .map(modResponseToRequest)

  return {
    gameId,
    name: profile.name,
    default: profile.default,
    manualMode: profile.manualMode,
    mods: modsRequest,
  }
}
