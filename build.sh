#!/bin/bash

# export signing key
export TAURI_SIGNING_PRIVATE_KEY="$HOME/.tauri/ai_nav.key"
# optionally also add a password
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""

cargo tauri build 
