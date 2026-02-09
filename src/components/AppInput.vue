<template>
  <div :class="attrs.class">
    <label :for="uid">
      <span
        v-if="label" :class="{
          'sr-only': srOnlyLabel }"
        class="block"
      >
        {{ label }}
      </span>
      <input :id="uid" v-bind="attrs" v-model="modelValue" class="px-1.5 py-1 bg-gray-800 appearance-none border  placeholder:italic placeholder:text-gray-500 border-gray-700 focus:outline-none focus-within:border-purple-600 hover:border-purple-600 transition-all transition-duration-300 rounded disabled:opacity-75">
    </label>
  </div>
</template>

<script lang="ts" setup>
defineOptions({
  inheritAttrs: false,
})

// Props
defineProps<{
  label?: string
  srOnlyLabel?: boolean
}>()

// V-Model(s)
const modelValue = defineModel()

// Non-reactive state
const uid = useId()
const attrs = useAttrs()
</script>
