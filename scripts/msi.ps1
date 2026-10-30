# Packages a built craftcraft.exe as the Windows installer, dist/craftcraft-windows-<arch>.msi,
# with WiX 3 (installer/craftcraft.wxs). The release signs the exe first and the .msi after.
#
#   pwsh scripts/msi.ps1 -Exe target/release/craftcraft.exe -Arch x86_64
#
# Finds WiX's candle.exe and light.exe through the WIX variable, as GitHub's Windows runners set
# it, or on PATH.

param(
    [Parameter(Mandatory)][string]$Exe,
    [Parameter(Mandatory)][ValidateSet('x86_64', 'aarch64')][string]$Arch,
    [string]$Out = 'dist'
)

$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot)

$bin = if ($env:WIX) { Join-Path $env:WIX 'bin' } else { Split-Path (Get-Command candle.exe).Source }
$exe = (Resolve-Path $Exe).Path
# Windows Installer versions are three numbers: 0.2.0-beta.1 installs as 0.2.0.
$version = (Get-Item $exe).VersionInfo.ProductVersion -replace '[-+].*$', ''
if ($version -notmatch '^\d+\.\d+\.\d+$') { throw "$exe has no usable version ($version)" }
$platform = @{ x86_64 = 'x64'; aarch64 = 'arm64' }[$Arch]

$obj = Join-Path ([IO.Path]::GetTempPath()) "craftcraft-msi-$Arch"
New-Item -ItemType Directory -Force $obj, $Out | Out-Null
$msi = Join-Path $Out "craftcraft-windows-$Arch.msi"

& "$bin\candle.exe" -nologo -arch $platform -ext WixUtilExtension `
    "-dVersion=$version" "-dExe=$exe" "-dIcon=$((Resolve-Path assets/icon.ico).Path)" `
    -out "$obj\" installer/craftcraft.wxs
if ($LASTEXITCODE -ne 0) { throw 'candle failed' }
# ICE91 only warns that per-user files don't follow a user who installs for everyone; this one can't.
& "$bin\light.exe" -nologo -ext WixUtilExtension -sice:ICE91 -spdb -out $msi "$obj\craftcraft.wixobj"
if ($LASTEXITCODE -ne 0) { throw 'light failed' }

Write-Host "Built $msi ($version)"
