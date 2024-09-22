<script setup lang="ts">
import CaretRight from './icons/CaretRight.vue'
</script>

<template>
  <div class="card mb-3">
    <div class="card-body d-flex justify-content-between align-items-center">
      <div>
        <h5 class="card-title">{{ event!.name }}</h5>
        <h6 class="card-time">{{ formattedStart }}</h6>
      </div>
      <CaretRight v-if="screenWidth < 768" />
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
  data() {
    return {
      screenWidth: window.innerWidth
    }
  },
  mounted() {
    window.addEventListener('resize', this.updateSize)
  },
  methods: {
    updateSize() {
      this.screenWidth = window.innerWidth
    }
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
