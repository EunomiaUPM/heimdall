#!/bin/bash
echo "Installing dependencies..."
npm install --legacy-peer-deps

echo "Building React Application for Production..."
npm run build

if [ $? -eq 0 ]; then
    echo -e "\033[0;32mBuild successful! The 'dist' folder is ready to be served by Rust.\033[0m"
else
    echo -e "\033[0;31mBuild failed.\033[0m"
fi
