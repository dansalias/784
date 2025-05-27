interface Neuron {
  weights: number[]
  bias: number
}

type Model = Neuron[][]

interface Message {
  model: Model
  input: number[]
}

onmessage = (message: MessageEvent<Message>) => {
  const
    { model, input } = message.data,
    logits = fprop(model, input),
    prediction = getMaxIndex(logits),
    output = softmax(logits).map(v => Math.round(v * 100) / 100)

  postMessage({
    type: 'prediction',
    prediction,
    softmax: output,
  })
}

const getMaxIndex = (values: number[]): number =>
  values.reduce((maxIndex, currentValue, currentIndex) =>
      currentValue > values[maxIndex] ? currentIndex: maxIndex, 0
    )

const fprop = (model: Model, input: number[]): number[] =>
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
