export const useSettingsStore = defineStore('useSettingsStore', {
  state: (): RecursivePartial<UserSettings> => ({}),
  actions: {
    setSettings(newSettings: UserSettings) {
      this.$state = newSettings
    },
  },
})
