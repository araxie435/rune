#!/bin/bash

if [[ $UID != 0 ]]; then
    echo "This script must be run as root!"
    exit 1
fi

echo "Hello from runeinstall.sh"

echo "Installing Rune..."

PROFILE_CODE=$(cat <<'EOF'

# -- Added by rune installer script --
if [[ -n $USER ]]; then
    UserPath="/home/$USER/.local/bin"

    if [[ ! -d /home/$USER/.local/bin ]]; then
        mkdir -p /home/$USER/.local/bin
    fi

    GroupList=""
    
    for Group in $(id -Gn "$USER"); do
        GroupPath="/usr/local/groups/$Group/bin"

        if [[ ! -d "$GroupPath" ]]; then
            mkdir -p "$GroupPath"
        fi

        if [[ "$PATH" != *"$GroupPath"* ]]; then
            GroupList="$GroupList:$GroupPath"
        fi
    done

    if [[ "$PATH" != *"$UserPath"* ]] && [[ ":$PATH:" != *"$GroupList"* ]]; then
        PATH="$UserPath:$GroupList:$PATH"
    fi
    
    export PATH
fi
# -- Added by rune installer script --

EOF
)

if grep -q "rune installer script" /etc/profile; then
    echo "Rune profile code already exists in /etc/profile, skipping..."
else
    echo "$PROFILE_CODE" >> /etc/profile
fi

# Here will be installation of rune binary and config generation
REPOSITORY="https://github.com/araxie/rune/releases/latest/download/"
FILE="rune-linux-amd64-bin"
TEMP_DIR=$(mktemp -d)

echo "Downloading Rune binary..."
curl -L "${REPOSITORY}${FILE}" -o "${TEMP_DIR}/rune"
chmod 555 "${TEMP_DIR}/rune"
mv "${TEMP_DIR}/rune" /bin/rune

echo "Rune installation completed."
exit 0