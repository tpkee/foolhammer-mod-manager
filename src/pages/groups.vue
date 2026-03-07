<template>
  <div class="space-y-4">
    <app-breadcrump
      :list="[{ label: 'Home', path: '/' }, { label: 'Groups', path: '/groups' }]"
    />

    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold">
          Groups
        </h1>
        <p class="text-sm text-gray-400">
          Manage your mod groups
        </p>
      </div>
      <app-button class="px-4 py-2" @click="createModalRef?.open()">
        Create New Group
      </app-button>
    </div>

    <table-groups
      :groups="gameStore.getGroups"
      :game-id="gameStore.selectedGame!"
    />

    <modal-create-group
      ref="createModalRef"
      :game-id="gameStore.selectedGame!"
    />
  </div>
</template>

<script lang="ts" setup>
const gameStore = useGameStore()

const createModalRef = useTemplateRef('createModalRef')
</script>
