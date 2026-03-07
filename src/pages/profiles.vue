<template>
  <div class="space-y-4">
    <app-breadcrump
      :list="[{ label: 'Home', path: '/' }, { label: 'Profiles', path: '/profiles' }]"
    />

    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold">
          Profiles
        </h1>
        <p class="text-sm text-gray-400">
          Manage your game profiles
        </p>
      </div>
      <app-button class="px-4 py-2" @click="createModalRef?.open()">
        Create New Profile
      </app-button>
    </div>

    <table-profiles
      :profiles="gameStore.getProfiles"
      :game-id="gameStore.selectedGame!"
      @refresh="gameStore.fetchGame"
    />

    <modal-create-profile
      ref="createModalRef"
      :game-id="gameStore.selectedGame!"
      :existing-profile-names="getProfileNames"
    />
  </div>
</template>

<script lang="ts" setup>
const gameStore = useGameStore()

const createModalRef = useTemplateRef('createModalRef')

const getProfileNames = computed(() => gameStore.getProfiles.map(p => p.name))
</script>
