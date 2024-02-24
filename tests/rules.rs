#![cfg(test)]
pub mod utils;
pub use utils::*;

use text_block_macros::text_block;

test_rule! {
    name: prefer_single_quotes,
    ext: "ts",
    from: "const a = \"hello world\"\n",
    into: "const a = 'hello world'\n",
}

test_rule! {
    name: template_string,
    ext: "ts",
    into: text_block! {
        "const a = 123"
        "const b = 456"
        "const c = 789"
        "const abc = `a ${a} b ${b} c ${c}`"
        ""
    },
}

test_rule! {
    name: allow_double_quotes_in_some_cases,
    ext: "ts",
    from: "const a = '\\'hello world\\''\n",
    into: "const a = \"'hello world'\"\n",
}

test_rule! {
    name: trailing_commas,
    ext: "js",
    from: text_block! {
        "const array = ["
        "  123,"
        "  456,"
        "  789"
        "]"
        ""
        "someFunc("
        "  123,"
        "  123456789,"
        "  'this is a string'"
        ")"
        ""
    },
    into: text_block! {
        "const array = ["
        "  123,"
        "  456,"
        "  789,"
        "]"
        ""
        "someFunc("
        "  123,"
        "  123456789,"
        "  'this is a string',"
        ")"
        ""
    },
}

test_rule! {
    name: do_not_use_parentheses_for_arrow_function_with_one_parameter,
    ext: "js",
    from: "const fn = (x) => x * x\n",
    into: "const fn = x => x * x\n",
}

test_rule! {
    name: allow_using_parentheses_for_arrow_function_with_one_parameter_when_there_is_type,
    ext: "ts",
    into: "const fn = (x: number) => x * x\n",
}

test_rule! {
    name: allow_using_parentheses_for_arrow_function_with_multiple_parameters,
    ext: "js",
    into: "const fn = (a, b, c) => a * b + c\n",
}

test_rule! {
    name: no_space_before_opening_parenthesis_in_function_declaration,
    ext: "js",
    into: text_block! {
        "function main() {"
        "  console.log('hello world')"
        "}"
        ""
    },
}

test_rule! {
    name: interface,
    ext: "ts",
    from: text_block! {
        "export interface MyInterface {"
        "  readonly a: number;"
        "  readonly b: string;"
        "  c: number;"
        "  d: string;"
        "}"
        ""
    },
    into: text_block! {
        "export interface MyInterface {"
        "  readonly a: number"
        "  readonly b: string"
        "  c: number"
        "  d: string"
        "}"
        ""
    },
}

test_rule! {
    name: object_literal_type,
    ext: "ts",
    from: text_block! {
        "export type MyObject = {"
        "  readonly a: number;"
        "  readonly b: string;"
        "  c: number;"
        "  d: string;"
        "}"
        ""
    },
    into: text_block! {
        "export type MyObject = {"
        "  readonly a: number"
        "  readonly b: string"
        "  c: number"
        "  d: string"
        "}"
        ""
    },
}

test_rule! {
    name: separator_between_properties_of_inline_object_literal_type,
    ext: "ts",
    into: "export type MyObject = { foo: number, bar: string }\n",
}

test_rule! {
    name: multi_line_union_or_intersection,
    ext: "ts",
    from: text_block! {
        "export type MyUnion ="
        "  { type: 0; value: number }"
        "  | { type: 1; value: string }"
        "  | { type: 2; value: symbol }"
        ""
        "type MyIntersection ="
        "  & { a: number }"
        "  & { b: number }"
        "  & { c: number }"
        ""
    },
    from: text_block! {
        "export type MyUnion ="
        "  { type: 0; value: number } |"
        "  { type: 1; value: string } |"
        "  { type: 2; value: symbol }"
        ""
        "type MyIntersection ="
        "  & { a: number }"
        "  & { b: number }"
        "  & { c: number }"
        ""
    },
    from: text_block! {
        "export type MyUnion ="
        "  | { type: 0, value: number }"
        "  | { type: 1, value: string }"
        "  | { type: 2, value: symbol }"
        ""
        "type MyIntersection ="
        "  { a: number }"
        "  & { b: number }"
        "  & { c: number }"
        ""
    },
    from: text_block! {
        "export type MyUnion ="
        "  | { type: 0, value: number }"
        "  | { type: 1, value: string }"
        "  | { type: 2, value: symbol }"
        ""
        "type MyIntersection = { a: number }"
        "  & { b: number }"
        "  & { c: number }"
        ""
    },
    into: text_block! {
        "export type MyUnion ="
        "  | { type: 0, value: number }"
        "  | { type: 1, value: string }"
        "  | { type: 2, value: symbol }"
        ""
        "type MyIntersection ="
        "  & { a: number }"
        "  & { b: number }"
        "  & { c: number }"
        ""
    },
}

test_rule! {
    name: multi_line_union_of_multi_line_object,
    ext: "ts",
    from: text_block! {
        "export type MyUnion ="
        "  {"
        "    readonly type: 0"
        "    readonly value: number"
        "  }"
        "  | {"
        "    readonly type: 1"
        "    readonly value: string"
        "  }"
        "  | {"
        "    readonly type: 2"
        "    readonly value: symbol"
        "  }"
        ""
    },
    into: text_block! {
        "export type MyUnion ="
        "  | {"
        "    readonly type: 0"
        "    readonly value: number"
        "  }"
        "  | {"
        "    readonly type: 1"
        "    readonly value: string"
        "  }"
        "  | {"
        "    readonly type: 2"
        "    readonly value: symbol"
        "  }"
        ""
    },
}

test_rule! {
    name: module_sort_import_declarations,
    ext: "ts",
    from: text_block! {
        "import {} from 'ABC'"
        "import {} from 'abc'"
        "import {} from 'DEF'"
        "import {} from 'def'"
        "import {} from '../utils'"
        "import {} from '../lib'"
        ""
    },
    into: text_block! {
        "import {} from 'ABC'"
        "import {} from 'DEF'"
        "import {} from 'abc'"
        "import {} from 'def'"
        "import {} from '../lib'"
        "import {} from '../utils'"
        ""
    },
}

test_rule! {
    name: module_sort_export_declarations,
    ext: "ts",
    from: text_block! {
        "export {} from 'ABC'"
        "export {} from 'abc'"
        "export {} from 'DEF'"
        "export {} from 'def'"
        ""
    },
    into: text_block! {
        "export {} from 'ABC'"
        "export {} from 'DEF'"
        "export {} from 'abc'"
        "export {} from 'def'"
        ""
    },
}

test_rule! {
    name: quote_props,
    ext: "ts",
    from: text_block! {
        "interface Foo {"
        "  noQuotes: 123"
        "  'unneededQuotes': 456"
        "  'needed quotes': 789"
        "}"
        ""
        "const foo: Foo = {"
        "  noQuotes: 123,"
        "  'unneededQuotes': 456,"
        "  'needed quotes': 789,"
        "}"
        ""
    },
    from: text_block! {
        "interface Foo {"
        "  noQuotes: 123"
        "  \"unneededQuotes\": 456"
        "  \"needed quotes\": 789"
        "}"
        ""
        "const foo: Foo = {"
        "  noQuotes: 123,"
        "  \"unneededQuotes\": 456,"
        "  \"needed quotes\": 789,"
        "}"
        ""
    },
    into: text_block! {
        "interface Foo {"
        "  noQuotes: 123"
        "  unneededQuotes: 456"
        "  'needed quotes': 789"
        "}"
        ""
        "const foo: Foo = {"
        "  noQuotes: 123,"
        "  unneededQuotes: 456,"
        "  'needed quotes': 789,"
        "}"
        ""
    },
}

test_rule! {
    name: ignore_node,
    ext: "js",
    into: text_block! {
        "// sane-fmt-ignore"
        "const a = 123;"
        ""
    },
}

test_rule! {
    name: ignore_file,
    ext: "js",
    into: text_block! {
        "// sane-fmt-ignore-file"
        "const a = 123;"
        ""
    },
}
