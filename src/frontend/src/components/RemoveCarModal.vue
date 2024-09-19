<template>
  <div
    class="modal fade"
    id="removeCarModal"
    tabindex="-1"
    role="dialog"
    aria-labelledby="removeCarModalLabel"
    aria-hidden="true"
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title" id="removeCarModalLabel">Remove Car</h4>
          <button type="button" class="close" data-bs-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <h5>Are you sure you want to remove your car?</h5>
        </div>
        <div class="modal-footer">
          <button
            type="button"
            id="removeCarClose"
            class="btn btn-secondary"
            data-bs-dismiss="modal"
          >
            Cancel
          </button>
          <button type="button" class="btn btn-primary" @click="removeCar">Remove</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, type PropType } from 'vue'
import { useEventStore } from '@/stores/events'
import type { Car } from '@/models'

export default defineComponent({
  props: {
    car: Object as PropType<Car>
  },
  methods: {
    async removeCar() {
      if (this.car == null) {
        console.error('Cannot delete: Car reference is null or undefined.')
        return
      }
      try {
        const eventStore = useEventStore()
        const response = await fetch(
          `/api/v1/event/${eventStore.selectedEvent?.id}/car/${this.car?.id}`,
          {
            method: 'DELETE'
          }
        )

        if (response.ok) {
          eventStore.removeCar(this.car)

          this.closeModal()
        } else {
          console.error('Error:', response.statusText)
        }
      } catch (error) {
        console.error('Network error:', error)
      }
    },
    closeModal() {
      const closeButton = document.getElementById('removeCarClose')
      closeButton?.click()
    }
  }
})
</script>
