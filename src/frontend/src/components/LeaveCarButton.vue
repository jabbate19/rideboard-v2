<template>
  <button type="button" class="btn btn-danger mt-3" data-bs-toggle="modal"
  data-bs-target="#leaveCarModal">Leave Car</button>
  <div
    class="modal fade"
    id="leaveCarModal"
    tabindex="-1"
    role="dialog"
    aria-labelledby="leaveCarModalLabel"
    aria-hidden="true"
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title" id="leaveCarModalLabel">Leave Car</h4>
          <button type="button" class="close" data-bs-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <h5>Are you sure you want to leave this car?</h5>
        </div>
        <div class="modal-footer">
          <button
            type="button"
            id="leaveCarClose"
            class="btn btn-secondary"
            data-bs-dismiss="modal"
          >
            Cancel
          </button>
          <button type="button" class="btn btn-primary" @click="leaveCar">Leave</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, type PropType } from 'vue'
import { useEventStore } from '@/stores/events'
import { useAuthStore } from '@/stores/auth'
import type { UserStub } from '@/models';

export default defineComponent({
  props: {
    carId: Number,
    rider: Object as PropType<UserStub>
  },
  methods: {
    async leaveCar() {
      try {
        const eventStore = useEventStore()
        const response = await fetch(
          `/api/v1/event/${eventStore.selectedEvent?.id}/car/${this.carId}/rider/`,
          {
            method: 'DELETE'
          }
        )

        if (response.ok) {
          const riders = eventStore.selectedEvent?.cars
            ?.find((car) => car.id === this.carId)?.riders
          riders?.splice(riders?.indexOf(this.rider!), 1)

          this.closeModal()
        } else {
          console.error('Error:', response.statusText)
        }
      } catch (error) {
        console.error('Network error:', error)
      }
    },
    closeModal() {
      const closeButton = document.getElementById('leaveCarClose')
      closeButton?.click()
    }
  }
})
</script>
