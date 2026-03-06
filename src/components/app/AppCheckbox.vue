<template>
  <label :for="uid" class="flex items-center gap-1.5 cursor-pointer" :class="attrs.class as any">

    <input
      :id="uid"
      v-model="modelValue"
      v-bind="attrs"
      type="checkbox"
      class="size-3.5 appearance-none rounded-xs border border-gray-700 bg-gray-800 accent-purple-600 checked:appearance-auto"
    >

    <span
      v-if="label" :class="{
        'sr-only': srOnlyLabel }"
    >
      {{ label }}
    </span>
  </label>
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
