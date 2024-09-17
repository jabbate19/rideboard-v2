<script lang="ts" setup>
import EventCard from '@/components/EventCard.vue'
import EventDetails from '@/components/EventDetails.vue'
</script>

<template>
  <div class="container">
    <div class="row">
      <!-- Left column: List of cards -->
      <div class="cardlist col-4 p-auto card-header">
        <EventCard
          v-for="(card, index) in cards"
          :card="card"
          :key="index"
          @click="selectCard(card)"
        />
      </div>

      <!-- Right column: Display selected card details -->
      <div class="col-8">
        <EventDetails v-if="selectedCard" :event="selectedCard" />
        <div v-else>
          <p>Select an Event to see details</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { type Event } from '@/models'

export default {
  data() {
    return {
      cards: [] as Event[],
      selectedCard: null as Event | null
    }
  },
  methods: {
    async fetchCardData() {
      try {
        const response = await fetch('/api/v1/event/')
        if (!response.ok) {
          throw new Error(`Error: ${response.statusText}`)
        }
        const data = await response.json()
        this.cards = data // assuming the API returns an array of card objects
      } catch (error) {
        console.error('Error fetching card data:', error)
      }
    },
    selectCard(card: Event) {
      this.selectedCard = card
    }
  },
  created() {
    this.fetchCardData() // Fetch card data when the component is created
  }
}
</script>

<style>
.cardlist {
  height: 90vh;
  max-height: 90vh;
  overflow: auto;
}
</style>
