<template>
    <ul v-if="userData" class="nav navbar-nav ml-auto">
        <li class="nav-item navbar-user dropdown">
            <a class="nav-link dropdown-toggle" data-toggle="dropdown">
                <img :src="userData.picture"> {{ userData.given_name }} {{ userData.family_name }}
                <span class="caret"></span>
            </a>
            <div class="dropdown-menu">
                <a class="dropdown-item" href="https://github.com/jabbate19/rideboard-v2/issues">Report an Issue</a>
                <a class="dropdown-item" @click="logout()">Logout</a>
            </div>
        </li>
    </ul>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

interface UserData {
  type: string;
  given_name: string;
  family_name: string;
  preferred_username: string | undefined;
  picture: string | undefined;
}

export default defineComponent({
  data() {
    return {
      userData: null as UserData | null, // to store API data
    };
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      try {
        await fetch('/api/v1/auth/').then(async response => {
            if (response.status != 200) {
                throw Error("Bad Return Code");
            }
            let jsonData: UserData = await response.json()
            return jsonData
        }).then(jsonData => {
            if (jsonData.type == "CSH") {
                jsonData.picture = `https://profiles.csh.rit.edu/image/${jsonData.preferred_username}`
            }
            this.userData = jsonData
        });
      } catch (error) {
        console.error('Error fetching data:', error);
        this.userData = null;
      }
    },
    async logout() {
        let response = await fetch('/api/v1/auth/logout', {method: 'POST'});
        window.location.href = "/login";
    }
  },
});
</script>

<style>
.show > .dropdown-menu {
  max-height: 800px;
  visibility: visible;
}
  
.dropdown-menu {
  display: block;
  max-height: 0;
  visibility: hidden;
  transition: all 0.4s ease-in-out;
  overflow: hidden;
}
</style>