export type PackResponseDto = RecursivePartial<{
  name: string
  path: string
  image: string | null
  lastUpdated: string | null
  fromSteamWorkshop: boolean
}>
