<script setup lang="ts">
import DeleteEventButton from './DeleteEventButton.vue';
import DeleteEventModal from './DeleteEventModal.vue';
</script>

<template>
  <button
    type="button"
    class="d-flex justify-content-center align-items-center btn btn-warning w-100"
    data-bs-toggle="modal"
    data-bs-target="#editEventModal"
  >
    Edit Event
  </button>
  <div
    class="modal fade"
    id="editEventModal"
    tabindex="-1"
    role="dialog"
    aria-labelledby="editEventModalLabel"
    aria-hidden="true"
  >
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header">
          <h4 class="modal-title" id="editEventModalLabel">Edit Event</h4>
          <button type="button" class="close" data-bs-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label for="editEventTitle">Event Title</label>
            <input
              v-model="eventTitle"
              class="form-control"
              id="editEventTitle"
              placeholder="My Super Cool Event"
            />
          </div>
          <div class="form-group">
            <label for="editEventLocation">Location</label>
            <input
              v-model="eventLocation"
              class="form-control"
              id="editEventLocation"
              placeholder="DSP 3"
            />
          </div>
          <div class="form-group">
            <label for="editEventStart">Start Time</label>
            <input
              v-model="eventStart"
              type="datetime-local"
              class="form-control"
              id="editEventStart"
            />
          </div>
          <div class="form-group">
            <label for="editEventEnd">End Time</label>
            <input
              v-model="eventEnd"
              type="datetime-local"
              class="form-control"
              id="editEventEnd"
            />
          </div>
        </div>
        <DeleteEventButton/>
        <div class="modal-footer">
          <button
            type="button"
            id="editEventClose"
            class="btn btn-secondary"
            data-bs-dismiss="modal"
          >
            Close
          </button>
          <button type="button" class="btn btn-primary" @click="editEvent">Edit</button>
        </div>
      </div>
    </div>
  </div>
  <DeleteEventModal/>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { useEventStore } from '@/stores/events'
import { useAuthStore } from '@/stores/auth'

export default defineComponent({
  data() {
    const eventStore = useEventStore()
    return {
      eventTitle: eventStore.selectedEvent!.name,
      eventLocation: eventStore.selectedEvent!.location,
      eventStart: eventStore.selectedEvent!.startTime,
      eventEnd: eventStore.selectedEvent!.endTime
    }
  },
  methods: {
    async editEvent() {
      const data = {
        name: this.eventTitle,
        location: this.eventLocation,
        startTime: new Date(this.eventStart).toISOString(),
        endTime: new Date(this.eventStart).toISOString()
      }

      try {
        const eventStore = useEventStore()
        const response = await fetch(`/api/v1/event/${eventStore.selectedEvent!.id}`, {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(data)
        })

        if (response.ok) {
          eventStore.selectedEvent!.name = data.name;
          eventStore.selectedEvent!.location = data.location;
          eventStore.selectedEvent!.startTime = new Date(data.startTime);
          eventStore.selectedEvent!.endTime = new Date(data.endTime);
          this.closeModal()
        } else {
          console.error('Error:', response.statusText)
        }
      } catch (error) {
        console.error('Network error:', error)
      }
    },
    closeModal() {
      const closeButton = document.getElementById('editEventClose')
      closeButton?.click()
    }
  }
})
</script>
