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
import { PopupType, type Car } from '@/models'
import { usePopupStore } from '@/stores/popup'

export default defineComponent({
  props: {
    car: Object as PropType<Car>
  },
  methods: {
    async removeCar() {
      const popupStore = usePopupStore()
      try {
        const eventStore = useEventStore()
        const response = await fetch(
          `/api/v1/event/${eventStore.selectedEvent?.id}/car/${this.car?.id}`,
          {
            method: 'DELETE'
          }
        )

        if (!response.ok) {
          popupStore.addPopup(PopupType.Danger, `Failed to Remove Car (${response.status})`)
          return
        }

        eventStore.removeCar(this.car!)
        popupStore.addPopup(PopupType.Success, 'Your car has been removed!')
        this.closeModal()
      } catch (error) {
        console.error(error)
        popupStore.addPopup(PopupType.Danger, 'Failed to Remove Car. An unknown error occured.')
      }
    },
    closeModal() {
      const closeButton = document.getElementById('removeCarClose')
      closeButton?.click()
    }
  }
})
</script>
