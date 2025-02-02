#!/bin/bash

set -e  # Arrêter le script en cas d'erreur

# Nom du dépôt et du binaire
GITHUB_REPO="m1kc3b/code-breaker"  # Remplacez par votre repo GitHub
BINARY_NAME="code_breaker"           # Nom du fichier binaire
INSTALL_DIR="/usr/local/bin"           # Dossier d'installation

# Détecter la dernière version publiée sur GitHub
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$GITHUB_REPO/releases/latest" | grep -oP '"tag_name": "\K(.*?)(?=")')

if [[ -z "$LATEST_RELEASE" ]]; then
    echo "Erreur : Impossible de récupérer la dernière version."
    exit 1
fi

echo "Dernière version trouvée : $LATEST_RELEASE"

# Construire l'URL du binaire à télécharger
BINARY_URL="https://github.com/$GITHUB_REPO/releases/download/$LATEST_RELEASE/$BINARY_NAME"

echo "Téléchargement de $BINARY_NAME depuis $BINARY_URL..."
curl -L -o "$BINARY_NAME" "$BINARY_URL"

# Rendre le fichier exécutable
chmod +x "$BINARY_NAME"

# Déplacer vers le dossier d'installation
echo "Installation de $BINARY_NAME dans $INSTALL_DIR..."
sudo mv "$BINARY_NAME" "$INSTALL_DIR/"

# Vérifier si le PATH contient bien le dossier
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "Ajout de $INSTALL_DIR au PATH..."
    echo 'export PATH="$INSTALL_DIR:$PATH"' >> ~/.bashrc
    echo "Redémarrez votre terminal ou exécutez 'source ~/.bashrc' pour appliquer les modifications."
fi

echo "Installation terminée ! Essayez d'exécuter '$BINARY_NAME --help' pour tester."
