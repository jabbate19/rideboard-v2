<template>
  <button
    type="button"
    class="btn btn-primary mb-2"
    data-bs-toggle="modal"
    data-bs-target="#addCarModal"
  >
    Add Car
  </button>
  <div
    class="modal fade"
    id="addCarModal"
    tabindex="-1"
    role="dialog"
    aria-labelledby="addCarModalLabel"
    aria-hidden="true"
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title" id="addCarModalLabel">Add Car</h4>
          <button type="button" class="close" data-bs-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label for="addCarDeparture">Departure Time</label>
            <input
              v-model="departureTime"
              type="datetime-local"
              class="form-control"
              id="addCarDeparture"
            />
          </div>
          <div class="form-group">
            <label for="addCarReturn">Return Time</label>
            <input
              v-model="returnTime"
              type="datetime-local"
              class="form-control"
              id="addCarReturn"
            />
          </div>
          <div class="form-group">
            <label for="addCarDeparture">Maximum Capacity</label>
            <input v-model="maxCapacity" type="number" class="form-control" id="addCarDeparture" />
          </div>
          <div class="form-group">
            <label for="addCarComments">Comments</label>
            <input v-model="comment" class="form-control" id="addCarComments" />
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" id="addCarClose" class="btn btn-secondary" data-bs-dismiss="modal">
            Close
          </button>
          <button type="button" class="btn btn-primary" @click="sendData">Create</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import { useEventStore } from '@/stores/events'
import { useAuthStore } from '@/stores/auth'
import type { UserStub } from '@/models'

export default defineComponent({
  setup() {
    const eventStore = useEventStore()
    const carDeparture = new Date(eventStore.selectedEvent!.startTime).toISOString().slice(0, 16)
    const carReturn = new Date(eventStore.selectedEvent!.endTime).toISOString().slice(0, 16)

    const carDepartureValue = ref(carDeparture)
    const carReturnValue = ref(carReturn)
    return {
      departureTime: carDepartureValue,
      returnTime: carReturnValue,
      comment: '',
      maxCapacity: 0
    }
  },
  methods: {
    async sendData() {
      if (this.departureTime === null || this.returnTime === null) {
        console.error('All times must be filled in.')
        return
      }
      const data = {
        departureTime: new Date(this.departureTime).toISOString(),
        returnTime: new Date(this.returnTime).toISOString(),
        maxCapacity: this.maxCapacity,
        comment: this.comment
      }

      try {
        const eventStore = useEventStore()
        const response = await fetch(`/api/v1/event/${eventStore.selectedEvent?.id}/car/`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(data)
        })

        if (response.ok) {
          const result = await response.json()

          const authStore = useAuthStore()
          const newCar = {
            id: result,
            driver: {
              id: authStore.user!.id,
              name: authStore.user!.given_name + ' ' + authStore.user!.family_name
            },
            departureTime: new Date(this.departureTime),
            returnTime: new Date(this.returnTime),
            maxCapacity: this.maxCapacity,
            comment: this.comment,
            riders: [] as UserStub[]
          }
          eventStore.addCar(newCar)
          this.closeModal()
        } else {
          console.error('Error:', response.statusText)
        }
      } catch (error) {
        console.error('Network error:', error)
      }
    },
    closeModal() {
      const closeButton = document.getElementById('addCarClose')
      closeButton?.click()
    }
  }
})
</script>
