<script lang="ts">
  import { onMount } from 'svelte'

  interface Props {
    size: number
  }

  const { size }: Props = $props()

  let canvas

  onMount(() => {
    const context = canvas.getContext('2d')

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

<canvas
  bind:this={canvas}
  width={size}
  height={size}
>
</canvas>

<style>
  canvas {
    position: absolute;
    z-index: -10;
  }
</style>
