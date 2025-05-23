<script lang="ts">
  import DemoWidgetDebugCanvas from './DemoWidgetDebugCanvas.svelte'
  import DemoWidgetDebugNeuron from './DemoWidgetDebugNeuron.svelte'

  let { data, isVisible = $bindable() } = $props()
</script>

<aside class="debug">
  {#if data && !isVisible}
    <button
      class="open"
      onclick={() => isVisible = true}
    >
      DEBUG
    </button>
  {/if}
  <button
    class="close"
    onclick={() => isVisible = false}
  >&times;</button>
  <ul class="thumbnails">
    {#each data.thumbnails as [title, imageData]}
      <li>
        <figure>
          <figcaption>{title}</figcaption>
          <DemoWidgetDebugCanvas imageData={imageData} />
        </figure>
      </li>
    {/each}
  </ul>
  <figure>
    <figcaption>SOFTMAX</figcaption>
    <ul class="neurons">
      {#each data.softmax as value, i}
        <li>
          <DemoWidgetDebugNeuron value={value} caption={i} />
        </li>
      {/each}
    </ul>
  </figure>
</aside>

<style>
  .debug {
    position: absolute;
    inset: 1rem 0 1rem auto;
    padding: 1rem;

    border-left: none;
    border-radius: 0 0.5rem 0.5rem 0;

    font-size: 0.7rem;

    > .open {
      position: absolute;
      inset: 1rem auto auto 100%;
      padding: 0.5rem 0.3rem 0.5rem 0.1rem;

      border-left: none;
      border-radius: 0 0.5rem 0.5rem 0;

      writing-mode: vertical-rl;
    }

    & > .close {
      position: absolute;
      inset: 0.5rem 0.5rem auto auto;

      border: none;

      font-size: 1.2rem;
    }

    figcaption {
      margin: 1.6rem 0 0.4rem;

      text-transform: uppercase;
    }

    > .thumbnails {
      display: flex;
      gap: 0.7rem;
    }

    .neurons {
      display: flex;
      gap: 0.3rem;

      text-align: center;
    }
  }
</style>
