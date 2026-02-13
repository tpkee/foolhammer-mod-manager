<template>
  <div class="space-y-2.5">
    <list-mods :list="getMods" :profile="getCurrentProfile" @refresh="refreshGame()" />
  </div>
</template>

<script setup lang="ts">
const { getCurrentProfile, refreshGame, getMods } = useCurrentGame()
</script>
