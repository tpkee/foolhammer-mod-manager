export interface ModRequestDto {
  name: string
  enabled: boolean
  order?: Nullable<number>
}

export type ModResponseDto = RecursivePartial<{
  name: string
  path: string | null
  enabled: boolean
  order: Nullable<number>
  canEnable: boolean
  lastUpdated: string | null
  fromSteamWorkshop: boolean
  image: string | null
}>
