# chivpet shell integration for zsh
# Source this file in your ~/.zshrc or run: eval "$(chivpet hook zsh)"

_chivpet_preexec() {
    # Non-blocking event dispatch to avoid delaying interactive shell
    (chivpet event pre-exec "$1" >/dev/null 2>&1 &)
}

_chivpet_precmd() {
    local exit_code=$?
    local last_cmd=""
    if [[ -n "$history" ]]; then
        last_cmd="$history[$((HISTCMD-1))]"
    fi
    (chivpet event post-exec "$last_cmd" --exit-code "$exit_code" >/dev/null 2>&1 &)
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _chivpet_preexec
add-zsh-hook precmd _chivpet_precmd
