<template>
  <app-tooltip :disabled="!tooltip">
    <template #content>
      {{ tooltip }}
    </template>

    <app-dropdown :disabled="!hasMenuItems">
      <template #trigger="{ toggle }">
        <button
          class="cursor-pointer bg-gray-800 border border-gray-700 p-1 rounded hover:border-purple-600 hover:bg-purple-900 transition-all duration-100"
          :class="{
            'border-purple-600 bg-purple-900 hover:bg-purple-900/90!': isActive,
          }"
          @click.right.prevent="toggle"
          @click="handleClick"
        >
          <slot />
          <span class="sr-only">{{ label }}</span>
        </button>
      </template>

      <template v-if="hasMenuItems" #default="{ close }">
        <slot name="menu" :close="close" />
      </template>
    </app-dropdown>
  </app-tooltip>
</template>

<script lang="ts" setup>
defineProps<{
  label: string
  tooltip?: string
  isActive?: boolean
}>()

const emit = defineEmits<{
  click: []
}>()

const slots = useSlots()

const hasMenuItems = computed(() => !!slots.menu)

function handleClick() {
  emit('click')
}
</script>
