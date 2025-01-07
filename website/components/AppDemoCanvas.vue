<template>
  <div class="flex gap-4 justify-center items-center">
    <div class="relative">
      <app-demo-canvas-grid />
      <canvas
        width="562" height="562"
        class="absolute inset-0 z-10 select-none touch-none w-[281px] h-[281px]"
        ref="canvas"
        @pointerdown="onPointerDown"
        @pointermove="onPointerMove"
        @pointerup="onPointerUp"
      ></canvas>
    </div>
    <div v-if="state.showDebug">Output:
      <canvas
        ref="debug"
        width="28"
        height="28"
        class="w-64 h-64 bg-white border border-red-400"
        style="image-rendering: pixelated;"
      ></canvas>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, ref, Ref } from 'vue'
  import { useState } from '../app'
  import { OutputImage } from '../lib/output-image'
  import AppDemoCanvasGrid from './AppDemoCanvasGrid.vue'

  interface Point {
    x: number
    y: number
  }

  const
    state = useState(),
    canvas: Ref<HTMLCanvasElement> = ref(null),
    debug: Ref<HTMLCanvasElement> = ref(null),
    points: Point[][] = [],
    emit = defineEmits(['drawstop']),
    pixels = defineModel()

  let
    context: CanvasRenderingContext2D = null,
    path: Path2D = new Path2D()

  onMounted(() => {
    context = canvas.value.getContext('2d', { willReadFrequently: true })
    context.scale(2, 2)
    context.strokeStyle = '#000000'
    context.lineWidth = 20
    context.lineCap = 'round'
    context.lineJoin = 'round'
    context.filter = 'blur(1px)'
  })

  const getPoint = (event: PointerEvent): Point => {
    const rect = canvas.value.getBoundingClientRect()

    return {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top,
    }
  }

  const getDistance = (p1: Point, p2: Point): number =>
    Math.sqrt((p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2)

  const getMidpoint = (p1: Point, p2: Point): Point => ({
    x: p1.x + (p2.x - p1.x) / 2,
    y: p1.y + (p2.y - p1.y) / 2,
  })

  const clear = () => {
    context.clearRect(0, 0, canvas.value.width, canvas.value.height)
  }

  const draw = () => {
    requestAnimationFrame(() => {
      clear()

      for (const segment of points) {
        path.moveTo(segment[0].x, segment[0].y)

        if (segment.length === 1) {
          path.lineTo(segment[0].x, segment[0].y)
        } else {
          for (let i = 1; i < segment.length - 1; i += 1) {
            const
              p1 = segment[i],
              p2 = segment[i + 1],
              mp = getMidpoint(p1, p2)

            path.quadraticCurveTo(p1.x, p1.y, mp.x, mp.y)
          }
        }
      }

      context.stroke(path)
    })
  }

  const erase = () => {
    points.length = 0
    path = new Path2D()
    pixels.value = []
    clear()
  }

  const onPointerDown = (event: PointerEvent) => {
    canvas.value.setPointerCapture(event.pointerId)

    points.push([ getPoint(event) ])

    draw()
  }

  const onPointerMove = (event: PointerEvent) => {
    if (canvas.value.hasPointerCapture(event.pointerId)) {
      const
        segment = points.at(-1),
        point = getPoint(event)

      if (getDistance(segment.at(-1), point) > 3) {
        segment.push(point)

        draw()
      }
    }
  }

  const onPointerUp = (event: PointerEvent) => {
    console.log('pointer up')
    if (canvas.value.hasPointerCapture(event.pointerId)) {
      canvas.value.releasePointerCapture(event.pointerId)

      const outputImage = new OutputImage(
          context.getImageData(0, 0, canvas.value.width, canvas.value.height)
        )
        .trim()
        .scaleTo(20, 20)
        .centerIn(28, 28)

      if (state.showDebug) {
        debug.value.getContext('2d').putImageData(outputImage.imageData, 0, 0)
      }

      emit('drawstop')

      pixels.value = outputImage.getPixels().flat()
    }
  }

  defineExpose({
    erase,
  })
</script>
