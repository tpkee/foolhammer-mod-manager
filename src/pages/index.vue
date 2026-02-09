<template>
  <div>
    <h1 class="text-3xl font-bold underline">
      {{ t('hello_world') }} {{ userSettings?.gameId }}
    </h1>
    <list-mods :list="listFetchedMods" :loading="pending" />
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()

// Fetching
const { data: userSettings } = await useAsyncData('user-settings', () => useTauriInvoke('get_state')) // todo: rework
const { data: listFetchedMods, pending, refresh } = await useAsyncData(`${userSettings.value!.gameId}-mods`, () => useTauriInvoke('get_mods'), {
  immediate: false,
  default: () => [],
})

// Watchers
watch(userSettings, (newVal, oldVal) => {
  if (newVal?.gameId !== oldVal?.gameId) {
    refresh()
  }
}, { immediate: true })
</script>
