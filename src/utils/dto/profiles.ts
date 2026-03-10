import type { ProfileRequestDto, ProfileResponseDto } from '~/types/dto'
import { modResponseToRequest } from './mods'
import { ProfileRequestSchema } from './schemas'

export function profileResponseToRequest(
  profile: ProfileResponseDto,
  gameId: string,
): ProfileRequestDto {
  const mods = (profile.mods ?? [])
    .filter(m => m != null)
    .map(modResponseToRequest)

  return ProfileRequestSchema.parse({
    gameId,
    name: profile.name,
    default: profile.default,
    manualMode: profile.manualMode,
    mods,
  })
}
