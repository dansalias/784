<script lang="ts">
  import DemoWidgetDebug from './DemoWidgetDebug.svelte'
  import DemoWidgetEraseButton from './DemoWidgetEraseButton.svelte'
  import DemoWidgetInputCanvas from './DemoWidgetInputCanvas.svelte'
  import DemoWidgetStatus from './DemoWidgetStatus.svelte'

  let
    predict,
    inputCanvas,
    pixels = $state.raw([]),
    modelBytesTotal = $state(0),
    modelBytesReceived = $state(0),
    model = $state.raw([]),
    prediction = $state(null),
    debug = $state({
      thumbnails: [
        [ 'trimmed',  new ImageData(1, 1) ],
        [ 'scaled',   new ImageData(1, 1) ],
        [ 'centered', new ImageData(1, 1) ],
      ],
      softmax: [],
    }),
    modelHasLoaded = $derived(
      modelBytesReceived > 0 &&
      modelBytesReceived === modelBytesTotal
    ),
    showDebug = $state(false)

  new Worker(
    new URL('../workers/network.load.ts', import.meta.url),
    { type: 'module' },
  ).addEventListener('message', (message) => {
    switch (message.data.type) {
      case 'loadStart':
        modelBytesTotal = message.data.modelBytesTotal
        break

      case 'loadProgress':
        modelBytesReceived = message.data.modelBytesReceived
        break

      case 'loadEnd':
        model = message.data.model
        break

      default:
        console.error('invalid message', message)
        break
    }
  })

  $effect(() => {
    if (predict) {
      predict.terminate()
    }

    // taking a long time...
    predict = new Worker(
      new URL('../workers/network.predict.ts', import.meta.url),
      { type: 'module' },
    )

    predict.addEventListener('message', (message) => {
      prediction = message.data.prediction
      debug.softmax = message.data.softmax
    })

    predict.postMessage({
      model,
      input: pixels,
    })
  })
</script>

<section class={[
  'demo-widget',
  showDebug && '-show-debug',
  !modelHasLoaded && '-show-status',
]}>
  <div class="left">
    <article class="content">
      <header>Draw a digit from 0-9:</header>
      <div class="canvas">
        <DemoWidgetInputCanvas
          bind:this={inputCanvas}
          bind:pixels={pixels}
          bind:debug={debug}
        />
      </div>
      <footer>
        <div class="prediction">
          Prediction: <strong>{prediction}</strong>
        </div>
        <DemoWidgetEraseButton onclick={inputCanvas.erase} />
      </footer>
    </article>
    <DemoWidgetStatus
      modelBytesReceived={modelBytesReceived}
      modelBytesTotal={modelBytesTotal}
    />
  </div>
  <div class="right">
    <DemoWidgetDebug
      data={debug}
      bind:isVisible={showDebug}
    />
  </div>
</section>

<style>
  .demo-widget {
    contain: layout;

    .right {
      position: absolute;
      inset: 0;
      z-index: -10;
    }

    .left, .right, :global(.status) {
      transition: transform .4s;
    }

    .content {
      position: relative;
      display: flex;
      flex-direction: column;

      background: var(--white);
      border: 1px solid var(--gray);
      border-radius: 0.75rem;
      box-shadow: var(--shadow);
      overflow: hidden;
    }

    .content > header {
      padding: 0 1rem;

      line-height: 2.8rem;
    }

    .content > .canvas {
      margin: 0 1rem 0.7rem;
    }

    .content > footer {
      display: flex;
      justify-content: space-between;

      background: linear-gradient(
        var(--white) 0% 50%,
        var(--gray-light) 50% 100%
      );

      line-height: 2.6rem;
    }

    .content > footer > .prediction {
      position: relative;
      flex: 1 0 auto;
      padding: 0 1rem;

      background: var(--gray-light);
      border-radius: 0 0.75rem 0 0;
      box-shadow: inset 0 2px 4px 0 #0000001d;
    }

    .content > footer > .prediction::after {
      position: absolute;
      inset: auto -0.5rem 0 auto;
      width: 1rem;
      height: 2.6rem;

      background:
        linear-gradient(
          #ffffff00,
          var(--gray-light) calc(100% - 0.75rem)
        );

      content: '';
      pointer-events: none;
    }

    :global(.debug, .status, .open) {
      position: absolute;
      z-index: -10;

      background: var(--gray-light);
      border: 1px solid var(--gray);
      opacity: 0.8;
    }

    &.-show-debug {
      > .left {
        transform: translateX(-8.2rem);
      }

      > .right {
        transform: translateX(8.2rem);
      }
    }

    &.-show-status {
      :global(.status) {
        transform: translate(0, 1.5rem);
      }
    }
  }

  @media all and (max-width: 37rem) {
    .demo-widget .right {
      display: none;
    }
  }
</style>
