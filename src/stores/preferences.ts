interface PreferencesStore {
  userSettings: Nullable<RecursivePartial<UserSettings>>
  currentGame: Nullable<string>
  currentProfile: Nullable<string>
}

export const usePreferencesStore = defineStore('preferencesStore', {
  state: (): PreferencesStore => ({
    userSettings: null,
    currentGame: null,
    currentProfile: null,
  }),
  actions: {
    setSettings(newSettings: Nullable<RecursivePartial<UserSettings>>) {
      this.userSettings = newSettings
    },
    setCurrentGame(game: Nullable<string>) {
      this.currentGame = game
    },
    setCurrentProfile(profile: Nullable<string>) {
      this.currentProfile = profile
    },
  },
})
