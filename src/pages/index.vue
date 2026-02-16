<template>
  <div class="space-y-2.5">
    <list-mods v-if="gameStore.selectedGame" :list="gameStore.getProfileMods" :profile="gameStore.getProfile" :game-id="gameStore.selectedGame" @refresh="refresh" />
  </div>
</template>

<script setup lang="ts">
import type { GameResponseDto } from '~/types/dto'

const gameStore = useGameStore()

const { data, refresh } = await useAsyncData<Nullable<GameResponseDto>>(gameStore.getDataKey, () => {
  if (!gameStore.selectedGame)
    return Promise.resolve(null)

  return useTauriInvoke('get_game', { gameId: gameStore.selectedGame })
}, {
  default: () => null,
  watch: [() => gameStore.selectedGame],
})

watch(data, () => {
  gameStore.setGame(data.value)
}, { immediate: true })
</script>
