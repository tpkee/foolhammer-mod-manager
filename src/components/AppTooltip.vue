<template>
  <div class="group relative inline-block">
    <div v-if="isEnabled" class="absolute bottom-6 hidden group-focus:block group-hover:block w-fit p-2 bg-gray-800 rounded z-50">
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
