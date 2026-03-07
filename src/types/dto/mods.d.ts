export interface ModRequestDto {
  name: string
  enabled: boolean
  groups?: Nullable<string[]>
  order?: Nullable<number>
}

export type ModResponseDto = RecursivePartial<{
  name: string
  path: string | null
  enabled: boolean
  groups: Nullable<string[]>
  order: Nullable<number>
  canEnable: boolean
  lastUpdated: string | null
  fromSteamWorkshop: boolean
  image: string | null
  dependencies: [boolean, string][] // [loadBefore, modName]
}>
