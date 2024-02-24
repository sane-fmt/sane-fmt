
use builtin;
use str;

set edit:completion:arg-completer[sane-fmt] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'sane-fmt'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
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
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
