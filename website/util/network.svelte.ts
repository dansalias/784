interface State {
  modelBytesTotal: number
  modelBytesReceived: number
  input: number[]
  prediction: number
  debug: {
    thumbnails: [string, ImageData][]
    softmax: number[]
  }
}

const network = new Worker(
  new URL('../workers/network.ts', import.meta.url),
  { type: 'module' },
)

export const connect = (state: State) => {
  network.onmessage = (message) => {
    switch (message.data.type) {
      case 'loadStart':
        state.modelBytesTotal = message.data.modelBytesTotal
        break

      case 'loadProgress':
        state.modelBytesReceived = message.data.modelBytesReceived
        break

      case 'prediction':
        state.prediction = message.data.prediction
        state.debug.softmax = message.data.softmax
        break

      default:
        console.error('invalid message', message)
        break
    }
  }

  $effect(() => {
    network.postMessage($state.snapshot(state.input))
  })
}
