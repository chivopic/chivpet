# chivpet shell integration for fish
# Source this file in ~/.config/fish/config.fish or run: chivpet hook fish | source

function __chivpet_postexec --on-event fish_postexec
    set -l exit_code $status
    chivpet event post-exec "$argv" --exit-code $exit_code >/dev/null 2>&1 &
end
