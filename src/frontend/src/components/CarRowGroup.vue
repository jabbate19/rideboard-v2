<script lang="ts" setup>
import CarRow from './CarRow.vue';
import CarDetail from './CarDetail.vue';
</script>

<template>
  <CarRow :car="car" :key="car!.id" :rotateCaret="visible" @click="visible = !visible" />
  <Transition name="collapse">
    <tr v-if="visible">
      <td colspan="5">
        <CarDetail :eventId="eventId" :car="car" />
      </td>
    </tr>
  </Transition>
</template>

<script lang="ts">
import { type Car } from '@/models';
import { defineComponent, type PropType } from 'vue';

export default defineComponent({
  props: {
    car: Object as PropType<Car>,
    eventId: Number
  },
  data() {
    return {
      visible: false
    };
  }
});
</script>

<style>
.collapse-enter-active,
.collapse-leave-active {
  transition: all 0.35s ease;
}

.collapse-enter-from,
.collapse-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.collapse-enter-active *,
.collapse-leave-active * {
  transition: all 0.35s ease;
}

.collapse-enter-from *,
.collapse-leave-to * {
  opacity: 0;
  transform: translateY(-30px);
}
</style>
