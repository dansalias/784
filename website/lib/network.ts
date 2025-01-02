export interface Neuron {
  weights: number[]
  bias: number
}

export type Network = Array<Array<Neuron>>

export const fprop = (network: Network, input: number[]): number[] => {
  return network.reduce<number[]>((
    input, layer
  ) => {
    return layer.map(neuron => {
      let activation = neuron
        .weights
        .map((w, i) => w * input[i])
        .reduce((sum, el) => sum + el, 0)
        + neuron.bias

      return Math.max(0, activation)
    })
  }, input)
}
