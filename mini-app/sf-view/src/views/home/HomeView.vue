<template>
  <div class="home">
    <HelloWorld msg="Работаем над разделом" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import HelloWorld from '@/components/HelloWorld.vue';

export default defineComponent({
  name: 'HomeView',
  components: {
    HelloWorld,
  },
});
</script>
