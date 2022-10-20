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
        .quote_props(QuoteProps::AsNeeded)
        .jsx_quote_style(JsxQuoteStyle::PreferSingle)
        .jsx_multi_line_parens(JsxMultiLineParens::Never)
        .jsx_element_space_before_self_closing_tag_slash(true)
        .static_block_brace_position(BracePosition::SameLineUnlessHanging)
        .ignore_node_comment_text("sane-fmt-ignore")
        .ignore_file_comment_text("sane-fmt-ignore-file")
}

/// Create desired configuration.
pub fn build_cfg() -> Cfg {
    Cfg {
        arguments_prefer_hanging: false,
        arguments_prefer_single_line: false,
        arguments_space_around: false,
        arguments_trailing_commas: TrailingCommas::OnlyMultiLine,
        array_expression_prefer_hanging: false,
        array_expression_prefer_single_line: false,
        array_expression_space_around: false,
        array_expression_trailing_commas: TrailingCommas::OnlyMultiLine,
        array_pattern_prefer_hanging: false,
        array_pattern_prefer_single_line: false,
        array_pattern_space_around: false,
        array_pattern_trailing_commas: TrailingCommas::OnlyMultiLine,
        arrow_function_brace_position: BracePosition::SameLine,
        arrow_function_use_parentheses: UseParentheses::PreferNone,
        binary_expression_line_per_expression: false,
        binary_expression_operator_position: OperatorPosition::SameLine,
        binary_expression_prefer_single_line: false,
        binary_expression_space_surrounding_bitwise_and_arithmetic_operator: true,
        class_declaration_brace_position: BracePosition::SameLine,
        class_expression_brace_position: BracePosition::SameLine,
        comment_line_force_space_after_slashes: false,
        computed_prefer_single_line: false,
        conditional_expression_operator_position: OperatorPosition::NextLine,
        conditional_expression_line_per_expression: true,
        conditional_expression_prefer_single_line: true,
        conditional_type_operator_position: OperatorPosition::NextLine,
        conditional_type_prefer_single_line: false,
        construct_signature_space_after_new_keyword: true,
        constructor_brace_position: BracePosition::SameLine,
        constructor_space_before_parentheses: false,
        constructor_type_space_after_new_keyword: true,
        decorators_prefer_single_line: false,
        do_while_statement_brace_position: BracePosition::SameLine,
        do_while_statement_next_control_flow_position: NextControlFlowPosition::SameLine,
        do_while_statement_prefer_hanging: false,
        do_while_statement_space_after_while_keyword: true,
        do_while_statement_space_around: false,
        enum_declaration_brace_position: BracePosition::SameLine,
        enum_declaration_member_spacing: MemberSpacing::Maintain,
        enum_declaration_trailing_commas: TrailingCommas::OnlyMultiLine,
        export_declaration_force_single_line: false,
        export_declaration_prefer_hanging: false,
        export_declaration_prefer_single_line: true,
        export_declaration_sort_named_exports: SortOrder::Maintain,
        export_declaration_space_surrounding_named_exports: true,
        export_declaration_trailing_commas: TrailingCommas::OnlyMultiLine,
        extends_clause_prefer_hanging: false,
        for_in_statement_brace_position: BracePosition::SameLine,
        for_in_statement_prefer_hanging: false,
        for_in_statement_single_body_position: SingleBodyPosition::Maintain,
        for_in_statement_space_after_for_keyword: true,
        for_in_statement_space_around: false,
        for_in_statement_use_braces: UseBraces::WhenNotSingleLine,
        for_of_statement_brace_position: BracePosition::SameLine,
        for_of_statement_prefer_hanging: false,
        for_of_statement_single_body_position: SingleBodyPosition::Maintain,
        for_of_statement_space_after_for_keyword: true,
        for_of_statement_space_around: false,
        for_of_statement_use_braces: UseBraces::WhenNotSingleLine,
        for_statement_brace_position: BracePosition::SameLine,
        for_statement_prefer_hanging: false,
        for_statement_prefer_single_line: false,
        for_statement_single_body_position: SingleBodyPosition::Maintain,
        for_statement_space_after_for_keyword: true,
        for_statement_space_after_semi_colons: true,
        for_statement_space_around: false,
        for_statement_use_braces: UseBraces::WhenNotSingleLine,
        function_declaration_brace_position: BracePosition::SameLine,
        function_declaration_space_before_parentheses: false,
        function_expression_brace_position: BracePosition::SameLine,
        function_expression_space_after_function_keyword: true,
        function_expression_space_before_parentheses: false,
        get_accessor_brace_position: BracePosition::SameLine,
        get_accessor_space_before_parentheses: false,
        if_statement_brace_position: BracePosition::SameLine,
        if_statement_next_control_flow_position: NextControlFlowPosition::SameLine,
        if_statement_prefer_hanging: false,
        if_statement_single_body_position: SingleBodyPosition::Maintain,
        if_statement_space_after_if_keyword: true,
        if_statement_space_around: false,
        if_statement_use_braces: UseBraces::WhenNotSingleLine,
        ignore_file_comment_text: "sane-fmt-ignore-file".to_string(),
        ignore_node_comment_text: "sane-fmt-ignore".to_string(),
        implements_clause_prefer_hanging: false,
        import_declaration_force_single_line: false,
        import_declaration_prefer_hanging: false,
        import_declaration_prefer_single_line: true,
        import_declaration_sort_named_imports: SortOrder::Maintain,
        import_declaration_space_surrounding_named_imports: true,
        import_declaration_trailing_commas: TrailingCommas::OnlyMultiLine,
        indent_width: 2,
        interface_declaration_brace_position: BracePosition::SameLine,
        jsx_attributes_prefer_hanging: false,
        jsx_attributes_prefer_single_line: false,
        jsx_element_prefer_single_line: false,
        jsx_element_space_before_self_closing_tag_slash: true,
        jsx_expression_container_space_surrounding_expression: false,
        jsx_force_new_lines_surrounding_content: false,
        jsx_multi_line_parens: JsxMultiLineParens::Never,
        jsx_quote_style: JsxQuoteStyle::PreferSingle,
        line_width: 120,
        mapped_type_prefer_single_line: false,
        member_expression_line_per_expression: false,
        member_expression_prefer_single_line: false,
        method_brace_position: BracePosition::SameLine,
        method_space_before_parentheses: false,
        module_declaration_brace_position: BracePosition::SameLine,
        module_sort_export_declarations: SortOrder::CaseSensitive,
        module_sort_import_declarations: SortOrder::CaseSensitive,
        new_line_kind: NewLineKind::LineFeed,
        object_expression_prefer_hanging: false,
        object_expression_prefer_single_line: false,
        object_expression_space_surrounding_properties: true,
        object_expression_trailing_commas: TrailingCommas::OnlyMultiLine,
        object_pattern_prefer_hanging: false,
        object_pattern_prefer_single_line: false,
        object_pattern_space_surrounding_properties: true,
        object_pattern_trailing_commas: TrailingCommas::OnlyMultiLine,
        parameters_prefer_hanging: false,
        parameters_prefer_single_line: false,
        parameters_space_around: false,
        parameters_trailing_commas: TrailingCommas::OnlyMultiLine,
        parentheses_prefer_single_line: false,
        quote_props: QuoteProps::AsNeeded,
        quote_style: QuoteStyle::PreferSingle,
        semi_colons: SemiColons::Asi,
        sequence_expression_prefer_hanging: false,
        set_accessor_brace_position: BracePosition::SameLine,
        set_accessor_space_before_parentheses: false,
        space_surrounding_properties: true,
        static_block_brace_position: BracePosition::SameLineUnlessHanging,
        switch_case_brace_position: BracePosition::SameLine,
        switch_statement_brace_position: BracePosition::SameLine,
        switch_statement_prefer_hanging: false,
        switch_statement_space_around: false,
        tagged_template_space_before_literal: false,
        try_statement_brace_position: BracePosition::SameLine,
        try_statement_next_control_flow_position: NextControlFlowPosition::SameLine,
        tuple_type_prefer_hanging: false,
        tuple_type_prefer_single_line: false,
        tuple_type_space_around: false,
        tuple_type_trailing_commas: TrailingCommas::OnlyMultiLine,
        type_annotation_space_before_colon: false,
        type_assertion_space_before_expression: true,
        type_literal_prefer_hanging: false,
        type_literal_prefer_single_line: false,
        type_literal_separator_kind_multi_line: SemiColonOrComma::SemiColon,
        type_literal_separator_kind_single_line: SemiColonOrComma::Comma,
        type_literal_space_surrounding_properties: true,
        type_literal_trailing_commas: TrailingCommas::OnlyMultiLine,
        type_parameters_prefer_hanging: false,
        type_parameters_prefer_single_line: false,
        type_parameters_trailing_commas: TrailingCommas::OnlyMultiLine,
        union_and_intersection_type_prefer_hanging: false,
        union_and_intersection_type_prefer_single_line: false,
        use_tabs: false,
        variable_statement_prefer_hanging: false,
        variable_statement_prefer_single_line: false,
        while_statement_brace_position: BracePosition::SameLine,
        while_statement_prefer_hanging: false,
        while_statement_single_body_position: SingleBodyPosition::Maintain,
        while_statement_space_after_while_keyword: true,
        while_statement_space_around: false,
        while_statement_use_braces: UseBraces::WhenNotSingleLine,
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
