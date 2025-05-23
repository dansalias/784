import { expect } from 'vitest'

expect.extend({
  toEqualPointerEvents(received: PointerEvent[], expected: PointerEvent[]) {
    const { isNot } = this

    return {
      pass:
        received.length === expected.length &&
        received.every((e, i) =>
          e.type === expected[i].type &&
          e.x === expected[i].x &&
          e.y === expected[i].y &&
          e.movementX === expected[i].movementX &&
          e.movementY === expected[i].movementY
      ),
      message: () => `${ received } pointer events ${ isNot ? '' : 'don\'t' } match ${ expected }`,
    }
  },
})
