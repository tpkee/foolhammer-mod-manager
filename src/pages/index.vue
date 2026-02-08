<template>
  <div>
    <h1 class="text-3xl font-bold underline">
      {{ t('hello_world') }}
    </h1>
    <button @click="getState">
      Get State
    </button>

    <button @click="getMods">
      Get Mods list
    </button>

    <list-mods />
  </div>
</template>

<script setup lang="ts">
// When using the Tauri API npm package:
const { t } = useI18n()

function getState() {
  useTauriInvoke('get_state').then((state) => {
    console.log(state)
  })
}

function getMods() {
  useTauriInvoke('get_mods').then((mods) => {
    console.log(mods)
  })
}
</script>
