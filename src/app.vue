<template>
  <div>
    <NuxtRouteAnnouncer />
    <h1 class="text-3xl font-bold underline">
      {{ t('hello_world') }}
    </h1>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
</script>
