let
  canvas: HTMLCanvasElement,
  boundingClientRect: DOMRect,
  context: CanvasRenderingContext2D,
  onchange: (imageData: ImageData) => void,
  eventQueue: PointerEvent[] = [],
  points: PointerEvent[] = [],
  isRenderScheduled = false,
  path = new Path2D()

export const bind = (
  element: HTMLCanvasElement,
  callback: (imageData: ImageData) => void,
) => {
  canvas = element
  boundingClientRect = canvas.getBoundingClientRect()
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

      scheduleRender(true)
    }
  })
}

const scheduleRender = (publish: boolean = false) => {
  if (!isRenderScheduled) {
    isRenderScheduled = true

    requestAnimationFrame(() => {
      isRenderScheduled = false

      render(publish)
    })
  }
}

const render = (publish: boolean) => {
  const offset = {
    x: boundingClientRect.left - window.scrollX,
    y: boundingClientRect.top - window.scrollY,
  }

  for (const event of eventQueue) {
    if (event.type === 'pointerdown') {
      path.moveTo(
        event.clientX - offset.x,
        event.clientY - offset.y,
      )
      points.push(event)
    } else if (
      Math.hypot(
        event.clientX - points.at(-1).clientX,
        event.clientY - points.at(-1).clientY,
      ) > 5
    ) {
      const mp = {
        x: 0.5 * (points.at(-1).clientX + event.clientX) - offset.x,
        y: 0.5 * (points.at(-1).clientY + event.clientY) - offset.y,
      }

      path.quadraticCurveTo(
        points.at(-1).clientX - offset.x,
        points.at(-1).clientY - offset.y,
        mp.x,
        mp.y,
      )
      points.push(event)
    }
  }

  context.stroke(path)

  eventQueue = []

  if (publish) {
    onchange(context.getImageData(0, 0, canvas.width, canvas.height))
  }
}
