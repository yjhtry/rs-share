<script setup lang="ts">
const props = defineProps<{
  initial: number
}>()

const { count, inc, dec } = useCounter(props.initial)
</script>

<template>
  <div class="space-x-2">
    {{ count }}
    <button class="inc btn" @click="inc()">
      +
    </button>
    <button class="dec btn" @click="dec()">
      -
    </button>
  </div>
</template>
