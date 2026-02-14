#!/bin/bash
echo "Installing dependencies..."
npm install --legacy-peer-deps

echo "Loading environment variables..."
# Vite automatically loads .env files.
# To manually export:
# export $(grep -v '^#' .env | xargs)

echo "Starting React Application..."
npm run dev
