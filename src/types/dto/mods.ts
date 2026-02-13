export interface ModRequestDto {
  name: string
  enabled: boolean
  order: number
}

export interface ModResponseDto {
  name: string
  path: string | null
  enabled: boolean
  order: number
  canEnable: boolean
  lastUpdated: string | null
  fromSteamWorkshop: boolean
  image: string | null
}
