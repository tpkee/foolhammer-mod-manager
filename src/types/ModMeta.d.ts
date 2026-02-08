interface ModMeta {
  path: string
  name: Nullable<string>
  pack: string // this is basically our uid within the profile or the game. It's like a (gameid/profileid, path) kind of thing
  isEnabled: boolean
  canEnable: boolean
  lastUpdated: number | string | Date
  image: Nullable<string>
}
