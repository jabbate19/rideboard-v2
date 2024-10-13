<template>
  <div>
    <h6><span class="badge badge-secondary" v-for="user in users" :key="user">{{ user }}<span>&times;</span></span></h6>
  </div>
  <input
    type="text"
    class="form-control"
    @input="onInput(($event?.target as HTMLTextAreaElement).value)"
    placeholder="Search for a user..."
  />
  <div v-if="loading">Loading...</div>
  <ul v-if="results.length" class="list-group list-group-flush">
    <li
      class="list-group-item list-group-item-action text-truncate"
      v-for="result in results"
      :key="result"
      @click="addUser(result)"
    >
      {{ result }}
    </li>
  </ul>
</template>

<script lang="ts">
import { PopupType } from '@/models';
import { usePopupStore } from '@/stores/popup';
import { defineComponent, type PropType } from 'vue';

export default defineComponent({
  props: {
    modelValue: {
      type: [] as PropType<string[]>,
      required: true
    }
  },
  data() {
    return {
      loading: false,
      timeout: null as number | null,
      users: [] as string[],
      results: [] as string[],
    };
  },
  emits: ['update:modelValue'],
  methods: {
    onInput(value: string) {
      clearTimeout(this.timeout!);
      this.loading = true;

      this.timeout = setTimeout(() => {
        this.fetchResults(value);
      }, 1000);
    },
    async fetchResults(value: string) {
      if (!this.modelValue) {
        this.results = [];
        this.loading = false;
        return;
      }
      const popupStore = usePopupStore();
      try {
        const response = await fetch(
          `/api/v1/chom?search=${value}`
        );
        if (!response.ok) {
          popupStore.addPopup(
            PopupType.Danger,
            `Failed to Get User Suggestions (${response.status})`
          );
        }
        this.results = await response.json();
      } catch (error) {
        console.error('Error fetching data:', error);
        popupStore.addPopup(
          PopupType.Danger,
          'Failed to Get User Suggestions. An unknown error occured.'
        );
        this.results = [];
      } finally {
        this.loading = false;
      }
    },
    addUser(user: string) {
      this.users.push(user);
      this.$emit('update:modelValue', this.users);
    }
  }
});
</script>
