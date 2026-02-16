<template>
  <div class="space-y-2.5">
    <list-mods v-if="gameStore.selectedGame" :list="gameStore.getProfileMods" :profile="gameStore.getProfile" :game-id="gameStore.selectedGame" @refresh="refresh" />
  </div>
</template>

<script setup lang="ts">
const gameStore = useGameStore()
const refresh = inject('refreshGame') as () => void
</script>
