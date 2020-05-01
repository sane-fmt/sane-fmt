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
    vec![
        "const a = 123",
        "const b = 456",
        "const c = 789",
        "const abc = `a ${a} b ${b} c ${c}`",
        "",
    ]
    .join("\n")
    .as_str(),
    &[]
);

test_rule!(
    allow_double_quotes_in_some_cases,
    "ts",
    "const a = \"'hello world'\"\n",
    &["const a = '\\'hello there\\''\n"]
);

test_rule!(
    trailing_commas,
    "js",
    vec![
        "const array = [",
        "  123,",
        "  456,",
        "  789,",
        "]",
        "",
        "someFunc(",
        "  123,",
        "  123456789,",
        "  'this is a string',",
        ")",
        "",
    ]
    .join("\n")
    .as_str(),
    &[[
        "const array = [",
        "  123,",
        "  456,",
        "  789",
        "]",
        "",
        "someFunc(",
        "  123,",
        "  123456789,",
        "  'this is a string'",
        ")",
        ""
    ]
    .join("\n")
    .as_str()]
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
    vec![
        "export interface MyInterface {",
        "  readonly a: number",
        "  readonly b: string",
        "  c: number",
        "  d: string",
        "}",
        ""
    ]
    .join("\n")
    .as_str(),
    &[[
        "export interface MyInterface {",
        "  readonly a: number;",
        "  readonly b: string;",
        "  c: number;",
        "  d: string;",
        "}",
        ""
    ]
    .join("\n")
    .as_str()]
);

test_rule!(
    object_literal_type,
    "ts",
    vec![
        "export type MyObject = {",
        "  readonly a: number",
        "  readonly b: string",
        "  c: number",
        "  d: string",
        "}",
        ""
    ]
    .join("\n")
    .as_str(),
    &[vec![
        "export type MyObject = {",
        "  readonly a: number;",
        "  readonly b: string;",
        "  c: number;",
        "  d: string;",
        "}",
        ""
    ]
    .join("\n")
    .as_str()]
);

test_rule!(
    separator_between_properties_of_inline_object_literal_type,
    "ts",
    // NOTE: I actually prefer commas, but I have yet find the config key
    "export type MyObject = { foo: number; bar: string }\n",
    &[]
);

test_rule!(
    multi_line_union_or_intersection,
    "ts",
    vec![
        "export type MyUnion =",
        "  | { type: 0; value: number }",
        "  | { type: 1; value: string }",
        "  | { type: 2; value: symbol }",
        "",
        // issue: https://github.com/dprint/dprint/issues/192
        // "type MyIntersection =",
        // "  & { a: number }",
        // "  & { b: number }",
        // "  & { c: number }",
        // "",
    ]
    .join("\n")
    .as_str(),
    &[
        vec![
            "export type MyUnion =",
            "  { type: 0; value: number }",
            "  | { type: 1; value: string }",
            "  | { type: 2; value: symbol }",
            "",
        ]
        .join("\n")
        .as_str(),
        vec![
            "export type MyUnion =",
            "  { type: 0; value: number } |",
            "  { type: 1; value: string } |",
            "  { type: 2; value: symbol }",
            "",
        ]
        .join("\n")
        .as_str()
    ]
);

test_rule!(
    multi_line_union_of_multi_line_object,
    "ts",
    vec![
        "export type MyUnion =",
        "  | {",
        "    readonly type: 0",
        "    readonly value: number",
        "  }",
        "  | {",
        "    readonly type: 1",
        "    readonly value: string",
        "  }",
        "  | {",
        "    readonly type: 2",
        "    readonly value: symbol",
        "  }",
        "",
    ]
    .join("\n")
    .as_str(),
    &[vec![
        "export type MyUnion =",
        "  {",
        "    readonly type: 0",
        "    readonly value: number",
        "  }",
        "  | {",
        "    readonly type: 1",
        "    readonly value: string",
        "  }",
        "  | {",
        "    readonly type: 2",
        "    readonly value: symbol",
        "  }",
        "",
    ]
    .join("\n")
    .as_str()]
);
