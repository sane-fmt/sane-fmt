
edit:completion:arg-completer[sane-fmt] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'sane-fmt'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'sane-fmt'= {
            cand --details 'File diff detail'
            cand --color 'When to use terminal color'
            cand --log-format 'Format of log messages'
            cand -I 'Files whose contents contain paths to target files (`-` means stdin, other strings mean text file)'
            cand --include 'Files whose contents contain paths to target files (`-` means stdin, other strings mean text file)'
            cand --stdio 'Reads unformatted code from standard input, prints formatted code to standard output, then exits'
            cand -w 'Whether to write or check'
            cand --write 'Whether to write or check'
            cand --hide-passed 'Do not log passed filenames'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
