<script lang="ts" setup>
import EventCard from '@/components/EventCard.vue'
import EventDetails from '@/components/EventDetails.vue'
import CreateEventButton from '@/components/CreateEventButton.vue'
import { useEventStore } from '@/stores/events'

const eventStore = useEventStore()
</script>

<template>
  <div class="container">
    <button
      v-if="screenWidth <= 576"
      class="btn btn-primary mb-2"
      type="button"
      @click="returnHome()"
    >
      All Events
    </button>
    <div class="row">
      <!-- Left column: List of cards -->
      <Transition name="list">
        <div v-if="show || screenWidth > 576" class="eventList col-md-4 col-sm-12">
          <EventCard
            v-for="(event, index) in eventStore.events"
            :event="event"
            :key="index"
            @click="selectEvent(event)"
          />
          <CreateEventButton v-if="!showPast" />
        </div>
      </Transition>
      <!-- Right column: Display selected card details -->
      <div class="col">
        <Transition name="details">
          <EventDetails
            v-if="eventStore.selectedEvent && (screenWidth > 576 || !show)"
            :event="eventStore.selectedEvent"
          />
        </Transition>
        <div v-if="eventStore.selectedEvent == null && screenWidth > 576">
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
      selectedCard: null as Event | null,
      show: true,
      screenWidth: window.innerWidth
    }
  },
  mounted() {
    window.addEventListener('resize', this.updateSize)
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
        eventStore.selectedEvent = null
      } catch (error) {
        console.error('Error fetching card data:', error)
      }
    },
    updateSize() {
      this.screenWidth = window.innerWidth
    },
    selectEvent(event: Event) {
      const eventStore = useEventStore()
      eventStore.selectEvent(event)
      this.show = false
    },
    returnHome() {
      this.show = true
      if (this.screenWidth < 576) {
        const eventStore = useEventStore()
        eventStore.selectedEvent = null
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
  overflow: scroll;
}

.eventList > * {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.35s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  width: 0;
}

.details-enter-active,
.details-leave-active {
  transition: all 0.35s ease;
}

.details-enter-from,
.details-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

.col-4 {
  flex: none !important;
}

svg {
  width: 1.5em;
}
</style>
