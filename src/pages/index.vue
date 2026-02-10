<template>
  <div>
    <list-mods :list="listAvailableMods" :loading="statusAvailableMods" />
  </div>
</template>

<script setup lang="ts">
const settingsStore = useSettingsStore()
const { t } = useI18n()

// Fetching
const { data: listAvailableMods, pending: statusAvailableMods } = await useAsyncData(`${settingsStore.gameId}-mods`, () => useTauriInvoke('get_mods'), {
  default: () => [],
  watch: [() => settingsStore.gameId],
})
</script>
