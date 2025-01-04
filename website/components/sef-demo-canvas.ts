import { OutputImage } from '../lib/output-image'
import { SefComponent } from './sef-component'

type Point = {
  x: number
  y: number
}

export class SefDemoCanvas extends SefComponent {
  private canvas: HTMLCanvasElement
  private context: CanvasRenderingContext2D
  private points: Point[][] = []
  private path: Path2D = new Path2D()

  constructor() {
    super('sef-demo-canvas')
  }

  connectedCallback() {
    this.canvas = this.querySelector('#canvas')
    this.context = this.canvas.getContext('2d', { willReadFrequently: true })

    this.setContext()
    this.createPointerListeners()
  }

  erase() {
    this.points = []
    this.path = new Path2D()
    this.clear()
  }

  getPixels() {
    const output = new OutputImage(
        this.context.getImageData(0, 0, this.canvas.width, this.canvas.height)
      )
      .trim()
      .scaleTo(20, 20)
      .centerIn(28, 28)

    this.debug(output)

    return output
      .getPixels()
      .flat()
  }

  private setContext() {
    this.context.scale(2, 2)
    this.context.strokeStyle = '#000000'
    this.context.lineWidth = 20
    this.context.lineCap = 'round'
    this.context.lineJoin = 'round'
    this.context.filter = 'blur(1px)'
  }

  private createPointerListeners() {
    this.addEventListener('pointerdown', (event) => {
      this.setPointerCapture(event.pointerId)

      this.points.push([
        this.getPoint(event),
      ])

      this.drawPath()
    })

    this.addEventListener('pointermove', (event) => {
      if (this.hasPointerCapture(event.pointerId)) {
        const
          points = this.points.at(-1),
          point = this.getPoint(event)

        if (this.getDistance(points.at(-1), point) > 3) {
          points.push(point)

          this.drawPath()
        }
      }
    })

    this.addEventListener('pointerup', (event) => {
      if (this.hasPointerCapture(event.pointerId)) {
        this.releasePointerCapture(event.pointerId)

        this.dispatchEvent(new CustomEvent('drawstop', {
          bubbles: true,
        }))
      }
    })
  }

  private getPoint(event: PointerEvent): Point {
    const rect = this.getBoundingClientRect()

    return {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top,
    }
  }

  private drawPath() {
    requestAnimationFrame(() => {
      this.clear()

      for (const points of this.points) {
        this.path.moveTo(points[0].x, points[0].y)

        if (points.length === 1) {
          this.path.lineTo(points[0].x, points[0].y)
        } else {
          for (let i = 1; i < points.length - 1; i += 1) {
            const
              p1 = points[i],
              p2 = points[i + 1],
              mp = this.getMidpoint(p1, p2)

            this.path.quadraticCurveTo(p1.x, p1.y, mp.x, mp.y)
          }
        }
      }

      this.context.stroke(this.path)
    })
  }

  private clear() {
    this.context.clearRect(0, 0, this.canvas.width, this.canvas.height)
  }

  private getMidpoint(p1: Point, p2: Point): Point {
    return {
      x: p1.x + (p2.x - p1.x) / 2,
      y: p1.y + (p2.y - p1.y) / 2,
    }
  }

  private getDistance(p1: Point, p2: Point): number {
    return Math.sqrt((p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2)
  }

  private debug(outputImage: OutputImage) {
    const debug = document.getElementById('debug-canvas') as HTMLCanvasElement

    debug.getContext('2d').putImageData(outputImage.imageData, 0, 0)
  }
}
