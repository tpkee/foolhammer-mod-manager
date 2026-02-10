<template>
  <div class="min-h-screen p-2.5 border-l border-gray-800 space-y-2.5 flex flex-col justify-between items-center">
    <h2>FHMM</h2>

    <div class="flex flex-col gap-1 grow">
      <sidebar-game v-for="(item, index) of games" :id="item" :key="index" :current-game="currentGame" />
    </div>

    <!-- <div>
      <nuxt-link-locale class="cursor-pointer" to="/settings">
        <nuxt-icon name="mi:settings" class="size-6" />
      </nuxt-link-locale>
    </div> -->
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  games: string[]
  currentGame: Nullable<string>
}>()
</script>
