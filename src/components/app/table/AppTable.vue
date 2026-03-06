<template>
  <div class="border overflow-hidden rounded border-gray-700 overflow-x-auto md:overflow-x-hidden">
    <table-head :columns="columns" />
    <slot :items="list" :columns="columns" />
  </div>
</template>

<script lang="ts" setup generic="T">
import type { AppTableColumn } from '~/types/common/AppTable'

defineProps<{
  columns: AppTableColumn[]
  list: T[]
}>()
</script>
