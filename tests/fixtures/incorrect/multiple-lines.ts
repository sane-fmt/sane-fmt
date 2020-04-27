export const a = 'correct'
export const b = 'correct'
export const c = "incorrect"
export const d = 'correct'
export const e = "incorrect";
export const f =   'incorrect'
export const g = 'correct'

interface Func {
  (x: number): number;
}

export function func (): Func {
  return x => x * x
}
