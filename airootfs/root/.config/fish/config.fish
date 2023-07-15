set -gx EDITOR kak
set -gx LESS -FR
set -gx BAT_PAGER less

if status is-login
    and test -z "$DISPLAY" -a "$XDG_VTNR" = 1
    exec startx
end


alias cp 'cp -i'
alias mv 'mv -i'
alias ip 'ip --color=auto'
abbr --add -g md mkdir

# theme
set -Ux fish_color_command 005fd7
set -Ux fish_color_comment red
set -Ux fish_color_end 009900
set -Ux fish_color_error ff0000
set -Ux fish_color_param 00afff
set -Ux fish_color_quote 999900
set -Ux fish_color_redirection 00afff

# thanks I hate it
function fish_command_not_found
	__fish_default_command_not_found_handler $argv
end
