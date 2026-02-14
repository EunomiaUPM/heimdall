Write-Host "Installing dependencies..."
npm install --legacy-peer-deps

Write-Host "Building React Application for Production..."
npm run build

if ($LASTEXITCODE -eq 0) {
    Write-Host "Build successful! The 'dist' folder is ready to be served by Rust." -ForegroundColor Green
}
else {
    Write-Host "Build failed." -ForegroundColor Red
}
