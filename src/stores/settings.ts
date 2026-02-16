import type { SettingsResponseDto } from '~/types/dto/settings'

interface SettingsStore {
  settings: Nullable<SettingsResponseDto>
}

export const useSettingsStore = defineStore('settingsStore', {
  state: (): SettingsStore => ({
    settings: null,
  }),
  actions: {
    setSettings(newSettings: Nullable<SettingsResponseDto>) {
      this.settings = newSettings
    },
  },
})
