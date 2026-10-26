# Builds CraftCraft for Windows and packages it as the release does:
# dist/craftcraft-windows-<arch>.zip, plus dist/SHA256SUMS.
#
#   pwsh scripts/dist.ps1                                  this computer's architecture
#   pwsh scripts/dist.ps1 -Target aarch64-pc-windows-msvc  another one (needs its Rust target)
#
# To try the result the way a user would: $env:CRAFTCRAFT_DIST = "$PWD\dist"; ./install/install.ps1

param([string]$Target)

$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot)

if (-not $Target) { $Target = (rustc -vV | Select-String '^host: (.+)$').Matches[0].Groups[1].Value }
$arch = switch -Wildcard ($Target) {
    'x86_64-pc-windows-*' { 'x86_64' }
    'aarch64-pc-windows-*' { 'aarch64' }
    default { throw "not a Windows target: $Target" }
}

cargo build --release --locked --target $Target
if ($LASTEXITCODE -ne 0) { throw 'cargo build failed' }

New-Item -ItemType Directory -Force dist | Out-Null
$zip = "dist/craftcraft-windows-$arch.zip"
Compress-Archive -Path "target/$Target/release/craftcraft.exe" -DestinationPath $zip -Force

# One "<sha256>  <file>" line per package, as sha256sum writes them.
Get-ChildItem dist -Filter 'craftcraft-*' | Sort-Object Name | ForEach-Object {
    "$((Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLower())  $($_.Name)"
} | Set-Content dist/SHA256SUMS -Encoding ascii

Write-Host "Built $zip"
