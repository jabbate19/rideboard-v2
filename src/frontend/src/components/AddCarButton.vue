<script setup lang="ts">
import UserSelectSearch from './UserSelectSearch.vue';
</script>

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
              :min="departureTime"
              type="datetime-local"
              class="form-control"
              id="addCarReturn"
            />
          </div>
          <div class="form-group">
            <label for="addCarDeparture">Maximum Capacity</label>
            <input
              v-model="maxCapacity"
              type="number"
              min="0"
              class="form-control"
              id="addCarDeparture"
            />
          </div>
          <div class="form-group">
            <label for="addCarComments">Comments</label>
            <input v-model="comment" class="form-control" id="addCarComments" />
          </div>
          <div class="form-group">
            <label>Riders</label>
            <UserSelectSearch v-model="riders" />
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
import { validateCar } from '@/validators';

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
      maxCapacity: 0,
      riders: [] as UserStub[]
    };
  },
  methods: {
    async sendData() {
      const popupStore = usePopupStore();
      const authStore = useAuthStore();
      const eventStore = useEventStore();
      let validate = validateCar(
        authStore.user!,
        this.departureTime,
        this.returnTime,
        this.maxCapacity,
        this.riders,
        eventStore.selectedEventCars!
      );
      if (validate.length != 0) {
        validate.forEach((issue) => popupStore.addPopup(PopupType.Danger, issue));
        return;
      }
      const data = {
        departureTime: new Date(this.departureTime).toISOString(),
        returnTime: new Date(this.returnTime).toISOString(),
        maxCapacity: this.maxCapacity,
        comment: this.comment,
        riders: this.riders.map((rider) => rider.id)
      };

      try {
        const response = await fetch(`/api/v1/event/${eventStore.selectedEvent?.id}/car/`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(data)
        });

        if (response.ok) {
          const result = await response.json();

          const newCar = {
            id: result,
            driver: {
              id: authStore.user!.id,
              realm: authStore.user!.type,
              name: authStore.user!.given_name + ' ' + authStore.user!.family_name,
              email: authStore.user!.email!
            },
            departureTime: new Date(this.departureTime),
            returnTime: new Date(this.returnTime),
            maxCapacity: this.maxCapacity,
            comment: this.comment,
            riders: this.riders
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
