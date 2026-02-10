<template>
  <button
    class="w-full text-left hover:bg-gray-700 cursor-pointer flex items-center gap-2"
    @click="emit('click')"
  >
    <nuxt-icon v-if="icon" :name="icon" class="size-4" />
    <slot>{{ label }}</slot>
  </button>
</template>

<script lang="ts" setup>
defineProps<{
  icon?: string
  label?: string
}>()

const emit = defineEmits<{
  click: []
}>()
</script>
