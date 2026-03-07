<template>
  <div class="space-y-2.5">
    <div
      v-if="gameStore.selectedGame"
      class="shrink-0 flex items-center justify-between gap-2.5 border border-gray-700 rounded p-2.5"
    >
      <div class="grid grid-cols-2 gap-6">
        <div>
          <p class="text-xs text-gray-400 uppercase tracking-wide">
            Game
          </p>
          <p class="font-medium">
            {{ getGameLabel }}
          </p>
        </div>
        <div>
          <p class="text-xs text-gray-400 uppercase tracking-wide">
            Profile
          </p>
          <p class="font-medium">
            {{ getProfileLabel }}
          </p>
        </div>
      </div>

      <div>
        <app-button @click="navigateTo('/profiles')">
          Manage Profiles
        </app-button>
      </div>
    </div>

    <table-mods v-if="gameStore.selectedGame" :loading="gameStore.fetchStatus === 'pending'" :list="gameStore.getProfileMods" :profile="gameStore.getProfile" :game-id="gameStore.selectedGame" @refresh="gameStore.fetchGame" />
  </div>
</template>

<script setup lang="ts">
const gameStore = useGameStore()
const { t } = useI18n()

const getGameLabel = computed(() => gameStore.selectedGame ? t(`games.${gameStore.selectedGame}`) : 'No game selected')
const getProfileLabel = computed(() => gameStore.getProfile?.name ?? 'No profile selected')
</script>
