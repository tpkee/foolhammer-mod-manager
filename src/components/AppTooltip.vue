<template>
  <div class="group/tooltip relative inline-block">
    <div v-if="isEnabled" class="absolute bottom-6 hidden group-hover/tooltip:block group-focus/tooltip:block w-fit p-2 bg-gray-800 rounded z-50 border border-gray-700 shadow z-[9999]">
      <slot name="content" />
    </div>

    <div
      :class="{
        'underline cursor-pointer': isEnabled,
      }"
    >
      <slot />
    </div>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  disable?: boolean
}>()

const isEnabled = computed(() => !props.disable)
</script>
