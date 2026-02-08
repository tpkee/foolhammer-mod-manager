<template>
  <div ref="dropdownRef">
    <button class="cursor-pointer" @click="toggleDropdown">
      <span class="sr-only">
        Toggle dropdown
      </span>

      <nuxt-icon name="mi:options-vertical" class="size-6" />
    </button>

    <transition name="fade" mode="out-in">
      <div v-if="isOpen" class="absolute right-0 mt-2 w-48 bg-gray-800 rounded shadow-lg z-50">
        <button
          v-for="(option, index) in options" :key="index" :icon="option.icon"
          class="p-2 text-sm hover:bg-gray-700 flex items-center gap-2 w-full text-left cursor-pointer"
          @click="getCallback(option.callback)"
        >
          <nuxt-icon v-if="option.icon" :name="option.icon" class="size-4" />
          {{ option.label }}
        </button>
      </div>
    </transition>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from 'vue'

// Props
defineProps<{
  options: Array<{
    icon?: string
    label: string
    callback?: () => void
  }>
}>()

// Reactive state
const isOpen = ref(false)
const dropdownRef = ref<HTMLElement | null>(null)

// Functions
function toggleDropdown() {
  isOpen.value = !isOpen.value
}

function getCallback(callback: (() => void) | undefined) {
  if (callback) {
    callback()
  }
  isOpen.value = false
  return () => {}
}

function handleClickOutside(event: MouseEvent) {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    isOpen.value = false
  }
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.25s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
