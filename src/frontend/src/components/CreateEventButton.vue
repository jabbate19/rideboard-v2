<script setup lang="ts">
import IconPlus from './icons/IconPlus.vue';
import CreateEventModal from './CreateEventModal.vue';
</script>

<template>
  <button
    type="button"
    class="btn btn-success w-100 py-1 px-0"
    data-bs-toggle="modal"
    data-bs-target="#createEventModal"
  >
    <IconPlus />
  </button>
  <CreateEventModal />
</template>
