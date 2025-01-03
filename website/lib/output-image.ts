export class OutputImage {
  private canvas: OffscreenCanvas

  constructor(imageData: ImageData) {
    const { width, height } = imageData

    this.canvas = new OffscreenCanvas(width, height)
    this.context.putImageData(imageData, 0, 0)
  }

  get imageData(): ImageData {
    return this.context.getImageData(
      0, 0, this.width, this.height
    )
  }

  getPixels(): number[][] {
    const
      { width, height, data } = this.imageData,
      pixels = Array.from({ length: height }, () => Array.from({ length: width }))

    for (let i = 0; i < data.length; i += 4) {
      const
        row = Math.floor(i / 4 / width),
        col = i / 4 % width,
        [ r, g, b, a ] = data.slice(i, i + 4)

      pixels[row][col] = (255 - (r + g + b) / 3) * (a / 255) / 255
    }

    return pixels as number[][]
  }

  trim(): OutputImage {
    let bounds = {
      left: this.width,
      right: 0,
      top: this.height,
      bottom: 0,
    },
    pixels = this.getPixels()

    for (let y = 0; y < pixels.length; y++) {
      for (let x = 0; x < pixels[0].length; x++) {
        if (pixels[y][x] > 0) {
          bounds = {
            left: Math.min(bounds.left, x),
            right: Math.max(bounds.right, x + 1),
            top: Math.min(bounds.top, y),
            bottom: Math.max(bounds.top, y + 1),
          }
        }
      }
    }

    const
      width = bounds.right - bounds.left,
      height = bounds.bottom - bounds.top,
      size = Math.max(width, height),
      imageData = this.context.getImageData(
        bounds.left, bounds.top, size, size
      )

    this
      .clear()
      .setSize(size, size)
      .context
      .putImageData(imageData, 0, 0)

    return this
  }

  scaleTo(width: number, height: number): OutputImage {
    const
      sw = this.width,
      sh = this.height,
      clone = this.cloneCanvas()

    this
      .clear()
      .setSize(width, height)
      .context
      .drawImage(
        clone,
        0, 0, sw, sh,
        0, 0, width, height,
      )

    return this
  }

  centerIn(width: number, height: number): OutputImage {
    let
      totalMass = 0,
      weightedX = 0,
      weightedY = 0,
      pixels = this.getPixels()

    for (let y = 0; y < pixels.length; y++) {
      for (let x = 0; x < pixels[0].length; x++) {
        const mass = pixels[y][x]

        totalMass += mass
        weightedX += (x + 0.5) * mass
        weightedY += (y + 0.5) * mass
      }
    }

    const
      imageData = this.imageData,
      centerOfMass = {
        x: weightedX / totalMass,
        y: weightedY / totalMass,
      }

    this
      .clear()
      .setSize(width, height)
      .context
      .putImageData(
        imageData,
        width / 2 - centerOfMass.x,
        height / 2 - centerOfMass.y,
      )

    return this
  }

  private get context(): OffscreenCanvasRenderingContext2D {
    return this.canvas.getContext('2d', { willReadFrequently: true })!
  }

  private get width() {
    return this.canvas.width
  }

  private get height() {
    return this.canvas.height
  }

  private clear() {
    this.context.reset()

    return this
  }

  private setSize(width: number, height: number) {
    this.canvas.width = width
    this.canvas.height = height

    return this
  }

  private cloneCanvas(): OffscreenCanvas {
    const
      canvas = new OffscreenCanvas(this.width, this.height),
      context = canvas.getContext('2d')

    context.drawImage(this.canvas, 0, 0)

    return canvas
  }
}
