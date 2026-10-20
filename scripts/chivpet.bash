# chivpet shell integration for bash
# Source this file in your ~/.bashrc or run: eval "$(chivpet hook bash)"

_chivpet_precmd() {
    local exit_code=$?
    local last_cmd=$(HISTTIMEFORMAT= history 1 | sed 's/^[ ]*[0-9]*[ ]*//')
    (chivpet event post-exec "$last_cmd" --exit-code "$exit_code" >/dev/null 2>&1 &)
}

if [[ ! "$PROMPT_COMMAND" =~ "_chivpet_precmd" ]]; then
    PROMPT_COMMAND="_chivpet_precmd${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
fi
