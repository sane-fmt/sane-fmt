const quotePreference = 'single quotes'
const doubleQuotesAreSometimesAllowed = "You're welcome"

namespace noTrailingCommasForSingleLine {
  const array = [0, 1, 2, 3, 'a', 'b']
  const object = { abc: 123, def: 456 }
  const arrowFunc = (abc: 123, def: 456) => undefined
  function funcDecl(abc: 123, def: 456) {}
}

namespace trailingCommasForMultiLine {
  const array = [
    'abc',
    'def',
    'ghi',
    'hello',
    'world',
  ]

  const object = {
    abc: 123,
    def: 456,
    ghi: 789,
  }

  const arrowFunc = (
    abc: ['a', 'b', 'c'],
    def: ['d', 'e', 'f'],
    ghi: ['g', 'h', 'i'],
  ) => undefined

  function funcDecl(
    abc: ['a', 'b', 'c'],
    def: ['d', 'e', 'f'],
    ghi: ['g', 'h', 'i'],
  ) {}
}

namespace arrowFunctions {
  const noParens = x => x * x
  const withParens = (x: number) => x * x
  const multiArgs = (a, b, c) => [a, b, c]
}

type MultiLineUnion =
  | { tag: 0; 0: 'zero' }
  | { tag: 1; 1: 'one' }
  | { tag: 2; 2: 'two' }
