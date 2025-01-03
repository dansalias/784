import { expect, test } from 'vitest'
import { OutputImage } from './output-image'

// 0 = white
// 1 = black
const testImage = [
  [ 0, 0, 0, 0, 0, 0 ],
  [ 0, 0, 0, 0, 0, 0 ],
  [ 0, 0, 0, 0, 1, 1 ],
  [ 0, 0, 0, 0, 1, 1 ],
  [ 0, 0, 0, 0, 1, 1 ],
  [ 0, 0, 0, 0, 1, 1 ],
]

const testImageData = new ImageData(
  new Uint8ClampedArray(testImage.map(row =>
    Array.from(row).map(
      pixel => Array(3).fill(255 * (1 - pixel)).concat(pixel ? 255 : 0)
    )
  ).flat(2)),
  testImage[0].length,
  testImage.length,
)

test('get pixels', () => {
  expect(new OutputImage(testImageData).getPixels()).toEqual(testImage)
})

test('trim', () => {
  expect(new OutputImage(testImageData).trim().getPixels()).toEqual([
    [ 1, 1, 0, 0 ],
    [ 1, 1, 0, 0 ],
    [ 1, 1, 0, 0 ],
    [ 1, 1, 0, 0 ],
  ])
})

test('scaleTo', () => {
  expect(new OutputImage(testImageData).scaleTo(3, 3).getPixels()).toEqual([
    [ 0, 0, 0 ],
    [ 0, 0, 1 ],
    [ 0, 0, 1 ],
  ])
})

test('centerIn', () => {
  expect(new OutputImage(testImageData).centerIn(8, 8).getPixels()).toEqual([
    [ 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 0, 0, 0, 1, 1, 0, 0, 0 ],
    [ 0, 0, 0, 1, 1, 0, 0, 0 ],
    [ 0, 0, 0, 1, 1, 0, 0, 0 ],
    [ 0, 0, 0, 1, 1, 0, 0, 0 ],
    [ 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 0, 0, 0, 0, 0, 0, 0, 0 ],
  ])
})
