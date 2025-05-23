interface Neuron {
  weights: number[]
  bias: number
}

let model: Neuron[][] = []

onmessage = (message: MessageEvent<number[]>) => postMessage({
    type: 'prediction',
    prediction: getPrediction(message.data),
    softmax: softmax(fprop(message.data)).map(v => Math.round(v * 100) / 100),
  })

const loadModel = async () => {
  const
    response = await fetch('/params.bin'),
    reader = response.body.getReader(),
    data: Uint8Array[] = [],
    modelBytesTotal = parseInt(response.headers.get('content-length'))

  let modelBytesReceived = 0

  postMessage({
    type: 'loadStart',
    modelBytesTotal,
  })

  while (true) {
    const { done, value } = await reader.read()

    if (done) {
      break
    }

    modelBytesReceived += value.byteLength

    postMessage({
      type: 'loadProgress',
      modelBytesReceived,
    })

    data.push(value)
  }

  const
    buffer = await new Blob(data).arrayBuffer(),
    dataview = new DataView(buffer),
    numberOfLayers = Number(dataview.getBigUint64(0, true)),
    structure = Array.from({ length: numberOfLayers }, (_, i) =>
      Number(dataview.getBigUint64(8 * (i + 1), true))
    )

  let byteOffset = 8 * numberOfLayers

  const getNextParam = () => {
    return dataview.getFloat64(byteOffset += 8, true)
  }

  model = Array.from({
    length: numberOfLayers - 1,
  }, (_, layerIndex) =>
    Array.from({ length: structure[layerIndex + 1] }, () => ({
      weights: Array.from({ length: structure[layerIndex] }, () =>
        getNextParam()
      ),
      bias: getNextParam(),
    }))
  )
}

const getPrediction = (input: number[]): number =>
  getMaxIndex(fprop(input))

const getMaxIndex = (values: number[]): number =>
  values.reduce((maxIndex, currentValue, currentIndex) =>
      currentValue > values[maxIndex] ? currentIndex: maxIndex, 0
    )

const fprop = (input: number[]): number[] =>
  model.reduce<number[]>(
    (input, layer) =>
      layer.map(neuron => {
        let activation = neuron
          .weights
          .map((w, i) => w * input[i])
          .reduce((sum, el) => sum + el, 0)
          + neuron.bias

        return Math.max(0, activation)
      }),
    input,
  )

const softmax = (values: number[]): number[] => {
  const
    exponentials = values.map(Math.exp),
    sum = exponentials.reduce((sum, v) => sum + v, 0)

  return exponentials.map(v => v / sum)
}

loadModel()
