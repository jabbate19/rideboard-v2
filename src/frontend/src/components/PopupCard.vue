<script setup lang="ts">
import { type PopupType } from '@/models';
</script>

<template>
  <div :class="'popup card text-white p-3 mb-3 ' + alertType!" tabindex="-2"><slot></slot></div>
</template>

<script lang="ts">
import { defineComponent, type PropType } from 'vue';

export default defineComponent({
  props: {
    uuid: String,
    alertType: String as PropType<PopupType>
  }
});
</script>

<style scoped>
div:hover {
  opacity: 60%;
  transition: all 0.5s ease;
}
</style>
