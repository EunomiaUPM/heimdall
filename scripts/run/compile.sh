#!/bin/bash
# Builds the React console into react/dist, which heimdall serves at /admin.
set -e

cd "$(dirname "${BASH_SOURCE[0]}")/../../react"

echo "Installing dependencies..."
npm install --legacy-peer-deps

echo "Building React Application for Production..."
if npm run build; then
    echo -e "\033[0;32mBuild successful! The 'dist' folder is ready to be served by Rust.\033[0m"
else
    echo -e "\033[0;31mBuild failed.\033[0m"
    exit 1
fi
