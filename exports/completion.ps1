
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'sane-fmt' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'sane-fmt'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'sane-fmt' {
            [CompletionResult]::new('--details', 'details', [CompletionResultType]::ParameterName, 'File diff detail')
            [CompletionResult]::new('--color', 'color', [CompletionResultType]::ParameterName, 'When to use terminal color')
            [CompletionResult]::new('--log-format', 'log-format', [CompletionResultType]::ParameterName, 'Format of log messages')
            [CompletionResult]::new('-I', 'I', [CompletionResultType]::ParameterName, 'Files whose contents contain paths to target files (`-` means stdin, other strings mean text file)')
            [CompletionResult]::new('--include', 'include', [CompletionResultType]::ParameterName, 'Files whose contents contain paths to target files (`-` means stdin, other strings mean text file)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--stdio', 'stdio', [CompletionResultType]::ParameterName, 'Reads unformatted code from standard input, prints formatted code to standard output, then exits')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Whether to write or check')
            [CompletionResult]::new('--write', 'write', [CompletionResultType]::ParameterName, 'Whether to write or check')
            [CompletionResult]::new('--hide-passed', 'hide-passed', [CompletionResultType]::ParameterName, 'Do not log passed filenames')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
