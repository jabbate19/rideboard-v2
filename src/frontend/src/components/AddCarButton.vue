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
import { defineComponent, ref } from 'vue';
import { useEventStore } from '@/stores/events';
import { useAuthStore } from '@/stores/auth';
import { PopupType, type UserStub } from '@/models';
import { format } from 'date-fns';
import { usePopupStore } from '@/stores/popup';

export default defineComponent({
  data() {
    const eventStore = useEventStore();
    const carDeparture = format(
      new Date(eventStore.selectedEvent!.startTime).toLocaleString(),
      "yyyy-MM-dd'T'HH:mm:ss"
    );
    const carReturn = format(
      new Date(eventStore.selectedEvent!.endTime).toLocaleString(),
      "yyyy-MM-dd'T'HH:mm:ss"
    );

    const carDepartureValue = ref(carDeparture);
    const carReturnValue = ref(carReturn);
    return {
      departureTime: carDepartureValue,
      returnTime: carReturnValue,
      comment: '',
      maxCapacity: 0
    };
  },
  methods: {
    async sendData() {
      const popupStore = usePopupStore();
      if (this.departureTime.length == 0 || this.returnTime.length == 0) {
        popupStore.addPopup(PopupType.Danger, 'All times must be filled in.');
        return;
      }
      if (this.maxCapacity < 0) {
        popupStore.addPopup(PopupType.Danger, 'Capacity must be greater than 0.');
        return;
      }
      const data = {
        departureTime: new Date(this.departureTime).toISOString(),
        returnTime: new Date(this.returnTime).toISOString(),
        maxCapacity: this.maxCapacity,
        comment: this.comment
      };

      try {
        const eventStore = useEventStore();

        const response = await fetch(`/api/v1/event/${eventStore.selectedEvent?.id}/car/`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(data)
        });

        if (response.ok) {
          const result = await response.json();

          const authStore = useAuthStore();
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
          };
          eventStore.addCar(newCar);
          popupStore.addPopup(PopupType.Success, 'Car Added!');
          this.closeModal();
        } else {
          popupStore.addPopup(PopupType.Danger, `Failed to Add Car (${response.status})`);
        }
      } catch (error) {
        console.error(error);
        popupStore.addPopup(PopupType.Danger, 'Failed to Add Car. An unknown error occured.');
      }
    },
    closeModal() {
      const closeButton = document.getElementById('addCarClose');
      closeButton?.click();
    }
  }
});
</script>
