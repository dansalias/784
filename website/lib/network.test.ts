import { expect, test } from 'bun:test'
import { fprop } from './network'

test('fprop', () => {
  expect(fprop([
    [
      {
        weights: [ 0.2, 0.3 ],
        bias: 0.0,
      },
      {
        weights: [ 0.2, 0.3 ],
        bias: -1.0,
      },
    ]
  ], [0.5, 0.5])).toBe([0.25, 0.0])
})
