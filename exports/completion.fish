complete -c sane-fmt -n "__fish_use_subcommand" -l details -d 'File diff detail' -r -f -a "count name diff"
complete -c sane-fmt -n "__fish_use_subcommand" -l color -d 'When to use terminal color' -r -f -a "auto never always"
complete -c sane-fmt -n "__fish_use_subcommand" -l log-format -d 'Format of log messages' -r -f -a "human github-actions"
complete -c sane-fmt -n "__fish_use_subcommand" -s I -l include -d 'Files whose contents contain paths to target files (`-` means stdin, other strings mean text file)'
complete -c sane-fmt -n "__fish_use_subcommand" -l stdio -d 'Reads unformatted code from standard input, prints formatted code to standard output, then exits'
complete -c sane-fmt -n "__fish_use_subcommand" -s w -l write -d 'Whether to write or check'
complete -c sane-fmt -n "__fish_use_subcommand" -l hide-passed -d 'Do not log passed filenames'
complete -c sane-fmt -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c sane-fmt -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
