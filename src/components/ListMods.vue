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
        <p class="col-span-5 ml-11">
          Pack
        </p>
        <p class="col-span-3">
          Last update
        </p>
      </div>
      <div>
        <div v-bind="containerProps" class="relative max-h-150">
          <div v-bind="wrapperProps">
            <div
              v-for="({ data }, index) of virtualisedList"
              :key="index"
              class="group w-[400%] sm:w-[125%] md:w-full"
              :style="{ height: `${ITEM_HEIGHT}px` }"
            >
              <item-mod
                :order="2"
                enabled
                :name="data.name"
                :pack="data.name"
                :last-updated="data.lastUpdated"
                :image="data.image"
              />
              <hr class="h-px mx-2.5 border-gray-800 group-last:border-none">
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
// Props
const props = defineProps<{
  list: unknown[]
  loading: boolean
}>()

// Non reactive state
const ITEM_HEIGHT = 60 // px

// Reactive state
const filters = ref({
  search: '',
  sortBy: 'order',
})

// Computed
const getList = computed(() => {
  const arr = Array.isArray(props.list) ? props.list : []

  if (arr.length === 0) {
    return []
  }

  const search = filters.value.search.toLowerCase()

  const checkTransformed = (x: string | undefined) => !x || x.toLowerCase().includes(search.replace(/ /g, '_'))
  const checkNormal = (x: string | undefined) => !x || x.toLowerCase().includes(search)
  const check = (x: string | undefined) => checkTransformed(x) || checkNormal(x)

  return arr
    .filter(item => check(item?.name))
    .sort((a, b) => {
      switch (filters.value.sortBy) {
        case 'order':
          return a.order - b.order
        case 'name':
          if (!a.name && !b.name)
            return 0
          return a.name.localeCompare(b.name)
        case 'pack':
          if (!a.pack && !b.pack)
            return 0
          return a.pack.localeCompare(b.pack)
        case 'lastUpdate':
          return new Date(b.lastUpdated).getTime() - new Date(a.lastUpdated).getTime()
        default:
          return 0
      }
    })
})
const sortOptions = computed(() => [
  { value: '', label: 'Sort by', disabled: true },
  { value: 'order', label: 'Order' },
  { value: 'name', label: 'Name' },
  { value: 'pack', label: 'Pack' },
  { value: 'lastUpdate', label: 'Last update' },
])

// Composables
const { list: virtualisedList, containerProps, wrapperProps } = useVirtualList(
  getList,
  {
    itemHeight: ITEM_HEIGHT,
  },
)
</script>
