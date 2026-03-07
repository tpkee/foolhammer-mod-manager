<template>
  <button
    class="w-full text-left hover:bg-gray-700 cursor-pointer flex items-center gap-2"
    :disabled="loading || disabled"
    :class="{ 'opacity-50 cursor-not-allowed': disabled && !loading }"
    @click="handleClick"
  >
    <app-spinner v-if="loading" class="size-4 animate-spin" />
    <nuxt-icon v-else-if="icon" :name="icon" class="size-4" />
    <slot>{{ label }}</slot>
  </button>
</template>

<script lang="ts" setup>
const props = defineProps<{
  icon?: string
  label?: string
  callback?: () => void | Promise<void>
  disabled?: boolean
}>()

const emit = defineEmits<{
  click: []
}>()

const loading = ref(false)

async function handleClick() {
  emit('click')
  const result = props.callback?.()
  if (result instanceof Promise) {
    loading.value = true
    try {
      await result
    }
    finally {
      loading.value = false
    }
  }
}
</script>
