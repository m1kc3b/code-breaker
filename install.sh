#!/bin/bash

set -e  # Arrêter le script en cas d'erreur

# Nom du dépôt et du binaire
GITHUB_REPO="m1kc3b/code-breaker"  # Remplacez par votre repo GitHub
BINARY_NAME="code-breaker"         # Nom du fichier binaire
INSTALL_DIR="/usr/local/bin"       # Dossier d'installation
RESULT_DIR="$HOME/.code-breaker"   # Dossier de sauvegarde

# Vérifier si `jq` est installé
if ! command -v jq &> /dev/null; then
    echo "Erreur : 'jq' n'est pas installé. Installez-le avec 'sudo apt install jq' (Debian/Ubuntu) ou 'sudo yum install jq' (RHEL)."
    exit 1
fi

# Détecter la dernière version publiée sur GitHub
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$GITHUB_REPO/releases/latest" | jq -r '.tag_name')

if [[ -z "$LATEST_RELEASE" || "$LATEST_RELEASE" == "null" ]]; then
    echo "❌ Erreur : Impossible de récupérer la dernière version."
    exit 1
fi

echo "✅ Dernière version trouvée : $LATEST_RELEASE"

# Construire l'URL du binaire à télécharger
BINARY_URL="https://github.com/$GITHUB_REPO/releases/download/$LATEST_RELEASE/$BINARY_NAME"

echo "🔽 Téléchargement de $BINARY_NAME depuis $BINARY_URL..."
if ! curl -L -o "$BINARY_NAME" "$BINARY_URL"; then
    echo "❌ Erreur : Échec du téléchargement."
    exit 1
fi

# Vérifier si le fichier a bien été téléchargé
if [[ ! -f "$BINARY_NAME" ]]; then
    echo "❌ Erreur : Le fichier binaire n'a pas été trouvé après le téléchargement."
    exit 1
fi

# Rendre le fichier exécutable
chmod +x "$BINARY_NAME"

# Déplacer vers le dossier d'installation
echo "🚀 Installation de $BINARY_NAME dans $INSTALL_DIR..."
sudo mv "$BINARY_NAME" "$INSTALL_DIR/"

# Créer le fichier de sauvegarde
if [ ! -d "$RESULT_DIR" ]; then
    echo "📁 Création du dossier de sauvegarde $RESULT_DIR..."
    mkdir "$RESULT_DIR"
fi

chmod 700 "$RESULT_DIR"
touch "$RESULT_DIR/results.txt"

# Vérifier si le dossier est dans le PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "➕ Ajout de $INSTALL_DIR au PATH..."
    echo 'export PATH="$INSTALL_DIR:$PATH"' >> ~/.bashrc
    echo "source ~/.bashrc"
fi

echo "🎉 Installation terminée ! Essayez d'exécuter '$BINARY_NAME --help' pour tester."
