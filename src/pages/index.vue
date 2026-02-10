<template>
  <div>
    <list-mods :list="listAvailableMods" :loading="statusAvailableMods" />
  </div>
</template>

<script setup lang="ts">
const preferencesStore = usePreferencesStore()
const { t } = useI18n()

// Fetching
const { data: listAvailableMods, pending: statusAvailableMods } = await useAsyncData(`${preferencesStore.currentGame}-mods`, () => useTauriInvoke('get_mods'), {
  default: () => [],
  watch: [() => preferencesStore.currentGame],
})
</script>
