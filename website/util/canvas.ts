let
  canvas: HTMLCanvasElement,
  context: CanvasRenderingContext2D,
  onchange: (imageData: ImageData) => void,
  eventQueue: PointerEvent[] = [],
  points: PointerEvent[] = [],
  pointerId: number,
  isRenderScheduled = false,
  path = new Path2D()

export const bind = (
  element: HTMLCanvasElement,
  callback: (imageData: ImageData) => void,
) => {
  canvas = element
  context = canvas.getContext('2d', { willReadFrequently: true })!
  onchange = callback

  setupContext(context)
  addPointerListeners(canvas)
}

export const erase = () => {
  path = new Path2D()
  context.clearRect(0, 0, canvas.width, canvas.height)
}

const setupContext = (context: CanvasRenderingContext2D) => {
  context.strokeStyle = '#000000'
  context.lineWidth = 20
  context.lineCap = 'round'
  context.lineJoin = 'round'
  context.filter = 'blur(1px)'
  context.scale(2, 2)
}

const addPointerListeners = (canvas: HTMLCanvasElement) => {
  canvas.addEventListener('pointerdown', (event) => {
    canvas.setPointerCapture(event.pointerId)

    pointerId = event.pointerId

    eventQueue.push(event)

    scheduleRender()
  })

  canvas.addEventListener('pointermove', (event) => {
    if (canvas.hasPointerCapture(event.pointerId)) {
      const
        coalesced =
          event.hasOwnProperty('getCoalescedEvents')
            ? event.getCoalescedEvents()
            : [],
        events = coalesced.length ? coalesced : [event]

      eventQueue.push(...events)

      scheduleRender()
    }
  })

  canvas.addEventListener('pointerup', (event) => {
    if (canvas.hasPointerCapture(event.pointerId)) {
      canvas.releasePointerCapture(event.pointerId)

      eventQueue.push(event)

      scheduleRender()
    }
  })
}

const scheduleRender = () => {
  if (!isRenderScheduled) {
    isRenderScheduled = true

    requestAnimationFrame(() => {
      isRenderScheduled = false

      render()
    })
  }
}

const render = () => {
  for (const event of eventQueue) {
    if (event.type === 'pointerdown') {
      path.moveTo(
        event.offsetX,
        event.offsetY,
      )
      points.push(event)
    } else if (
      Math.hypot(
        event.offsetX - points.at(-1).offsetX,
        event.offsetY - points.at(-1).offsetY,
      ) > 5
    ) {
      const mp = {
        x: 0.5 * (points.at(-1).offsetX + event.offsetX),
        y: 0.5 * (points.at(-1).offsetY + event.offsetY),
      }

      path.quadraticCurveTo(
        points.at(-1).offsetX,
        points.at(-1).offsetY,
        mp.x,
        mp.y,
      )
      points.push(event)
    }
  }

  context.stroke(path)

  eventQueue = []

  if (!canvas.hasPointerCapture(pointerId)) {
    onchange(context.getImageData(0, 0, canvas.width, canvas.height))
  }
}
