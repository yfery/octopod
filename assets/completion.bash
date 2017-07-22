#!bash
# References:
# - https://blog.heckel.xyz/2015/03/24/bash-completion-with-sub-commands-and-dynamic-options/

_octopod()
{
    local cur firstword
    firstword=$(_octopod_get_firstword) 

    GLOBAL_COMMANDS="subscribe unsubscribe list download-dir pending download download-dir help jsonfeed update version"

    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"

    case "${firstword}" in
        download-dir)
            # Special handling: return directories, no space at the end
            compopt -o nospace
            COMPREPLY=( $( compgen -d -S "/" -- "$cur" ) )
            ;;
        subscribe | update)
            compopt +o nospace
            if [[ $cur == -* ]]; then
                COMPREPLY=( $( compgen -W "-d" -- "$cur" ))
            fi
            ;;
        *)
            compopt +o nospace
            COMPREPLY=( $( compgen -W "$GLOBAL_COMMANDS" -- "$cur" ))
            ;;
    esac

    return 0
}

# Determines the first non-option word of the command line. This
# is usually the command
_octopod_get_firstword() {
    local firstword i

    firstword=
    for ((i = 1; i < ${#COMP_WORDS[@]}; ++i)); do
        if [[ ${COMP_WORDS[i]} != -* ]]; then
            firstword=${COMP_WORDS[i]}
            break
        fi
    done

    echo "$firstword"
}

complete -o nospace -F _octopod octopod
