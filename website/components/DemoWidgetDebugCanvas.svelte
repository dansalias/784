<script lang="ts">
  const { imageData } = $props()

  let canvas

  $effect(() => {
    const size = Math.max(imageData.width, imageData.height)

    canvas.width = size
    canvas.height = size
    canvas.getContext('2d').putImageData(imageData, 0, 0)
  })
</script>

<canvas
  class="debug-canvas"
  bind:this={canvas}
>
</canvas>

<style>
  .debug-canvas {
    width: 4.4rem;
    height: 4.4rem;

    background: var(--white);
  }
</style>
