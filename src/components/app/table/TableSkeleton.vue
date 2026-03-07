<template>
  <div class="space-y-2.5">
    <table-row
      v-for="i in rows"
      :key="i"
      :columns="columns"
    >
      <template
        v-for="col in columns"
        :key="col.key"
        #[col.key]
      >
        <div class="h-4 bg-gray-700 rounded animate-pulse" />
      </template>
    </table-row>
  </div>
</template>

<script setup lang="ts">
import type { AppTableColumn } from '~/types/common/AppTable'

const props = defineProps<{
  columns: AppTableColumn[]
  rows?: number
}>()

const rows = computed(() => props.rows ?? 5)
</script>
