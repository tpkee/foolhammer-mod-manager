<template>
  <label :for="uid" class="flex items-center gap-1.5 cursor-pointer" :class="attrs.class as any">
    <input
      :id="uid"
      v-model="modelValue"
      :checked="modelValue"
      type="radio"
      v-bind="attrs"
      class="size-4 appearance-none rounded-full border border-gray-700 bg-gray-800 accent-purple-600 checked:appearance-auto cursor-pointer"
    >
    <span
      v-if="label" :class="{
        'sr-only': srOnlyLabel,
      }"
    >
      {{ label }}
    </span>
  </label>
</template>

<script lang="ts" setup>
defineOptions({
  inheritAttrs: false,
})

defineProps<{
  label?: string
  srOnlyLabel?: boolean
}>()

const modelValue = defineModel<boolean>()

const uid = useId()
const attrs = useAttrs()
</script>
