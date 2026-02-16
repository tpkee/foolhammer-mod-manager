import type { ProfileRequestDto, ProfileResponseDto } from '~/types/dto'
import { modRequestToResponse, modResponseToRequest } from './mods'

export function profileResponseToRequest(
  profile: ProfileResponseDto,
  gameId: string,
): ProfileRequestDto {
  if (!profile.name) {
    throw new Error('Profile name is required')
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

export function profileRequestToResponse(
  profile: ProfileRequestDto,
  overrides: Partial<ProfileResponseDto> = {},
): ProfileResponseDto {
  if (!profile.name) {
    throw new Error('Profile name is required')
  }

  return {
    name: profile.name,
    default: profile.default ?? false,
    manualMode: profile.manualMode ?? false,
    mods: overrides.mods ?? profile.mods?.map(mod => modRequestToResponse(mod)) ?? [],
  }
}
