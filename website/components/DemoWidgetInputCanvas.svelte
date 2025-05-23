<script lang="ts">
  import { onMount } from 'svelte'
  import { bind, erase } from '../util/canvas'
  import DemoWidgetInputCanvasBackground from './DemoWidgetInputCanvasBackground.svelte'

  const
    size = 281,
    canvasWorker = new Worker(
      new URL('../workers/canvas.ts', import.meta.url),
      { type: 'module' },
    )

  let
    canvas,
    {
      pixels = $bindable(),
      debug = $bindable(),
    } = $props()

  onMount(() => {
    bind(
      canvas,
      (imageData) => {
        canvasWorker.postMessage(imageData)

        canvasWorker.onmessage = (message) => {
          pixels = message.data.pixels
          debug = message.data.debug
        }
      }
    )
  })

  export { erase }
</script>

<div>
  <DemoWidgetInputCanvasBackground size={size} />
  <canvas
    bind:this={canvas}
    width={size * 2}
    height={size * 2}
    style="width: {size}px; height: {size}px;"
  >
  </canvas>
</div>

<style>
  div {
    position: relative;
    isolation: isolate;
  }

  canvas {
    touch-action: none;
    user-select: none;
  }
</style>
