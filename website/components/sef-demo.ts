import { Network, fprop } from '../lib/network'
import { SefComponent } from './sef-component'
import { SefDemoCanvas } from './sef-demo-canvas'

export class SefDemo extends SefComponent {
  private canvas: SefDemoCanvas
  private network: Network

  constructor() {
    super('sef-demo')
  }

  connectedCallback() {
    this.canvas = this.querySelector('sef-demo-canvas')

    this.canvas.addEventListener('drawstop', () => {
      this.showPrediction()
    })

    this.querySelector('button')!.addEventListener('click', () => {
      this.canvas.erase()

      document.getElementById('prediction').innerHTML = ''
    })

    this.loadNetwork()
  }

  private async loadNetwork() {
    const
      response = await fetch('./params.bin'),
      buffer = await response.arrayBuffer(),
      dataview = new DataView(buffer),
      numberOfLayers = Number(dataview.getBigUint64(0, true)),
      structure = Array.from({ length: numberOfLayers }, (_, i) =>
        Number(dataview.getBigUint64(8 * (i + 1), true))
      )

    let byteOffset = 8 * numberOfLayers

    const getNextParam = () => {
      return dataview.getFloat64(byteOffset += 8, true)
    }

    this.network = Array.from({
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

  private async showPrediction() {
    const
      input = this.canvas.getPixels(),
      output = fprop(this.network, input),
      prediction = output.reduce((maxIndex, currentValue, currentIndex) =>
        currentValue > output[maxIndex] ? currentIndex : maxIndex,
      0)

    document.getElementById('prediction')!.innerHTML = prediction.toString()
  }
}
