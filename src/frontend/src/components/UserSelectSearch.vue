<template>
  <div>
    <h6>
      <span class="badge badge-secondary" v-for="user in users" :key="user.id"
        >{{ user.name }} ({{ user.email }})<span class="removeUser ml-1" @click="removeUser(user)"
          >&times;</span
        ></span
      >
    </h6>
  </div>
  <input
    type="text"
    class="form-control"
    v-model="query"
    @input="onInput(($event?.target as HTMLTextAreaElement).value)"
    placeholder="Search for a user..."
  />
  <div v-if="loading">Loading...</div>
  <ul v-if="results.length" class="list-group list-group-flush">
    <li
      class="list-group-item list-group-item-action text-truncate"
      v-for="result in results"
      :key="result.id"
      @click="addUser(result)"
    >
      {{ result.name }} ({{ result.email }})
    </li>
  </ul>
</template>

<script lang="ts">
import { PopupType, type UserStub } from '@/models';
import { usePopupStore } from '@/stores/popup';
import { defineComponent } from 'vue';

export default defineComponent({
  props: {
    modelValue: {
      type: Array<UserStub>,
      required: true
    }
  },
  data() {
    return {
      query: '',
      loading: false,
      timeout: null as number | null,
      users: this.modelValue as UserStub[],
      results: [] as UserStub[]
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
        const response = await fetch(`/api/v1/user/?query=${value}`);
        if (!response.ok) {
          popupStore.addPopup(
            PopupType.Danger,
            `Failed to Get User Suggestions (${response.status})`
          );
        }
        let data: UserStub[] = await response.json();
        this.results = data.filter((user) => !this.users.map((x) => x.id).includes(user.id));
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
    addUser(user: UserStub) {
      this.users.push(user);
      this.query = '';
      this.results = [];
      this.$emit('update:modelValue', this.users);
    },
    removeUser(user: UserStub) {
      this.users.splice(
        this.users.findIndex((x) => x === user),
        1
      );
      this.$emit('update:modelValue', this.users);
    }
  }
});
</script>

<style scoped>
.removeUser {
  cursor: pointer;
}
</style>
