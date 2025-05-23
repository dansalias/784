<script lang="ts">
  import { numToHex } from '../util/color'

  const
    { value, caption } = $props(),
    bg = numToHex(value),
    fg = value > 0.5 ? '#ffffff' : '#000000'
</script>

<figure class="debug-neuron">
  <div
    class="value"
    style:color={fg}
    style:background-color={bg}
  >
    {value}
  </div>
  <figcaption
    class="caption"
  >
    {caption}
  </figcaption>
</figure>

<style>
  .debug-neuron {
    > .value {
      width: 1.2rem;
      height: 1.2rem;

      background: white;
      border-radius: 100%;
      font-size: 0.5rem;
      line-height: 1.2rem;
    }

    > .caption {
      opacity: 0.4;
    }
  }
</style>
