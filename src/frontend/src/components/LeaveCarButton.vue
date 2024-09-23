<template>
  <button
    type="button"
    class="btn btn-danger mt-3"
    data-bs-toggle="modal"
    :data-bs-target="'#leaveCarModal' + carId!"
  >
    Leave Car
  </button>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    carId: Number
  }
})
</script>
