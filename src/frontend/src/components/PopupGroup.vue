<script setup lang="ts">
import { usePopupStore } from '@/stores/popup';
import Popup from './PopupCard.vue';

const popupStore = usePopupStore();
</script>

<template>
  <div class="d-flex flex-column-reverse justify-content-end popup-container">
    <TransitionGroup name="popup">
      <Popup
        v-for="(popup, index) in popupStore.popups"
        :alertType="popup.alertType"
        :key="popup.uuid"
        @click="popupStore.deletePopup(popup.uuid)"
        :style="{ bottom: `${10 + index}%` }"
        >{{ popup.text }}</Popup
      >
    </TransitionGroup>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({});
</script>

<style>
.popup-container {
  position: absolute;
  right: 5%;
  bottom: 10%;
  width: 15%;
}

@media (max-width: 768px) {
  .popup-container {
    width: 75%;
  }
}

.popup-enter-active,
.popup-leave-active {
  transition: all 0.35s ease;
}

.popup-enter-from,
.popup-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
