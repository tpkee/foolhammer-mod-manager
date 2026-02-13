<template>
  <div class="min-h-screen p-2.5 border-l border-gray-800 space-y-2.5 flex flex-col justify-between items-center">
    <div class="flex flex-col gap-1 grow">
      <sidebar-game v-for="(item, index) of games" :id="item" :key="index" :current-game="currentGame" />
    </div>

    <div>
      <sidebar-button v-if="currentGame" label="play" tooltip="Start the game" @click="playGame">
        <nuxt-icon name="mi:play" class="size-10" />
      </sidebar-button>
    </div>

    <div class="grid gap-2.5">
      <sidebar-profile
        v-if="currentGame && currentGameData"
        :game-id="currentGame"
        :profiles="currentGameData.profiles ?? []"
        @refresh="emit('refreshGame')"
      />
      <sidebar-button v-if="currentGame" label="settings" tooltip="Settings" class="block">
        <nuxt-icon name="mi:settings" class="size-10" />
      </sidebar-button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { GameResponseDto } from '~/types/dto'

const props = defineProps<{
  games: string[]
  currentGame: Nullable<string>
  currentGameData: Nullable<GameResponseDto>
}>()

const emit = defineEmits<{
  refreshGame: []
}>()

async function playGame() {
  console.warn('TODO: implement playGame function')
  // await useTauriInvoke('start_game', { gameId: props.currentGame })
}
</script>
