<script lang="ts" setup>
import AddCarButton from './AddCarButton.vue'
import UpdateCarButton from './EditCarButton.vue'
import CarRowGroup from './CarRowGroup.vue'
import LeaveCarModal from './LeaveCarModal.vue'
import Loading from './LoadingWheel.vue'

const eventStore = useEventStore()
</script>

<template>
  <Loading v-if="loading" />
  <div v-else>
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
          <th scope="col"></th>
        </tr>
      </thead>
      <tbody>
        <CarRowGroup
          v-for="car in eventStore.selectedEventCars"
          :car="car"
          :eventId="eventId"
          :key="car.id"
        />
      </tbody>
    </table>
    <LeaveCarModal v-for="car in eventStore.selectedEventCars" :carId="car!.id" :key="car!.id" />
  </div>
</template>

<script lang="ts">
import { PopupType, type Car } from '@/models'
import { defineComponent, inject } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useEventStore } from '@/stores/events'
import { usePopupStore } from '@/stores/popup'

export default defineComponent({
  props: {
    eventId: Number
  },
  data() {
    return {
      historyMode: inject('historyMode'),
      loading: true
    }
  },
  computed: {
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
          const popupStore = usePopupStore()
          popupStore.addPopup(PopupType.Danger, `Failed to Get Cars (${response.status})`)
          return
        }
        const data: Car[] = await response.json()
        const eventStore = useEventStore()
        if (eventStore.selectedEvent) {
          eventStore.selectedEvent.cars = data
        }
        this.loading = false
      } catch (error) {
        console.error(error)
        const popupStore = usePopupStore()
        popupStore.addPopup(PopupType.Danger, 'Failed to Get Cars. An unknown error occured.')
      }
    }
  },
  created() {
    this.fetchCarData() // Fetch card data when the component is created
  }
})
</script>

<style>
.collapse-enter-active,
.collapse-leave-active {
  transition: all 0.35s ease;
}

.collapse-enter-from,
.collapse-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.collapse-enter-active *,
.collapse-leave-active * {
  transition: all 0.35s ease;
}

.collapse-enter-from *,
.collapse-leave-to * {
  opacity: 0;
  transform: translateY(-30px);
}
</style>
