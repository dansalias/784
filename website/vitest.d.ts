import 'vitest'

interface CustomMatchers<R = unknown> {
  toEqualPointerEvents: (e: PointerEvent[]) => R
}

declare module 'vitest' {
  interface Assertion<T = any> extends CustomMatchers<T> {}
  interface AssymetricMatchersContaining extends CustomMatchers {}
}
