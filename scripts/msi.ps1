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

& "$bin\candle.exe" -nologo -arch $platform -ext WixUtilExtension -ext WixUIExtension `
    "-dVersion=$version" "-dExe=$exe" "-dIcon=$((Resolve-Path assets/icon.ico).Path)" `
    -out "$obj\" installer/craftcraft.wxs
if ($LASTEXITCODE -ne 0) { throw 'candle failed' }
# The validators take the Start menu for a per-user folder, so they want its component kept under
# HKCU (ICE38, ICE43, ICE57) and warn about the shortcut (ICE91). It's HKMU instead: HKCU just for
# the user, and HKLM beside everyone's Start menu, which they can't tell.
$ice = 'ICE38', 'ICE43', 'ICE57', 'ICE91' | ForEach-Object { "-sice:$_" }
& "$bin\light.exe" -nologo -ext WixUtilExtension -ext WixUIExtension -cultures:en-us @ice -spdb -out $msi "$obj\craftcraft.wixobj"
if ($LASTEXITCODE -ne 0) { throw 'light failed' }

Write-Host "Built $msi ($version)"
