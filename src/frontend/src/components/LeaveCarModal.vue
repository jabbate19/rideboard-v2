<template>
  <div
    class="modal fade"
    :id="'leaveCarModal' + carId!"
    tabindex="-1"
    role="dialog"
    :aria-labelledby="'leaveCarModalLabel' + carId!"
    aria-hidden="true"
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title" :id="'leaveCarModalLabel' + carId!">Leave Car</h4>
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
            :id="'leaveCarClose' + carId!"
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
import { defineComponent } from 'vue';
import { useEventStore } from '@/stores/events';
import { useAuthStore } from '@/stores/auth';
import { usePopupStore } from '@/stores/popup';
import { PopupType } from '@/models';

export default defineComponent({
  props: {
    carId: Number
  },
  methods: {
    async leaveCar() {
      const popupStore = usePopupStore();
      try {
        const eventStore = useEventStore();
        const authStore = useAuthStore();
        const response = await fetch(
          `/api/v1/event/${eventStore.selectedEvent?.id}/car/${this.carId}/rider/`,
          {
            method: 'DELETE'
          }
        );

        if (!response.ok) {
          popupStore.addPopup(PopupType.Danger, `Failed to Leave Car (${response.status})`);
          return;
        }

        const riders = eventStore.selectedEvent?.cars?.find((car) => car.id === this.carId)?.riders;
        const rider = {
          id: authStore.user!.id,
          name: authStore.user!.given_name + ' ' + authStore.user!.family_name
        };
        riders?.splice(riders?.indexOf(rider), 1);
        popupStore.addPopup(PopupType.Success, 'You have been removed from this car!');
        this.closeModal();
      } catch (error) {
        console.error(error);
        popupStore.addPopup(PopupType.Danger, 'Failed to Leave Car. An unknown error occured.');
      }
    },
    closeModal() {
      const closeButton = document.getElementById('leaveCarClose' + this.carId!);
      closeButton?.click();
    }
  }
});
</script>
