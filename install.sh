#!/bin/bash

set -e  # Stoppe le script en cas d'erreur

# Définir le nom du package
PACKAGE_NAME="mon_app_cli"
DEB_FILE="target/debian/${PACKAGE_NAME}_0.1.0_amd64.deb"

# Vérifier si l'utilisateur est root (nécessaire pour dpkg)
if [ "$EUID" -ne 0 ]; then
  echo "❌ Ce script doit être exécuté en root. Utilise sudo ./install.sh"
  exit 1
fi

# Vérifier si le fichier .deb existe
if [ ! -f "$DEB_FILE" ]; then
  echo "⚠️ Fichier $DEB_FILE introuvable ! Compilation en cours..."
  cargo deb
fi

# Installer le package
echo "📦 Installation de $PACKAGE_NAME..."
dpkg -i "$DEB_FILE"

# Vérifier les dépendances manquantes et les corriger
echo "🔄 Vérification des dépendances..."
apt-get install -f -y

echo "✅ Installation terminée !"
echo "🛠️ Teste la commande avec : $PACKAGE_NAME --help"
