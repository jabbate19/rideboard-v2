<template>
  <div class="card mb-3">
    <div class="card-body">
      <h5 class="card-title">{{ event!.name }}</h5>
      <h6 class="card-time">{{ formattedStart }}</h6>
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
    formattedStart() {
      let data = this.event?.startTime.toLocaleString()
      return data ? format(data, 'MM/dd/yyyy HH:mm a') : 'N/A'
    }
  }
})
</script>

<style scoped>
.card {
  cursor: pointer;
}
</style>
