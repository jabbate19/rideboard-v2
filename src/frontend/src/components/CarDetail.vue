<script setup lang="ts">
import JoinCarButton from './JoinCarButton.vue'
import LeaveCarButton from './LeaveCarButton.vue'
</script>

<template>
  <div class="accordian-body collapse m-4" v-bind:id="'carDetail' + car!.id">
    <h4>Driver Comment:</h4>
    <p>{{ car!.comment }}</p>
    <h4>Passengers:</h4>
    <ul class="no-bullets">
      <li v-for="(rider, index) in car!.riders" :key="index">{{ rider.name }}</li>
    </ul>
    <LeaveCarButton v-if="userInCar" :carId="car!.id" :rider="userInCar" />
    <JoinCarButton
      v-else-if="car!.riders.length < car!.maxCapacity && userCanJoinCar"
      :carId="car?.id"
    />
  </div>
</template>

<script lang="ts">
import { type Car } from '@/models'
import { defineComponent, type PropType } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useEventStore } from '@/stores/events'

export default defineComponent({
  props: {
    car: Object as PropType<Car>,
    eventId: Number
  },
  computed: {
    userCanJoinCar() {
      const authStore = useAuthStore()
      const eventStore = useEventStore()
      return (
        !(
          eventStore.selectedEvent?.cars?.map((car) => car.driver.id).includes(authStore.user!.id) ||
          eventStore.selectedEvent?.cars
            ?.map((car) => car.riders)
            .flat()
            .map((rider) => rider.id)
            .includes(authStore.user!.id)
        )
      )
    },
    userInCar() {
      const authStore = useAuthStore()
      return this.car?.riders.find((rider) => rider.id === authStore.user!.id)
    }
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
