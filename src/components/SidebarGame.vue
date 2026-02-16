<template>
  <sidebar-button
    :label="`games.${id}`"
    :tooltip="t(`games.${id}`)"
    :is-active="currentGame === id"
    @click="switchGame()"
  >
    <img v-if="useGameImage(id)" :src="useGameImage(id)!" :alt="t(`games.${id}`)" class="size-10">

    <template #menu="{ close }">
      <item-option
        class="px-4 py-2"
        @click="openGameSettings(); close()"
      >
        Open game settings
      </item-option>
      <item-option
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
    @save="emit('onRefresh')"
  />
</template>

<script lang="ts" setup>
const props = defineProps<{
  id: string
  currentGame: Nullable<string>
}>()
const emit = defineEmits<{
  onRefresh: []
}>()
const gameStore = useGameStore()
const { t } = useI18n()
const gameSettingsModal = ref()

function switchGame() {
  gameStore.setActiveGame(props.id)
}

function openGameSettings() {
  gameSettingsModal.value?.open()
}

function setDefaultGame() { // TODO: make a call to set this as default
  console.warn('ding dong it\'s a todo')
}
</script>
