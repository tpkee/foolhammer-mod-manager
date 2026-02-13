<template>
  <div class="space-y-2.5">
    <list-mods :list="getMods" :profile="getProfile" @refresh="refresh()" />
  </div>
</template>

<script setup lang="ts">
import type { GameResponseDto } from '~/types/dto'

const preferencesStore = usePreferencesStore()
const { t } = useI18n()

// Fetching
const { data: selectedGame, refresh } = await useAsyncData<GameResponseDto>(`${preferencesStore.currentGame}-game`, () => useTauriInvoke('get_game', { gameId: preferencesStore.currentGame }), {
  watch: [() => preferencesStore.currentGame],
})

const getProfile = computed(() => {
  if (!selectedGame.value || selectedGame.value.profiles?.length === 0)
    return null

  const first = selectedGame.value.profiles[0]

  return selectedGame.value.profiles.find(p => p.default) ?? first
})

const getMods = computed(() => {
  if (!getProfile.value)
    return []

  return getProfile.value.mods
})
</script>
