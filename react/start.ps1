Write-Host "Loading environment variables..."
# Vite automatically loads .env files, but if you needed to load them manually into the shell session:
# Get-Content .env | ForEach-Object {
#     $name, $value = $_.Split('=', 2)
#     [Environment]::SetEnvironmentVariable($name, $value, "Process")
# }

Write-Host "Starting React Application..."
npm run dev
