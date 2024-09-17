<script lang="ts" setup>
import { computed } from 'vue'
import { useAuthStore } from '@/stores/auth'
const authStore = useAuthStore()
const user = computed(() => authStore.user!)
</script>

<template>
  <div class="accordian-body collapse m-4" v-bind:id="'carDetail' + car!.id">
    <h4>Driver Comment:</h4>
    <p>{{ car!.comment }}</p>
    <h4>Passengers:</h4>
    <ul class="no-bullets">
      <li v-for="(rider, index) in car!.riders" :key="index">{{ rider }}</li>
    </ul>
    <button
      v-if="
        car!.riders.length < car!.max_capacity && !car!.riders.includes(user.id) && !userIsDriver
      "
      type="button"
      class="btn btn-primary mt-3"
    >
      Join Car
    </button>
  </div>
</template>

<script lang="ts">
import { type Car } from '@/models'
import { defineComponent, type PropType } from 'vue'

export default defineComponent({
  props: {
    car: Object as PropType<Car>,
    eventId: Number,
    userIsDriver: Boolean
  }
})
</script>

<style>
ul.no-bullets {
  list-style-type: none; /* Remove bullets */
  padding: 0; /* Remove padding */
  margin: 0; /* Remove margins */
}
</style>
