<template>
  <div class="space-y-2.5">
    <div class="flex items-center gap-2.5 w-full">
      <app-input v-model="filters.search" placeholder="Search by pack or name..." class="w-full" label="Search" />
      <div class="flex items-center gap-2.5">
        <app-select v-model="filters.sortBy" :options="sortOptions" label="Sort by" />
      </div>
    </div>

    <div class="border overflow-hidden rounded border-gray-700 overflow-x-auto md:overflow-x-hidden">
      <div class="grid grid-cols-12 p-2.5 border border-gray-800 items-center gap-2.5 text-left [&>p]:whitespace-nowrap bg-gray-800 w-[400%] sm:w-[125%] md:w-full">
        <div class="flex items-center gap-2.5 col-span-2">
          <p class="ml-9">
            Order
          </p>
        </div>
        <p>
          Enabled?
        </p>
        <p class="col-span-3 ml-11">
          Pack
        </p>
        <p>
          Last update
        </p>
      </div>
      <div
        v-for="(item, index) in list"
        :key="index"
        class="group w-[400%] sm:w-[125%] md:w-full"
      >
        <item-mod
          :order="2"
          enabled
          :name="item.name"
          :pack="item.name"
          :last-update="item.lastUpdate"
          :image="item.image"
        />
        <hr class="h-px mx-2.5 border-gray-800 group-last:border-none">
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  list: unknown[]
  loading: boolean
}>()

// Reactive state
const filters = ref({
  search: '',
  sortBy: 'order',
})

// Computed
const sortOptions = computed(() => [
  { value: '', label: 'Sort by', disabled: true },
  { value: 'order', label: 'Order' },
  { value: 'name', label: 'Name' },
  { value: 'pack', label: 'Pack' },
  { value: 'lastUpdate', label: 'Last update' },
])
</script>
