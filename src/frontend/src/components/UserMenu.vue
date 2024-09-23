<template>
  <ul v-if="userData" class="nav navbar-nav ml-auto">
    <div class="nav-item navbar-user dropdown">
      <a
        class="nav-link dropdown-toggle"
        id="userDropdownLink"
        data-bs-toggle="dropdown"
        aria-expanded="false"
      >
        <img :src="userData.picture" /> {{ userData.given_name }} {{ userData.family_name }}
        <span class="caret"></span>
      </a>
      <div class="dropdown-menu" aria-labelledby="userDropdownLink">
        <a class="dropdown-item" href="https://github.com/jabbate19/rideboard-v2/issues"
          >Report an Issue</a
        >
        <a class="dropdown-item" @click="logout()">Logout</a>
      </div>
    </div>
  </ul>
</template>

<script lang="ts" setup>
import { useAuthStore } from '@/stores/auth';

import { computed } from 'vue';
const authStore = useAuthStore();

const userData = computed(() => authStore.user!);

const logout = async () => {
  await fetch('/api/v1/auth/logout', { method: 'POST' });
  window.location.href = '/login';
};
</script>
