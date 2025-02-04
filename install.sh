#!/bin/bash

# Define variables
REPO="m1kc3b/code-breaker"
EXEC_NAME="code-breaker"

# 1. Fetch the latest version from GitHub releases
API_URL="https://api.github.com/repos/$REPO/releases/latest"
LATEST_RELEASE=$(curl -s "$API_URL")
LATEST_VERSION=$(echo "$LATEST_RELEASE" | jq -r '.tag_name')

# 2. Check if a version is already installed
if command -v "$EXEC_NAME" &> /dev/null; then
  INSTALLED_VERSION="$("$EXEC_NAME" --version 2>/dev/null | grep -oE '[0-9.]+$')" # Extract version number
else
  INSTALLED_VERSION=""
fi

# 3. Compare versions
if [[ -z "$INSTALLED_VERSION" ]]; then
  echo "No version installed. Installing latest version: $LATEST_VERSION"
elif [[ "$INSTALLED_VERSION" == "$LATEST_VERSION" ]]; then
  echo "✅ Latest version ($LATEST_VERSION) already installed."
  exit 0
else
  echo " Installed version: $INSTALLED_VERSION, latest version: $LATEST_VERSION"
  read -p "❓ Do you want to update to the latest version (yes/no)? " UPDATE
  if [[ "$UPDATE" != "yes" ]]; then
    echo " Update cancelled."
    exit 0
  fi
fi

# 4. Download and install the latest version
RELEASE_URL="https://github.com/m1kc3b/code-breaker/releases/download/$LATEST_VERSION/$EXEC_NAME"
echo "🔽 Downloading version $LATEST_VERSION..."
curl -L -o "$EXEC_NAME" "$RELEASE_URL"

# Check if the download was successful
if [[ $? -ne 0 ]]; then
  echo "❌ Error: Failed to download the executable."
  exit 1
fi

# Set execution permissions
chmod +x "$EXEC_NAME"

# Install
echo "🚀 Installing $EXEC_NAME in /usr/local/bin..."
sudo mv "$EXEC_NAME" "/usr/local/bin/$EXEC_NAME"

# Create a hidden directory and results.txt file
HIDDEN_DIR="$HOME/.code-breaker"
mkdir -p "$HIDDEN_DIR"
touch "$HIDDEN_DIR/results.txt"
echo "📁 Creating the backup folder..."

echo "🎉 Installation completed. Run '$EXEC_NAME' to start the application."