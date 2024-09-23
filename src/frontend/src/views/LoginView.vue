<template>
  <main>
    <div class="container h-80 text-center">
      <div class="row justify-content-around align-items-stretch h-80">
        <div class="col-6">
          <h1>Log in with CSH</h1>
        </div>
        <div class="col-6">
          <h1>Log in with Google</h1>
        </div>
      </div>
      <div class="row justify-content-around align-items-stretch h-80">
        <div class="col-6">
          <div class="login-option align-bottom m-2" @click="redirect('csh')">
            <img src="@/assets/csh.png" alt="CSH Login" />
          </div>
        </div>
        <div class="col-6">
          <div class="login-option align-bottom m-2" @click="redirect('google')">
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
        const response = await fetch('/api/v1/auth/csh/');
        window.location.href = await response.text();
      } else if (provider === 'google') {
        const response = await fetch('/api/v1/auth/google/');
        window.location.href = await response.text();
      }
    }
  }
};
</script>

<style>
main {
  height: 80vh;
}

.login-option img {
  position: absolute;
  top: 50%;
  left: 50%;
  height: 50%;
  width: auto;
  transform: translate(-50%, -50%);
}

.login-option {
  position: relative;
  display: inline-block;
  width: 30vw; /* Adjust size as needed */
  max-width: 20em;
  height: 30vw; /* Adjust size as needed */
  max-height: 20em;
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
