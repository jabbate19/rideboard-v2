<script lang="ts" setup>
import EventCard from '@/components/EventCard.vue'
import EventDetails from '@/components/EventDetails.vue'
import CreateEventButton from '@/components/CreateEventButton.vue'
import { useEventStore } from '@/stores/events'

const eventStore = useEventStore()
</script>

<template>
  <div class="container">
    <div class="row">
      <!-- Left column: List of cards -->
      <div class="cardlist col-4 p-auto card-header">
        <EventCard
          v-for="(event, index) in eventStore.events"
          :event="event"
          :key="index"
          @click="eventStore.selectEvent(event)"
        />
        <CreateEventButton v-if="!showPast" />
      </div>
      <!-- Right column: Display selected card details -->
      <div class="col-8">
        <EventDetails v-if="eventStore.selectedEvent" :event="eventStore.selectedEvent" />
        <div v-else>
          <p>Select an Event to see details</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { type Event } from '@/models'
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    showPast: Boolean
  },
  data() {
    return {
      selectedCard: null as Event | null
    }
  },
  methods: {
    async fetchCardData() {
      try {
        const response = await fetch(
          '/api/v1/event/?' +
            new URLSearchParams({
              past: this.showPast.toString()
            }).toString()
        )
        if (!response.ok) {
          throw new Error(`Error: ${response.statusText}`)
        }
        const data = await response.json()
        const eventStore = useEventStore()
        eventStore.setEvents(data)
        eventStore.selectedEvent = null;
      } catch (error) {
        console.error('Error fetching card data:', error)
      }
    }
  },
  created() {
    this.fetchCardData() // Fetch card data when the component is created
  },
  provide() {
    return {
      historyMode: this.showPast
    }
  }
})
</script>

<style>
.cardlist {
  height: 90vh;
  max-height: 90vh;
  overflow: auto;
}
</style>
