<template>
  <main>
    <div class="container h-100 text-center">
      <div class="row justify-content-around align-items-center h-100">
        <div class="mx-3">
          <h1>Log in with CSH</h1>
          <div class="login-option" @click="redirect('csh')">
            <img src="@/assets/csh.png" alt="CSH Login" />
          </div>
        </div>
        <div class="mx-3">
          <h1>Log in with Google</h1>
          <div class="login-option" @click="redirect('google')">
            <img src="@/assets/google.png" alt="Google Login" />
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<script lang="ts">
export default {
  methods: {
    async redirect(provider: string) {
      if (provider === 'csh') {
        const response = await fetch('/api/v1/auth/csh/')
        window.location.href = await response.text()
      } else if (provider === 'google') {
        const response = await fetch('/api/v1/auth/google/')
        window.location.href = await response.text()
      }
    }
  }
}
</script>

<style>
main {
  height: 100vh;
}

.login-option img {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 50%;
  height: auto;
}

.login-option {
  position: relative;
  display: inline-block;
  width: 20em; /* Adjust size as needed */
  height: 20em; /* Adjust size as needed */
  border-radius: 50%;
  background-color: rgb(183, 183, 183);
  border: 2px solid rgb(183, 183, 183);
  overflow: hidden;
  transition:
    background-color 0.3s,
    transform 0.3s; /* Smooth transition */
}

.login-option:hover {
  background-color: #f0f0f0; /* Highlight color */
  transform: scale(1.05); /* Optional: slightly enlarge the item */
}
</style>
