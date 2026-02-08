<template>
  <div class="border overflow-hidden rounded border-gray-700">
    <div class="grid grid-cols-12 p-2.5 border border-gray-800 items-center gap-2.5 text-left [&>p]:whitespace-nowrap bg-gray-800">
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
      v-for="i in 3"
      :key="i"
      class="group"
    >
      <item-mod
        :order="2"
        enabled
        name="Example Mod"
        pack="Example Pack"
        last-update="2024-06-01"
      />
      <hr class="h-px mx-2.5 border-gray-800 group-last:border-none">
    </div>
  </div>
</template>

<script lang="ts" setup>

</script>
