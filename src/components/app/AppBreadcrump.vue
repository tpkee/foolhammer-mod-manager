<template>
  <nav class="flex items-center gap-1.5 text-sm">
    <button
      class="flex items-center justify-center size-7 rounded text-gray-400 hover:text-white hover:bg-gray-700 transition-colors cursor-pointer"
      @click="router.back()"
    >
      <IconMiChevronLeft class="size-5" />
    </button>

    <template v-for="(crumb, index) in list" :key="index">
      <IconMiChevronRight v-if="index > 0" class="size-3.5 text-gray-600 shrink-0" />

      <nuxt-link-locale
        :to="crumb.path"
        :class="getClass(index === list.length - 1)"
      >
        {{ crumb.label }}
      </nuxt-link-locale>
    </template>
  </nav>
</template>

<script lang="ts" setup>
interface BreadcrumbItem {
  label: string
  path: string
}

defineProps<{
  list: BreadcrumbItem[]
}>()

const router = useRouter()

function getClass(isActive: boolean) {
  return isActive ? 'text-white font-medium' : 'text-gray-400 hover:text-white transition-colors'
}
</script>
