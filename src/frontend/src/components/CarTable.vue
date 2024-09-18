<script lang="ts" setup>
import CarRow from './CarRow.vue'
import AddCarButton from './AddCarButton.vue'
import UpdateCarButton from './EditCarButton.vue'
</script>

<template>
  <div v-if="!historyMode">
    <AddCarButton v-if="userCar == null" />
    <UpdateCarButton v-else :car="userCar" />
  </div>
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
      <CarRow v-for="(car, index) in cars" :eventId="eventId" :car="car" :key="index" />
    </tbody>
  </table>
</template>

<script lang="ts">
import { type Car } from '@/models'
import { defineComponent, inject } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useEventStore } from '@/stores/events'


export default defineComponent({
  props: {
    eventId: Number
  },
  data() {
    return {
      historyMode: inject("historyMode")
    }
  },
  computed: {
    cars() {
      const eventStore = useEventStore()
      return eventStore.selectedEvent?.cars
    },
    userCar() {
      const eventStore = useEventStore()
      const authStore = useAuthStore()
      return eventStore.selectedEvent?.cars
        ?.filter((car) => car.driver.id === authStore.user?.id)
        .pop()
    }
  },
  methods: {
    async fetchCarData() {
      try {
        const response = await fetch(`/api/v1/event/${this.eventId}/car/`)
        if (!response.ok) {
          throw new Error(`Error: ${response.statusText}`)
        }
        const data: Car[] = await response.json()
        const eventStore = useEventStore()
        if (eventStore.selectedEvent) {
          eventStore.selectedEvent.cars = data
        }
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
