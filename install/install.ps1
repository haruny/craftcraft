# Installs or updates CraftCraft for the current user:
#
#   irm https://haruny.github.io/craftcraft/install.ps1 | iex
#
# Downloads the latest release from https://github.com/haruny/craftcraft/releases, checks its
# SHA-256, puts it in %LOCALAPPDATA%\Programs\CraftCraft, adds it to the Start menu and to
# Settings > Apps (to uninstall it), and opens it. Run it again to update.
#
#   CRAFTCRAFT_DIST       install a build from this folder or address instead (for testing)
#   CRAFTCRAFT_NO_LAUNCH  1: don't open CraftCraft afterwards

& {
    $ErrorActionPreference = 'Stop'
    # Windows PowerShell's progress bar slows downloads to a crawl.
    $ProgressPreference = 'SilentlyContinue'
    [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12

    $repo = 'haruny/craftcraft'
    $dir = Join-Path $env:LOCALAPPDATA 'Programs\CraftCraft'
    $exe = Join-Path $dir 'craftcraft.exe'
    $shortcut = Join-Path ([Environment]::GetFolderPath('Programs')) 'CraftCraft.lnk'
    $uninstallKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftCraft'

    if (-not [Environment]::Is64BitOperatingSystem) { throw 'CraftCraft needs 64-bit Windows.' }
    $arch = 'x86_64'
    try {
        if ([Runtime.InteropServices.RuntimeInformation]::OSArchitecture -eq 'Arm64') { $arch = 'aarch64' }
    } catch {
        if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') { $arch = 'aarch64' }
    }
    $package = "craftcraft-windows-$arch.zip"

    $dist = $env:CRAFTCRAFT_DIST
    if (-not $dist) {
        try {
            $release = Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest" -UseBasicParsing
        } catch {
            throw "Couldn't find the latest CraftCraft release at https://github.com/$repo/releases ($($_.Exception.Message))."
        }
        $dist = "https://github.com/$repo/releases/download/$($release.tag_name)"
    }

    $tmp = Join-Path ([IO.Path]::GetTempPath()) ("craftcraft-" + [guid]::NewGuid())
    New-Item -ItemType Directory $tmp | Out-Null
    try {
        foreach ($file in $package, 'SHA256SUMS') {
            if ($dist -match '^https?://') {
                Invoke-WebRequest "$dist/$file" -OutFile (Join-Path $tmp $file) -UseBasicParsing
            } else {
                Copy-Item (Join-Path $dist $file) $tmp
            }
        }

        $line = Get-Content (Join-Path $tmp 'SHA256SUMS') | Where-Object { ($_ -split '\s+\*?')[1] -eq $package } | Select-Object -First 1
        if (-not $line) { throw "SHA256SUMS has no entry for $package." }
        $expected = ($line -split '\s+')[0].ToLower()
        $actual = (Get-FileHash (Join-Path $tmp $package) -Algorithm SHA256).Hash.ToLower()
        if ($actual -ne $expected) { throw "$package is damaged: its SHA-256 is $actual, not $expected. Nothing was installed." }

        Expand-Archive (Join-Path $tmp $package) (Join-Path $tmp 'unpacked')

        # A running copy holds its file open; ask it to close, as clicking X would.
        Get-Process craftcraft -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $exe } | ForEach-Object {
            $_.CloseMainWindow() | Out-Null
            if (-not $_.WaitForExit(5000)) { $_.Kill(); $_.WaitForExit() }
        }
        New-Item -ItemType Directory -Force $dir | Out-Null
        Copy-Item (Join-Path $tmp 'unpacked\craftcraft.exe') $exe -Force
    } finally {
        Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
    }

    # From the exe's version resource: it's a windowed app, so running it with --version can't
    # reliably print to this console.
    $version = (Get-Item $exe).VersionInfo.ProductVersion

    $shell = New-Object -ComObject WScript.Shell
    $link = $shell.CreateShortcut($shortcut)
    $link.TargetPath = $exe
    $link.WorkingDirectory = $dir
    $link.Description = 'Install, update and open software rewritten in Rust'
    $link.Save()

    # Settings > Apps lists CraftCraft and runs this to remove it. Apps it installed and its data
    # folder are kept, so reinstalling picks up where it left off.
    Set-Content (Join-Path $dir 'uninstall.ps1') -Encoding UTF8 -Value @'
$ErrorActionPreference = 'SilentlyContinue'
$exe = Join-Path $PSScriptRoot 'craftcraft.exe'
Get-Process craftcraft | Where-Object { $_.Path -eq $exe } | Stop-Process -Force
Start-Sleep -Milliseconds 500
Remove-Item (Join-Path ([Environment]::GetFolderPath('Programs')) 'CraftCraft.lnk') -Force
Remove-Item 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftCraft' -Recurse -Force
Set-Location $env:TEMP
Remove-Item $PSScriptRoot -Recurse -Force
'@
    New-Item $uninstallKey -Force | Out-Null
    $strings = @{
        DisplayName     = 'CraftCraft'
        DisplayVersion  = $version
        Publisher       = 'CraftCraft'
        DisplayIcon     = $exe
        InstallLocation = $dir
        URLInfoAbout    = "https://github.com/$repo"
        UninstallString = "powershell.exe -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File `"$dir\uninstall.ps1`""
    }
    foreach ($k in $strings.Keys) { New-ItemProperty $uninstallKey -Name $k -Value $strings[$k] -PropertyType String -Force | Out-Null }
    foreach ($k in 'NoModify', 'NoRepair') { New-ItemProperty $uninstallKey -Name $k -Value 1 -PropertyType DWord -Force | Out-Null }

    Write-Host "CraftCraft $version is installed in $dir."
    Write-Host 'Open it from the Start menu. Run the same command again to update it.'
    if ($env:CRAFTCRAFT_NO_LAUNCH -ne '1') { Start-Process $exe }
}
