<script lang="ts" setup>
import CarDetail from './CarDetail.vue'
</script>

<template>
  <tr
    data-bs-toggle="collapse"
    v-bind:data-bs-target="'#carDetail' + car!.id"
    class="accordion-toggle hover-row"
  >
    <td>{{ car!.driver }}</td>
    <td>{{ car!.riders.length }} / {{ car!.max_capacity }}</td>
    <td>{{ departureTime }}</td>
    <td>{{ returnTime }}</td>
  </tr>
  <tr class="hiddenRow">
    <td colspan="4" class="hiddenData">
      <CarDetail :car="car" :eventId="eventId" :userIsDriver="userIsDriver" />
    </td>
  </tr>
</template>

<script lang="ts">
import { type Car } from '@/models'
import { defineComponent, type PropType } from 'vue'
import { format } from 'date-fns'

export default defineComponent({
  props: {
    car: Object as PropType<Car>,
    eventId: Number,
    userIsDriver: Boolean
  },
  computed: {
    departureTime() {
      let data = this.car!.departure_time.toLocaleString()
      return format(data, 'MM/dd/yyyy HH:mm a')
    },
    returnTime() {
      let data = this.car!.return_time.toLocaleString()
      return format(data, 'MM/dd/yyyy HH:mm a')
    }
  }
})
</script>

<style>
.hiddenData {
  padding: 0 !important;
}
</style>
