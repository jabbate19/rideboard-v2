<template>
    <div class="card mb-3">
        <div class="card-body">
        <h5 class="card-title">{{ card.name }}</h5>
        <h6 class="card-time">{{ startTime }}</h6>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { type Event } from '@/models';
import { format } from 'date-fns';

export default defineComponent({
    props: {
        card: Object as Event,
    },
    computed: {
        startTime() {
            let data = this.card.start_time.toLocaleString();
            return format(data, 'MM/dd/yyyy HH:mm a')
        }
    }
})
</script>

<style scoped>
.card {
  cursor: pointer;
}
</style>