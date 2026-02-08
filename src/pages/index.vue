<template>
  <div>
    <h1 class="text-3xl font-bold underline">
      {{ t('hello_world') }}
    </h1>
    <button @click="getState">
      Get State
    </button>

    <list-mods />
  </div>
</template>

<script setup lang="ts">
// When using the Tauri API npm package:
const { t } = useI18n()

const invoke = useInvoke()

function getState() {
  invoke('get_state').then((state) => {
    console.log(state)
  })
}
</script>
