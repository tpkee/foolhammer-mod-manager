<template>
  <div class="min-h-screen p-2.5 border-l border-gray-800 space-y-2.5 flex flex-col justify-between items-center">
    <div class="flex flex-col gap-1 grow">
      <sidebar-game v-for="(item, index) of games" :id="item" :key="index" :current-game="gameStore.selectedGame" />
    </div>

    <div v-if="gameStore.selectedGame && gameStore.getProfile" class="flex flex-col gap-1">
      <sidebar-play
        :current-game="gameStore.selectedGame"
        :profile="gameStore.getProfile"
      />
      <sidebar-play
        v-if="gameStore.getSaves.length"
        variant="continue"
        :current-game="gameStore.selectedGame"
        :profile="gameStore.getProfile"
        :saves="gameStore.getSaves"
      />
    </div>

    <!--
    <div class="grid gap-2.5">
      TODO: add settings and shit
    </div>
    -->
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  games: string[]
}>()

const gameStore = useGameStore()
</script>
