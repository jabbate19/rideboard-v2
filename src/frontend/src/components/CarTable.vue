<script lang="ts" setup>
import CarRow from './CarRow.vue'
const authStore = useAuthStore()
const user = computed(() => authStore.user!)
</script>

<template>
  <button
    v-if="!cars.map((car) => car.driver).includes(user.id)"
    type="button"
    class="btn btn-primary"
  >
    Add Car
  </button>
  <button v-else type="button" class="btn btn-danger mb-2">Remove Car</button>
  <table class="table">
    <thead>
      <tr>
        <th scope="col">Driver</th>
        <th scope="col">Capacity</th>
        <th scope="col">Departure</th>
        <th scope="col">Return</th>
      </tr>
    </thead>
    <tbody>
      <CarRow
        v-for="(car, index) in cars"
        :eventId="eventId"
        :car="car"
        :userIsDriver="cars.map((car) => car.driver).includes(user.id)"
        :key="index"
      />
    </tbody>
  </table>
</template>

<script lang="ts">
import { type Car } from '@/models'
import { defineComponent, computed } from 'vue'
import { useAuthStore } from '@/stores/auth'

export default defineComponent({
  data() {
    return {
      cars: [] as Car[]
    }
  },
  props: {
    eventId: Number
  },
  methods: {
    async fetchCarData() {
      try {
        const response = await fetch(`/api/v1/event/${this.eventId}/car/`)
        if (!response.ok) {
          throw new Error(`Error: ${response.statusText}`)
        }
        const data = await response.json()
        this.cars = data // assuming the API returns an array of card objects
      } catch (error) {
        console.error('Error fetching card data:', error)
      }
    }
  },
  created() {
    this.fetchCarData() // Fetch card data when the component is created
  }
})
</script>
