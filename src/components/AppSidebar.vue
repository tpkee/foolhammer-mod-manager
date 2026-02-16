<template>
  <div class="min-h-screen p-2.5 border-l border-gray-800 space-y-2.5 flex flex-col justify-between items-center">
    <div class="flex flex-col gap-1 grow">
      <sidebar-game v-for="(item, index) of games" :id="item" :key="index" :current-game="gameStore.selectedGame" />
    </div>

    <div class="flex flex-col gap-1">
      <sidebar-play
        v-if="gameStore.selectedGame && gameStore.selectedProfile"
        :current-game="gameStore.selectedGame"
        :profile-name="gameStore.selectedProfile"
      />

      <!-- <sidebar-button v-if="currentGame" label="Load from last save" tooltip="Load from last save" @click="playGame">
        <nuxt-icon name="mi:next" class="size-10" />
      </sidebar-button> -->
    </div>

    <div class="grid gap-2.5">
      <sidebar-profile
        v-if="gameStore.selectedGame"
        :current-profile="gameStore.getActiveProfile"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  games: string[]
}>()

const gameStore = useGameStore()
</script>
