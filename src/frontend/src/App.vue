<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <nav class="navbar fixed-top navbar-expand-lg navbar-dark bg-primary">
    <div class="container">
      <a class="navbar-brand abs" href="/">Rideboard</a>
      <button class="navbar-toggler navbar-toggler-right" type="button" data-toggle="collapse"
              data-target="#navbarResponsive">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarResponsive">
        <ul class="navbar-nav mr-auto">
          <RouterLink class="nav-link" to="/history">History</RouterLink>
        </ul>
        <ul class="nav navbar-nav ml-auto">
          <li class="nav-item navbar-user dropdown">
            <a class="nav-link dropdown-toggle" data-toggle="dropdown" href="#" id="user01">
              <img src=''> First Last
              <span class="caret"></span>
            </a>
            <div class="dropdown-menu" aria-labelledby="user01">
              <!-- <div class="dropdown-divider"></div> -->
              <a class="dropdown-item" href="/logout">Logout</a>
            </div>
          </li>
        </ul>
      </div>
    </div>
  </nav>

  <RouterView />
</template>

<style>
main {
  margin-top: 75px;
  margin-left: 150px;
  margin-right: 150px;
}
</style>