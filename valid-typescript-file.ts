export type MyUnion =
  | { type: 0; value: number }
  | { type: 1; value: string }
  | { type: 2; value: symbol }

export type MyIntersection =
  & { a: number }
  & { b: string }
  & { c: symbol }
