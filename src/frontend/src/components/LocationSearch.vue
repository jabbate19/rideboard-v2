<template>
  <input
    type="text"
    class="form-control"
    :value="modelValue"
    @input="onInput(($event?.target as HTMLTextAreaElement).value)"
    placeholder="Search for a location..."
  />
  <div v-if="loading">Loading...</div>
  <ul v-if="results.length" class="list-group list-group-flush">
    <li
      class="list-group-item list-group-item-action text-truncate"
      v-for="result in results"
      :key="result.place_id"
      @click="setLocation(result.display_name)"
    >
      {{ result.display_name }}
    </li>
  </ul>
</template>

<script lang="ts">
import { PopupType, type Place } from '@/models';
import { usePopupStore } from '@/stores/popup';
import { defineComponent, type PropType } from 'vue';

export default defineComponent({
  props: {
    modelValue: {
      type: String as PropType<string>,
      required: true
    }
  },
  data() {
    return {
      loading: false,
      timeout: null as number | null,
      results: [] as Place[]
    };
  },
  emits: ['update:modelValue'],
  methods: {
    onInput(value: string) {
      this.$emit('update:modelValue', value);
      clearTimeout(this.timeout!);
      this.loading = true;

      this.timeout = setTimeout(() => {
        this.fetchResults();
      }, 1000); // 1 second idle time
    },
    async fetchResults() {
      if (!this.modelValue) {
        this.results = [];
        this.loading = false;
        return;
      }
      const popupStore = usePopupStore();
      try {
        const response = await fetch(
          `https://nominatim.openstreetmap.org/search?q=${encodeURIComponent(this.modelValue)}&format=jsonv2`
        );
        if (!response.ok) {
          popupStore.addPopup(
            PopupType.Danger,
            `Failed to Get Location Suggestions (${response.status})`
          );
        }
        this.results = await response.json();
      } catch (error) {
        console.error('Error fetching data:', error);
        popupStore.addPopup(
          PopupType.Danger,
          'Failed to Get Location Suggestions. An unknown error occured.'
        );
        this.results = [];
      } finally {
        this.loading = false;
      }
    },
    setLocation(displayName: string) {
      this.results = [];
      this.$emit('update:modelValue', displayName);
    }
  }
});
</script>
