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

    <div class="space-y-2 max-h-[calc(100vh-12rem)] overflow-y-auto">
      <item-profile
        v-for="profile in gameStore.getProfiles"
        :key="profile.name"
        :profile="profile"
        :is-active="profile.name === gameStore.getProfile?.name"
        :game-id="gameStore.selectedGame!"
        :all-profiles="gameStore.getProfiles"
        @merged="refreshGame()"
      />
    </div>

    <modal-create-profile
      ref="createModalRef"
      :game-id="gameStore.selectedGame!"
      :existing-profile-names="getProfileNames"
      @created="refreshGame()"
    />
  </div>
</template>

<script lang="ts" setup>
const gameStore = useGameStore()

const createModalRef = useTemplateRef('createModalRef')

const getProfileNames = computed(() => gameStore.getProfiles.map(p => p.name))

const refreshGame = inject('refreshGame') as () => void
</script>
