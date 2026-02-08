interface GameProfile {
  id: string
  profileName: string
  mods: ModMeta[]
}

interface GameMeta {
  gameId: string
  name: string
  profiles: GameProfile[]
  mods: ModMeta[]
  defaultProfile: Nullable<string>
  installationPath: Nullable<string>
  steamWorkshopPath: Nullable<string>
  executableName: Nullable<string>
  modsPath: Nullable<string>
  savesPath: Nullable<string>
}
