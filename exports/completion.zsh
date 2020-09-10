#compdef sane-fmt

autoload -U is-at-least

_sane-fmt() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'--details=[File diff detail]: :(count name diff)' \
'--color=[When to use terminal color]: :(auto never always)' \
'--log-format=[Format of log messages]: :(human github-actions)' \
'-I+[Files whose contents contain paths to target files (`-` means stdin, other strings mean text file)]' \
'--include=[Files whose contents contain paths to target files (`-` means stdin, other strings mean text file)]' \
'--stdio[Reads unformatted code from standard input, prints formatted code to standard output, then exits]' \
'-w[Whether to write or check]' \
'--write[Whether to write or check]' \
'--hide-passed[Do not log passed filenames]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::files -- Files to process:_files' \
&& ret=0
    
}

(( $+functions[_sane-fmt_commands] )) ||
_sane-fmt_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'sane-fmt commands' commands "$@"
}

_sane-fmt "$@"