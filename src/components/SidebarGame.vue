<template>
  <sidebar-button
    :label="`games.${id}`"
    :tooltip="t(`games.${id}`)"
    :is-active="currentGame === id"
    @click="switchGame()"
  >
    <img
      v-if="useGameImage(id)"
      :src="useGameImage(id)!"
      :alt="t(`games.${id}`)"
      class="size-10"
    >

    <template #menu="{ close }">
      <item-option
        class="px-4 py-2"
        @click="openGameSettings(); close()"
      >
        Open game settings
      </item-option>
      <item-option
        v-if="settingsStore.settings?.defaultGame !== id"
        class="px-4 py-2"
        @click="setDefaultGame(); close()"
      >
        Set as default
      </item-option>
    </template>
  </sidebar-button>

  <modal-game
    ref="gameSettingsModal"
    :game-id="id"
  />
</template>

<script lang="ts" setup>
import { useGameImage } from '~/composables/gameImage'

// Props
const props = defineProps<{
  id: string
  currentGame: Nullable<string>
}>()

// Template refs
const gameSettingsModal = useTemplateRef('gameSettingsModal')

// Stores
const gameStore = useGameStore()
const settingsStore = useSettingsStore()

// composables
const { t } = useI18n()

// Functions
function switchGame() {
  gameStore.setGameId(props.id)
}

function openGameSettings() {
  gameSettingsModal.value?.open()
}

async function setDefaultGame() {
  await useTauriInvoke('set_default_game', { gameId: props.id })
}
</script>
