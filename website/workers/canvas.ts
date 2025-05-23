import { OutputImage } from '../util/output-image'

onmessage = (message: MessageEvent<ImageData>) => {
  const
    trimmed = new OutputImage(message.data).trim(),
    scaled = trimmed.clone().scaleTo(20, 20),
    centered = scaled.clone().centerIn(28, 28),
    pixels = centered.clone().getPixels().flat()

  postMessage({
    pixels,
    debug: {
      thumbnails: [
        ['trimmed',  trimmed.imageData],
        ['scaled',   scaled.imageData],
        ['centered', centered.imageData],
      ],
    },
  })
}
