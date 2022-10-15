complete -c sane-fmt -l details -d 'File diff detail' -r -f -a "{count	,name	,diff	}"
complete -c sane-fmt -l color -d 'When to use terminal color' -r -f -a "{auto	,never	,always	}"
complete -c sane-fmt -l log-format -d 'Format of log messages' -r -f -a "{human	,github-actions	}"
complete -c sane-fmt -s I -l include -d 'Files whose contents contain paths to target files (`-` means stdin, other strings mean text file)' -r
complete -c sane-fmt -l stdio -d 'Reads unformatted code from standard input, prints formatted code to standard output, then exits'
complete -c sane-fmt -s w -l write -d 'Whether to write or check'
complete -c sane-fmt -l hide-passed -d 'Do not log passed filenames'
complete -c sane-fmt -s h -l help -d 'Print help information (use `--help` for more detail)'
complete -c sane-fmt -s V -l version -d 'Print version information'
