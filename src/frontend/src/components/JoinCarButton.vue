<template>
  <button type="button" class="btn btn-primary mt-3" @click="joinCar">Join Car</button>
</template>

<script lang="ts">
import { useAuthStore } from '@/stores/auth'
import { useEventStore } from '@/stores/events'
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    carId: Number
  },
  methods: {
    async joinCar() {
      try {
        const authStore = useAuthStore()
        const eventStore = useEventStore()
        const response = await fetch(
          `/api/v1/event/${eventStore.selectedEvent?.id}/car/${this.carId}/rider/`,
          {
            method: 'POST',
          }
        )

        if (response.ok) {
          eventStore.selectedEvent?.cars
            ?.filter((car) => car.id === this.carId)
            .pop()
            ?.riders.push({
              id: authStore.user!.id,
              name: authStore.user!.given_name + " " + authStore.user!.family_name
            })
        } else {
          console.error('Error:', response.statusText)
        }
      } catch (error) {
        console.error('Network error:', error)
      }
    }
  }
})
</script>
