<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import UserMenu from './components/UserMenu.vue'
</script>

<template>
  <nav class="navbar fixed-top navbar-expand-lg navbar-dark bg-primary">
    <div class="container">
      <a class="navbar-brand abs" href="/">Rideboard</a>
      <button
        class="navbar-toggler navbar-toggler-right"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#navbarResponsive"
      >
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarResponsive">
        <ul class="navbar-nav mr-auto">
          <RouterLink class="nav-link" to="/history">History</RouterLink>
        </ul>
        <UserMenu />
      </div>
    </div>
  </nav>

  <RouterView :key="$route.path" />
  <footer class="text-center fixed-bottom">
    <a class="text-primary" href="https://github.com/jabbate19/rideboard-v2">Rideboard</a>
  </footer>
</template>

<style>
main {
  margin-top: 1em;
  margin-left: 2em;
  margin-right: 2em;
}

body {
  padding-top: 70px;
}
</style>
