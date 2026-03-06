<template>
  <button
    class="px-1.5 py-1 rounded transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed h-fit"
    :class="variantClasses"
  >
    <slot />
  </button>
</template>

<script lang="ts" setup>
type ButtonVariant = 'primary' | 'secondary' | 'tertiary'

const props = withDefaults(
  defineProps<{
    variant?: ButtonVariant
  }>(),
  {
    variant: 'primary',
  },
)

const variantClasses = computed(() => {
  const variants: Record<ButtonVariant, string> = {
    primary: 'border border-purple-600 bg-purple-700 hover:bg-purple-900/90',
    secondary: 'border border-gray-600 bg-gray-700 hover:bg-gray-600',
    tertiary: 'border border-red-600 bg-red-700 hover:bg-red-600',
  }
  return variants[props.variant]
})
</script>
