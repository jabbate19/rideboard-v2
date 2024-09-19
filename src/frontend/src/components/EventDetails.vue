<script lang="ts" setup>
import CarTable from './CarTable.vue'
import EditEventButton from './EditEventButton.vue'
import IconPin from './icons/IconPin.vue'
</script>

<template>
  <div class="card">
    <div class="card-body">
      <div>
        <h3 class="card-title">{{ event?.name }}</h3>
        <h5 class="card-text">
          <a v-bind:href="'https://maps.google.com/?q=' + event?.location"
            ><IconPin /> {{ event?.location }}</a
          >
        </h5>
        <h5 class="card-text">{{ startTime }} - {{ endTime }}</h5>
      </div>
      <div class="mt-4">
        <h4>Cars</h4>
        <CarTable :eventId="event?.id" :key="event?.id" />
      </div>
      <EditEventButton v-if="userOwnsEvent" />
      <div>
        <i>Created by {{ event?.creator.name }}</i>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, type PropType } from 'vue'
import { type Event } from '@/models'
import { format } from 'date-fns'
import { useAuthStore } from '@/stores/auth'

export default defineComponent({
  props: {
    event: Object as PropType<Event>
  },
  data() {
    const authStore = useAuthStore()
    return {
      userOwnsEvent: this.event!.creator.id === authStore.user!.id
    }
  },
  computed: {
    startTime() {
      let data = this.event!.startTime.toLocaleString()
      return format(data, 'LLLL dd, yyyy HH:mm a')
    },
    endTime() {
      let data = this.event!.endTime.toLocaleString()
      return format(data, 'LLLL dd, yyyy HH:mm a')
    }
  }
})
</script>
