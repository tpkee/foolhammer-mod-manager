<template>
  <app-tooltip>
    <template #content>
      {{ t(`games.${id}`) }}
    </template>

    <app-dropdown>
      <template #trigger="{ toggle }">
        <button
          class="cursor-pointer bg-gray-800 border border-gray-700 p-1 rounded hover:border-purple-600 hover:bg-purple-900 transition-all duration-100" :class="{
            'border-purple-600 bg-purple-900 hover:bg-purple-900/90!': currentGame === id,
          }"

          @click.right.prevent="toggle"
          @click="switchGame()"
        >
          <img v-if="useGameImage(id)" :src="useGameImage(id)!" :alt="t(`games.${id}`)" class="size-10 ">
          <span class="sr-only">Switch to {{ t(`games.${id}`) }}</span>
        </button>
      </template>

      <template #default="{ close }">
        <item-option
          class="px-4 py-2"
          @click="openGameSettings(); close()"
        >
          Open game settings
        </item-option>
      </template>
    </app-dropdown>
  </app-tooltip>

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
const preferencesStore = usePreferencesStore()
const { t } = useI18n()
const gameSettingsModal = ref()

function switchGame() {
  preferencesStore.setCurrentGame(props.id)
}

function openGameSettings() {
  gameSettingsModal.value?.open()
}
</script>
