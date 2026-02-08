<template>
  <label :for="uid">
    <span
      v-if="label" :class="{
        'sr-only': srOnlyLabel }"
      class="block"
    >
      {{ label }}
    </span>
    <input
      :id="uid"
      v-model="model"
      v-bind="$attrs"
      type="checkbox"
      class="size-3.5 appearance-none rounded-xs border border-gray-700 bg-gray-800 accent-purple-600 checked:appearance-auto"
    >
  </label>
</template>

<script lang="ts" setup>
// Props
defineProps<{
  label?: string
  srOnlyLabel?: boolean
}>()

// V-Model(s)
const model = defineModel()

// Non-reactive state
const uid = useId()
</script>
