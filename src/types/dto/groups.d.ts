export type GroupResponseDto = RecursivePartial<{
  id: string
  name: string
  mods: string[]
}>

export interface GroupRequestDto {
  id?: string | null
  gameId: string
  name: string
  mods: string[]
}
