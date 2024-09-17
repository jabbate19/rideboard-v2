<script lang="ts" setup>
import CarTable from './CarTable.vue'
</script>

<template>
  <div class="card">
    <div class="card-body">
      <div>
        <h3 class="card-title">{{ event!.name }}</h3>
        <h5 class="card-text">
          <a v-bind:href="'https://maps.google.com/?q=' + event!.location">{{ event!.location }}</a>
        </h5>
        <h5 class="card-text">{{ startTime }} - {{ endTime }}</h5>
      </div>
      <div class="mt-4">
        <h4>Cars</h4>
        <CarTable :eventId="event!.id" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, type PropType } from 'vue'
import { type Event } from '@/models'
import { format } from 'date-fns'

export default defineComponent({
  props: {
    event: Object as PropType<Event>
  },
  computed: {
    startTime() {
      let data = this.event!.start_time.toLocaleString()
      return format(data, 'LLLL dd, yyyy HH:mm a')
    },
    endTime() {
      let data = this.event!.end_time.toLocaleString()
      return format(data, 'LLLL dd, yyyy HH:mm a')
    }
  }
})
</script>
