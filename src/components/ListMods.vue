<template>
  <div class="space-y-2.5">
    <div class="flex items-end gap-2.5 w-full">
      <app-input v-model="filters.search" placeholder="Search by pack or name..." class="w-full" label="Search" />
      <div class="flex items-end gap-2.5">
        <app-select v-model="filters.sortBy" :options="sortOptions" label="Sort by" />
        <app-select v-model="filters.sortOrder" :options="orderOptions" label="Order " />
        <app-dropdown>
          <template #trigger="{ toggle }">
            <div>
              <app-button variant="secondary" @click="toggle">
                <span class="sr-only">Menu</span>
                <nuxt-icon name="mi:menu" class="size-5 align-middle block" />
              </app-button>
            </div>
          </template>
          <template #default>
            <div class="p-2.5">
              <div class="grid gap-1.5">
                <app-button @click="toggleAllMods()">
                  Toggle All mods
                </app-button>
              </div>
            </div>
          </template>
        </app-dropdown>
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
        <div v-bind="containerProps" class="relative max-h-[80svh]">
          <div v-bind="wrapperProps">
            <div
              v-for="({ data }, index) of virtualisedList"
              :key="index"
              class="group w-[400%] sm:w-[125%] md:w-full"
              :style="{ height: `${ITEM_HEIGHT}px` }"
            >
              <item-mod
                :order="data.order"
                :enabled="Boolean(data.enabled)"
                :name="data.name!"
                :last-updated="data.lastUpdated"
                :image="data.image"
                :can-enable="data.canEnable"
                @status="changeStatus(data.name!, $event)"
                @order="changeOrder(data.name!, $event)"
                @refresh="emit('refresh')"
              />
              <hr class="h-px mx-2.5 border-gray-800 group-last:border-none select-none" aria-hidden="true">
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <mods-edit-bar
    :visible="hasEdits"
    :can-undo="canUndo"
    :can-redo="canRedo"
    @cancel="cancel()"
    @save="saveEdits"
    @undo="undo()"
    @redo="redo()"
  />
</template>

<script lang="ts" setup>
import type { ModResponseDto, ProfileRequestDto, ProfileResponseDto } from '~/types/dto'
import { profileResponseToRequest } from '~/utils/dto'
// Props
const props = defineProps<{
  gameId: string
  list: ModResponseDto[]
  profile: Nullable<ProfileResponseDto>
}>()

const emit = defineEmits<{
  refresh: []
}>()

// Non reactive state
const ITEM_HEIGHT = 60 // px
const sortOptions = [
  { value: '', label: 'Sort by', disabled: true, selected: true },
  { value: 'order', label: 'Order' },
  { value: 'name', label: 'Name' },
  { value: 'lastUpdate', label: 'Last update' },
]
const orderOptions = [
  { value: 'asc', label: 'Asc', selected: true },
  { value: 'desc', label: 'Desc' },
]

// Reactive state
const filters = ref({
  search: '',
  sortBy: 'order',
  sortOrder: 'desc',
})
const localList = ref<ModResponseDto[]>([])

const {
  snapshots,
  undo,
  redo,
  commit,
  cancel,
  canUndo,
  canRedo,
} = useHistory(localList)

// Computed
const getList = computed(() => Array.isArray(props.list) ? props.list : [])
const getLocalList = computed(() => {
  const search = filters.value.search.toLowerCase()

  const checkTransformed = (x: string | undefined) => !x || x.toLowerCase().includes(search.replace(/ /g, '_'))
  const checkNormal = (x: string | undefined) => !x || x.toLowerCase().includes(search)
  const check = (x: string | undefined) => checkTransformed(x) || checkNormal(x)

  const sorted = localList.value.filter(item => Boolean(item.name) && check(item.name))
    .sort((a, b) => {
      switch (filters.value.sortBy) {
        case 'order':
          return (a.order ?? 0) - (b.order ?? 0)
        case 'name':
          return a.name!.localeCompare(b.name!)
        case 'lastUpdate':
          if (!a.lastUpdated || !b.lastUpdated)
            return 0

          return new Date(b.lastUpdated).getTime() - new Date(a.lastUpdated).getTime()
        default:
          return 0
      }
    })

  return filters.value.sortOrder === 'asc' ? sorted.reverse() : sorted
})

watch(() => getList.value, (value) => {
  localList.value = value.map(m => ({ ...m })) // Shallow clone to trigger reactivity on properties
  commit()
}, { immediate: true, deep: true })

// Composables
const { list: virtualisedList, containerProps, wrapperProps } = useVirtualList(
  getLocalList,
  {
    itemHeight: ITEM_HEIGHT,
  },
)
const hasEdits = computed(() => snapshots.value.length > 1)

// Functions
function toggleAllMods() {
  const toggle = !localList.value.every(mod => mod.enabled)
  localList.value.forEach((mod: ModResponseDto) => {
    mod.enabled = toggle
  })
}

async function saveEdits() {
  try {
    const profileRequest = profileResponseToRequest(
      {
        ...props.profile,
        mods: localList.value,
      },
      props.gameId,
    )

    await useTauriInvoke<ProfileRequestDto>('update_profile', { payload: profileRequest })
    commit()
    emit('refresh')
  }
  catch (error) {
    console.error('Failed to save edits:', error)
  }
}

function changeStatus(name: string, value: boolean) {
  const modIndex = localList.value.findIndex((m: ModResponseDto) => m.name === name)
  if (modIndex !== -1) {
    localList.value[modIndex]!.enabled = value
  }
}

function changeOrder(name: string, value: number) {
  const modIndex = localList.value.findIndex((m: ModResponseDto) => m.name === name)
  if (modIndex !== -1) {
    localList.value[modIndex]!.order = value
  }
}
</script>
