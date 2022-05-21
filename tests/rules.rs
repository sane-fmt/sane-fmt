#![cfg(test)]
pub mod utils;
pub use utils::*;

test_rule!(
    prefer_single_quotes,
    "ts",
    "const a = 'hello world'\n",
    &["const a = \"hello world\"\n"]
);

test_rule!(
    template_string,
    "ts",
    text_block! {
        "const a = 123"
        "const b = 456"
        "const c = 789"
        "const abc = `a ${a} b ${b} c ${c}`"
        ""
    },
    &[]
);

test_rule!(
    allow_double_quotes_in_some_cases,
    "ts",
    "const a = \"'hello world'\"\n",
    &["const a = '\\'hello world\\''\n"]
);

test_rule!(
    trailing_commas,
    "js",
    text_block! {
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
    &[text_block! {
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
    }]
);

test_rule!(
    do_not_use_parentheses_for_arrow_function_with_one_parameter,
    "js",
    "const fn = x => x * x\n",
    &["const fn = (x) => x * x\n"]
);

test_rule!(
    allow_using_parentheses_for_arrow_function_with_one_parameter_when_there_is_type,
    "ts",
    "const fn = (x: number) => x * x\n",
    &[]
);

test_rule!(
    allow_using_parentheses_for_arrow_function_with_multiple_parameters,
    "js",
    "const fn = (a, b, c) => a * b + c\n",
    &[]
);

test_rule!(
    interface,
    "ts",
    text_block! {
        "export interface MyInterface {"
        "  readonly a: number"
        "  readonly b: string"
        "  c: number"
        "  d: string"
        "}"
        ""
    },
    &[text_block! {
        "export interface MyInterface {"
        "  readonly a: number;"
        "  readonly b: string;"
        "  c: number;"
        "  d: string;"
        "}"
        ""
    }]
);

test_rule!(
    object_literal_type,
    "ts",
    text_block! {
        "export type MyObject = {"
        "  readonly a: number"
        "  readonly b: string"
        "  c: number"
        "  d: string"
        "}"
        ""
    },
    &[text_block! {
        "export type MyObject = {"
        "  readonly a: number;"
        "  readonly b: string;"
        "  c: number;"
        "  d: string;"
        "}"
        ""
    }]
);

test_rule!(
    separator_between_properties_of_inline_object_literal_type,
    "ts",
    "export type MyObject = { foo: number, bar: string }\n",
    &[]
);

test_rule!(
    multi_line_union_or_intersection,
    "ts",
    text_block! {
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
    &[
        text_block! {
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
        text_block! {
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
        text_block! {
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
        text_block! {
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
    ]
);

test_rule!(
    multi_line_union_of_multi_line_object,
    "ts",
    text_block! {
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
    &[text_block! {
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
    }]
);

test_rule!(
    module_sort_import_declarations,
    "ts",
    text_block! {
        "import {} from 'ABC'"
        "import {} from 'DEF'"
        "import {} from 'abc'"
        "import {} from 'def'"
        "import {} from '../lib'"
        "import {} from '../utils'"
        ""
    },
    &[text_block! {
        "import {} from 'ABC'"
        "import {} from 'abc'"
        "import {} from 'DEF'"
        "import {} from 'def'"
        "import {} from '../utils'"
        "import {} from '../lib'"
        ""
    }]
);

test_rule!(
    module_sort_export_declarations,
    "ts",
    text_block! {
        "export {} from 'ABC'"
        "export {} from 'DEF'"
        "export {} from 'abc'"
        "export {} from 'def'"
        ""
    },
    &[text_block! {
        "export {} from 'ABC'"
        "export {} from 'abc'"
        "export {} from 'DEF'"
        "export {} from 'def'"
        ""
    }]
);

test_rule!(
    quote_props,
    "ts",
    text_block! {
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
    &[
        text_block! {
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
        text_block! {
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
    ]
);

test_rule!(
    ignore_node,
    "js",
    text_block! {
        "// sane-fmt-ignore"
        "const a = 123;"
        ""
    },
    &[]
);

test_rule!(
    ignore_file,
    "js",
    text_block! {
        "// sane-fmt-ignore-file"
        "const a = 123;"
        ""
    },
    &[]
);
