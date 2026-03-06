<template>
  <div
    class="grid p-2.5 items-center gap-2.5 text-left w-[400%] sm:w-[125%] md:w-full"
    :style="gridStyle"
  >
    <div
      v-for="col in columns"
      :key="col.key"
      :class="col.cellClass"
      :style="spanStyle(col.span)"
    >
      <slot :name="col.key" />
    </div>
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
