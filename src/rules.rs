pub mod cfg;

mod fmt;

pub use cfg::{Configuration as Cfg, ConfigurationBuilder as CfgBuilder};
pub use fmt::Fmt;

use cfg::*;

/// Shape a `ConfigurationBuilder` to desired configuration.
pub fn modify(builder: &mut CfgBuilder) -> &mut CfgBuilder {
    builder
        .deno()
        .line_width(120)
        .quote_style(QuoteStyle::PreferSingle)
        .semi_colons(SemiColons::Asi)
        .type_literal_separator_kind_single_line(SemiColonOrComma::Comma)
        .trailing_commas(TrailingCommas::OnlyMultiLine)
        .arrow_function_use_parentheses(UseParentheses::PreferNone)
        .module_sort_import_declarations(SortOrder::CaseSensitive)
        .module_sort_export_declarations(SortOrder::CaseSensitive)
        .import_declaration_sort_named_imports(SortOrder::Maintain)
        .export_declaration_sort_named_exports(SortOrder::Maintain)
        .jsx_quote_style(JsxQuoteStyle::PreferSingle)
        .jsx_multi_line_parens(false)
        .jsx_element_space_before_self_closing_tag_slash(true)
        .ignore_node_comment_text("sane-fmt-ignore")
        .ignore_file_comment_text("sane-fmt-ignore-file")
}

/// Create desired configuration.
pub fn build_cfg() -> Cfg {
    Cfg {
        indent_width: 2,
        line_width: 120,
        use_tabs: false,
        new_line_kind: NewLineKind::LineFeed,
        quote_style: QuoteStyle::PreferSingle,
        semi_colons: SemiColons::Asi,
        arrow_function_use_parentheses: UseParentheses::PreferNone,
        binary_expression_line_per_expression: false,
        jsx_quote_style: JsxQuoteStyle::PreferSingle,
        jsx_multi_line_parens: false,
        member_expression_line_per_expression: false,
        type_literal_separator_kind_single_line: SemiColonOrComma::Comma,
        type_literal_separator_kind_multi_line: SemiColonOrComma::SemiColon,
        module_sort_import_declarations: SortOrder::CaseSensitive,
        module_sort_export_declarations: SortOrder::CaseSensitive,
        import_declaration_sort_named_imports: SortOrder::Maintain,
        export_declaration_sort_named_exports: SortOrder::Maintain,
        ignore_node_comment_text: "sane-fmt-ignore".to_string(),
        ignore_file_comment_text: "sane-fmt-ignore-file".to_string(),
        arrow_function_brace_position: BracePosition::SameLine,
        class_declaration_brace_position: BracePosition::SameLine,
        class_expression_brace_position: BracePosition::SameLine,
        constructor_brace_position: BracePosition::SameLine,
        do_while_statement_brace_position: BracePosition::SameLine,
        enum_declaration_brace_position: BracePosition::SameLine,
        get_accessor_brace_position: BracePosition::SameLine,
        if_statement_brace_position: BracePosition::SameLine,
        interface_declaration_brace_position: BracePosition::SameLine,
        for_statement_brace_position: BracePosition::SameLine,
        for_in_statement_brace_position: BracePosition::SameLine,
        for_of_statement_brace_position: BracePosition::SameLine,
        function_declaration_brace_position: BracePosition::SameLine,
        function_expression_brace_position: BracePosition::SameLine,
        method_brace_position: BracePosition::SameLine,
        module_declaration_brace_position: BracePosition::SameLine,
        set_accessor_brace_position: BracePosition::SameLine,
        switch_case_brace_position: BracePosition::SameLine,
        switch_statement_brace_position: BracePosition::SameLine,
        try_statement_brace_position: BracePosition::SameLine,
        while_statement_brace_position: BracePosition::SameLine,
        arguments_prefer_hanging: false,
        array_expression_prefer_hanging: false,
        array_pattern_prefer_hanging: false,
        do_while_statement_prefer_hanging: false,
        export_declaration_prefer_hanging: false,
        extends_clause_prefer_hanging: false,
        for_statement_prefer_hanging: false,
        for_in_statement_prefer_hanging: false,
        for_of_statement_prefer_hanging: false,
        if_statement_prefer_hanging: false,
        implements_clause_prefer_hanging: false,
        import_declaration_prefer_hanging: false,
        jsx_attributes_prefer_hanging: false,
        object_expression_prefer_hanging: false,
        object_pattern_prefer_hanging: false,
        parameters_prefer_hanging: false,
        sequence_expression_prefer_hanging: false,
        switch_statement_prefer_hanging: false,
        tuple_type_prefer_hanging: false,
        type_literal_prefer_hanging: false,
        type_parameters_prefer_hanging: false,
        union_and_intersection_type_prefer_hanging: false,
        variable_statement_prefer_hanging: false,
        while_statement_prefer_hanging: false,
        enum_declaration_member_spacing: MemberSpacing::Maintain,
        if_statement_next_control_flow_position: NextControlFlowPosition::SameLine,
        try_statement_next_control_flow_position: NextControlFlowPosition::SameLine,
        binary_expression_operator_position: OperatorPosition::SameLine,
        conditional_expression_operator_position: OperatorPosition::NextLine,
        if_statement_single_body_position: SingleBodyPosition::Maintain,
        for_statement_single_body_position: SingleBodyPosition::Maintain,
        for_in_statement_single_body_position: SingleBodyPosition::Maintain,
        for_of_statement_single_body_position: SingleBodyPosition::Maintain,
        while_statement_single_body_position: SingleBodyPosition::Maintain,
        arguments_trailing_commas: TrailingCommas::OnlyMultiLine,
        parameters_trailing_commas: TrailingCommas::OnlyMultiLine,
        array_expression_trailing_commas: TrailingCommas::OnlyMultiLine,
        array_pattern_trailing_commas: TrailingCommas::OnlyMultiLine,
        enum_declaration_trailing_commas: TrailingCommas::OnlyMultiLine,
        export_declaration_trailing_commas: TrailingCommas::OnlyMultiLine,
        import_declaration_trailing_commas: TrailingCommas::OnlyMultiLine,
        object_pattern_trailing_commas: TrailingCommas::OnlyMultiLine,
        object_expression_trailing_commas: TrailingCommas::OnlyMultiLine,
        tuple_type_trailing_commas: TrailingCommas::OnlyMultiLine,
        type_literal_trailing_commas: TrailingCommas::OnlyMultiLine,
        type_parameters_trailing_commas: TrailingCommas::OnlyMultiLine,
        if_statement_use_braces: UseBraces::WhenNotSingleLine,
        for_statement_use_braces: UseBraces::WhenNotSingleLine,
        for_of_statement_use_braces: UseBraces::WhenNotSingleLine,
        for_in_statement_use_braces: UseBraces::WhenNotSingleLine,
        while_statement_use_braces: UseBraces::WhenNotSingleLine,
        array_expression_prefer_single_line: false,
        array_pattern_prefer_single_line: false,
        arguments_prefer_single_line: false,
        binary_expression_prefer_single_line: false,
        computed_prefer_single_line: false,
        conditional_expression_prefer_single_line: true,
        conditional_type_prefer_single_line: false,
        decorators_prefer_single_line: false,
        export_declaration_prefer_single_line: true,
        for_statement_prefer_single_line: false,
        import_declaration_prefer_single_line: true,
        jsx_attributes_prefer_single_line: false,
        jsx_element_prefer_single_line: false,
        mapped_type_prefer_single_line: false,
        member_expression_prefer_single_line: false,
        object_expression_prefer_single_line: false,
        object_pattern_prefer_single_line: false,
        parameters_prefer_single_line: false,
        parentheses_prefer_single_line: false,
        tuple_type_prefer_single_line: false,
        type_literal_prefer_single_line: false,
        type_parameters_prefer_single_line: false,
        union_and_intersection_type_prefer_single_line: false,
        variable_statement_prefer_single_line: false,
        binary_expression_space_surrounding_bitwise_and_arithmetic_operator: true,
        comment_line_force_space_after_slashes: false,
        construct_signature_space_after_new_keyword: true,
        constructor_space_before_parentheses: false,
        constructor_type_space_after_new_keyword: true,
        do_while_statement_space_after_while_keyword: true,
        export_declaration_space_surrounding_named_exports: true,
        for_statement_space_after_for_keyword: true,
        for_statement_space_after_semi_colons: true,
        for_in_statement_space_after_for_keyword: true,
        for_of_statement_space_after_for_keyword: true,
        function_declaration_space_before_parentheses: false,
        function_expression_space_before_parentheses: false,
        function_expression_space_after_function_keyword: true,
        get_accessor_space_before_parentheses: false,
        if_statement_space_after_if_keyword: true,
        import_declaration_space_surrounding_named_imports: true,
        jsx_expression_container_space_surrounding_expression: false,
        jsx_element_space_before_self_closing_tag_slash: true,
        method_space_before_parentheses: false,
        object_expression_space_surrounding_properties: true,
        object_pattern_space_surrounding_properties: true,
        set_accessor_space_before_parentheses: false,
        static_block_brace_position: BracePosition::SameLineUnlessHanging,
        space_surrounding_properties: true,
        tagged_template_space_before_literal: false,
        type_annotation_space_before_colon: false,
        type_assertion_space_before_expression: true,
        type_literal_space_surrounding_properties: true,
        while_statement_space_after_while_keyword: true,
    }
}

#[test]
fn test_cfg() {
    use pipe_trait::*;
    use pretty_assertions::assert_eq;
    use serde_json::to_value;

    let actual = CfgBuilder::new()
        .pipe_mut(modify)
        .build()
        .pipe_ref(to_value)
        .expect("convert actual to json value");

    let expected = build_cfg()
        .pipe_ref(to_value)
        .expect("convert expected to json value");

    assert_eq!(actual, expected);
}

/// Create a formatter for desired configuration.
pub fn build_fmt() -> Fmt {
    Fmt::from_cfg(build_cfg())
}
