import type { ProfileRequestDto, ProfileResponseDto } from '~/types/dto'
import { modRequestToResponse, modResponseToRequest } from './mods'

export function profileResponseToRequest(
  profile: ProfileResponseDto,
  gameId: string,
): ProfileRequestDto {
  return {
    gameId,
    name: profile.name,
    default: profile.default,
    manualMode: profile.manualMode,
    mods: profile.mods.map(modResponseToRequest),
  }
}

export function profileRequestToResponse(
  profile: ProfileRequestDto,
  overrides: Partial<ProfileResponseDto> = {},
): ProfileResponseDto {
  return {
    name: profile.name,
    default: profile.default ?? false,
    manualMode: profile.manualMode ?? false,
    mods: overrides.mods ?? profile.mods.map(mod => modRequestToResponse(mod)),
  }
}
