<template>
  <canvas id="canvas" width="281" height="281" ref="canvas"></canvas>
</template>

<script setup lang="ts">
  import { onMounted, ref, Ref } from 'vue'

  const canvas: Ref<HTMLCanvasElement> = ref(null)

  onMounted(() => {
    const context = canvas.value.getContext('2d') 

    for (let x = 0; x <= 280; x += 10) {
      context.moveTo(0.5 + x, 0)
      context.lineTo(0.5 + x, 280)
    }

    for (let y = 0; y <= 280; y += 10) {
      context.moveTo(0, 0.5 + y)
      context.lineTo(280, 0.5 + y)
    }

    context.strokeStyle = '#e9e9e9'
    context.stroke()
  })
</script>
