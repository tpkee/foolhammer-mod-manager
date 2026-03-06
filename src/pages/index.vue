<template>
  <div class="space-y-2.5">
    <table-mods v-if="gameStore.selectedGame" :list="gameStore.getProfileMods" :profile="gameStore.getProfile" :game-id="gameStore.selectedGame" @refresh="gameStore.fetchGame" />
  </div>
</template>

<script setup lang="ts">
const gameStore = useGameStore()
</script>
