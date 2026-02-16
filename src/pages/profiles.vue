<template>
  <div>
    <div class="space-y-4">
      <!-- Breadcrumb -->
      <div class="flex items-center gap-2">
        <app-button variant="secondary" class="px-3 py-1.5" @click="goBack">
          <div class="flex items-center gap-2">
            <nuxt-icon name="mi:arrow-left" class="size-5" />
            <span>Back</span>
          </div>
        </app-button>
      </div>

      <!-- Page Header -->
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

      <!-- Profiles List -->
      <div class="space-y-2 max-h-[calc(100vh-12rem)] overflow-y-auto">
        <item-profile
          v-for="profile in gameStore.getProfiles"
          :key="profile.name"
          :profile="profile"
          :is-active="profile.name === gameStore.getProfile?.name"
          :game-id="gameStore.selectedGame!"
          :all-profiles="gameStore.getProfiles"
        />
      </div>
    </div>

    <modal-create-profile
      ref="createModalRef"
      :game-id="gameStore.selectedGame!"
      :existing-profile-names="getProfileNames"
      @created="clearNuxtData(gameStore.getDataKey)"
    />
  </div>
</template>

<script lang="ts" setup>
const router = useRouter()
const gameStore = useGameStore()

const createModalRef = useTemplateRef('createModalRef')

const getProfileNames = computed(() => gameStore.getProfiles.map(p => p.name))

function goBack() {
  router.back()
}
</script>
