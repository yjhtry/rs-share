<script setup lang="ts">
import { Universe } from '@wasm/adder'

const game = ref('')

const universe = Universe.new()

function renderLoop() {
  game.value = universe.render()
  universe.tick()

  requestAnimationFrame(renderLoop)
}

onMounted(() => {
  renderLoop()
})
</script>

<template>
  {{ game }}
</template>

<style>
body {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
</style>
