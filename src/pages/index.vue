<template>
  <div>
    <h1 class="text-3xl font-bold underline">
      {{ t('hello_world') }}
    </h1>
    <button @click="getState">
      Get State
    </button>
    <list-mods :list="data" :loading="pending" />
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
const selectedGame = ref('')

function getState() {
  useTauriInvoke('get_state').then((state) => {
    // @ts-expect-error type this, someday
    selectedGame.value = state.gameId ?? null
  })
}

const getId = computed(() => {
  return `${selectedGame.value}-mods`
})

const { data, pending } = useAsyncData(getId, () => {
  return useTauriInvoke('get_mods')
})
</script>
