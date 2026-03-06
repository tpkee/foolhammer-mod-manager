<template>
  <div
    class="grid p-2.5 border border-gray-800 items-center gap-2.5 text-left bg-gray-800 w-[400%] sm:w-[125%] md:w-full"
    :style="gridStyle"
  >
    <span
      v-for="col in columns"
      :key="col.key"
      :class="col.headerClass"
      :style="spanStyle(col.span)"
    >
      {{ col.label }}
    </span>
  </div>
</template>

<script lang="ts" setup>
import type { AppTableColumn } from '~/types/common/AppTable'

const props = defineProps<{ columns: AppTableColumn[] }>()

const totalCols = computed(() => props.columns.reduce((acc, col) => acc + (col.span ?? 1), 0))
const gridStyle = computed(() => ({ gridTemplateColumns: `repeat(${totalCols.value}, minmax(0, 1fr))` }))

function spanStyle(span = 1) {
  return { gridColumn: `span ${span} / span ${span}` }
}
</script>
