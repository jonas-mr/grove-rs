#!/bin/bash

# Konfiguriere Variablen
LOCAL_DIR="./"
REMOTE_DIR="/home/microcontroller/grove-rs/"
REMOTE_HOST="raspi"
IGNORE_FILE=".rsync-ignore"

# Synchronisiere vom lokalen Rechner zum Remote-Host
rsync -avz --update --exclude-from=$IGNORE_FILE $LOCAL_DIR $REMOTE_HOST:$REMOTE_DIR

# Synchronisiere vom Remote-Host zum lokalen Rechner
# rsync -avz --update --exclude-from=$IGNORE_FILE $REMOTE_HOST:$REMOTE_DIR $LOCAL_DIR

echo "Synchronisierung abgeschlossen."

