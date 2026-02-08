<template>
  <label :for="uid">
    <span
      v-if="label" :class="{
        'sr-only': srOnlyLabel }"
      class="block mb-0.5"
    >
      {{ label }}
    </span>
    <select :id="uid" v-model="model" class="px-1.5 py-1 bg-gray-800 border border-gray-700 appearance-none focus:outline-none hover:border-purple-600 transition-all transition-duration-300 rounded disabled:opacity-75" v-bind="$attrs">
      <option v-for="option in options" :key="option.value" :value="option.value" :disabled="option.disabled">
        {{ option.label }}
      </option>
    </select>
  </label>
</template>

<script lang="ts" setup>
// Props
defineProps<{
  label?: string
  srOnlyLabel?: boolean
  options: Array<{
    value: string | number
    label: string
    disabled?: boolean
  }>
}>()

// V-Model(s)
const model = defineModel()

// Non-reactive state
const uid = useId()
</script>
