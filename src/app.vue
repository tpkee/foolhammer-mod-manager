<template>
  <main class="p-2.5">
    <NuxtRouteAnnouncer />
    <NuxtPage />
  </main>
</template>

<script setup lang="ts">
import { useSettingsStore } from './stores/settings'

const settingsStore = useSettingsStore()
useTauriListener('update/user-settings', (event) => {
  console.log('Received app state update:', event.payload)
})

// Functions
function updateUserSettings(newSettings: UserSettings) {
}
</script>
