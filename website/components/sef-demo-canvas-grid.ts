import { SefComponent } from './sef-component'

export class SefDemoCanvasGrid extends SefComponent {
  private canvas: HTMLCanvasElement
  private context: CanvasRenderingContext2D

  constructor() {
    super('sef-demo-canvas-grid')
  }

  connectedCallback() {
    this.canvas = this.querySelector('#canvas')
    this.context = this.canvas.getContext('2d')
    this.drawGrid()
  }

  private drawGrid() {
    for (let x = 0; x <= 280; x += 10) {
      this.context.moveTo(0.5 + x, 0)
      this.context.lineTo(0.5 + x, 280)
    }

    for (let y = 0; y <= 280; y += 10) {
      this.context.moveTo(0, 0.5 + y)
      this.context.lineTo(280, 0.5 + y)
    }

    this.context.strokeStyle = '#e9e9e9'
    this.context.stroke()
  }
}
