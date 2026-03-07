<template>
  <div class="border overflow-hidden rounded border-gray-700 overflow-x-auto md:overflow-x-hidden">
    <table-head :columns="columns" />
    <transition
      mode="out-in"
    >
      <slot v-if="!loading" :items="list" :columns="columns" />
      <table-skeleton v-else :columns="columns" :rows="8" />
    </transition>
  </div>
</template>

<script lang="ts" setup generic="T">
import type { AppTableColumn } from '~/types/common/AppTable'

defineProps<{
  columns: AppTableColumn[]
  list: T[]
  loading?: boolean
}>()
</script>

<style scoped>
.v-enter-active,
.v-leave-active {
  transition: opacity 0.25s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
