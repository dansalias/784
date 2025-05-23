interface Bounds {
  lower: number
  upper: number
}

export const numToHex = (
  value: number,
  bounds: Bounds = {
    lower: 0,
    upper: 1,
  },
): string => '#' + Math
    .round((bounds.upper - value) * 255)
    .toString(16)
    .padStart(2, '0')
    .repeat(3)
