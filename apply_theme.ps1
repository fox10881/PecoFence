# PecoFence — Invisible Canvas "top shelf" layout
# 1) Applies assets\themes\Top Shelf Desktop Wallpaper Full HD 4k To Use.jpg as wallpaper
# 2) Stops PecoFence so it cannot overwrite config.json on exit
# 3) Writes config.json: themeStyle=invisibleCanvas, three shelf fences, all icons → Left Shelves
# 4) Launches target\debug\pecofence.exe
[CmdletBinding()]
param([switch]$NoLaunch)

$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot

# --- 1. Wallpaper -----------------------------------------------------------
Add-Type -TypeDefinition @'
using System.Runtime.InteropServices;
public static class PecofenceWallpaper {
    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);
}
'@
$wall = Join-Path $root 'assets\themes\Top Shelf Desktop Wallpaper Full HD 4k To Use.jpg'
if (-not (Test-Path $wall)) { throw "Wallpaper not found: $wall" }
# SPI_SETDESKWALLPAPER = 0x0014; SPIF_UPDATEINIFILE | SPIF_SENDCHANGE = 0x03
[PecofenceWallpaper]::SystemParametersInfo(0x0014, 0, $wall, 0x03) | Out-Null
Write-Host "Wallpaper applied: $wall"

# --- 2. Stop the running app -------------------------------------------------
Get-Process pecofence -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 400

# --- 3. Rewrite config.json --------------------------------------------------
$cfgPath = Join-Path $env:APPDATA 'PecoFence\config.json'
if (-not (Test-Path $cfgPath)) { throw "Config not found: $cfgPath (run PecoFence once first)" }
Copy-Item $cfgPath "$cfgPath.bak-invisible" -Force

$cfg = Get-Content $cfgPath -Raw | ConvertFrom-Json
$cfg.settings.themeStyle = 'invisibleCanvas'
$cfg.settings.hideRealIcons = $true

# Percentages of the monitor work area, matching the wallpaper's shelf positions.
$specs = @(
    @{ title = 'Left Shelves';  xp = 0.05; yp = 0.15; wp = 0.25; hp = 0.60 }
    @{ title = 'Right Shelves'; xp = 0.70; yp = 0.15; wp = 0.25; hp = 0.60 }
    @{ title = 'Bottom Shelf';  xp = 0.30; yp = 0.80; wp = 0.40; hp = 0.15 }
)

$defaultLeftId = $null
foreach ($layout in $cfg.layouts) {
    $mon = @($layout.fingerprint)[0]
    if (-not $mon) { continue }
    $workW = [double]$mon.workDip[0]
    $workH = [double]$mon.workDip[1]

    $left = $null
    foreach ($s in $specs) {
        $x = [math]::Round($workW * $s.xp, 2)
        $y = [math]::Round($workH * $s.yp, 2)
        $w = [math]::Round($workW * $s.wp, 2)
        $h = [math]::Round($workH * $s.hp, 2)

        $fence = @($layout.fences) | Where-Object { $_.title -eq $s.title } | Select-Object -First 1
        if (-not $fence) {
            $fence = [pscustomobject][ordered]@{
                id            = [guid]::NewGuid().ToString()
                title         = $s.title
                kind          = 'virtual'
                source        = [pscustomobject]@{ kind = 'desktop' }
                geometry      = [pscustomobject][ordered]@{
                    monitor = $mon.devicePath
                    x = $x; y = $y; w = $w; h = $h
                    workW = $workW; workH = $workH
                    anchor  = 'leftTop'
                }
                rolledUp      = $false
                expandedH     = $h
                view          = [pscustomobject][ordered]@{
                    iconSize = 96; sort = 'manual'; labelLines = 2; autoHeight = $false
                    reverse = $false; layout = 'icons'; spacing = 'normal'
                    columnWidths = $null; columnsVisible = $null; groupByDate = $false
                    showLabels = $false
                }
                appearance    = [pscustomobject]@{ titleOnHover = $true }
                excludeFromQuickHide = $false
                locked        = $false
                tabHost       = $null
                activeTab     = $null
                tabOrder      = @()
                portalNavigate = $true
                hideTitleIcon = $false
                hideWhenEmpty = $false
                items         = @()
            }
            $layout.fences = @($layout.fences) + $fence
        }
        else {
            $fence.geometry = [pscustomobject][ordered]@{
                monitor = $mon.devicePath
                x = $x; y = $y; w = $w; h = $h
                workW = $workW; workH = $workH
                anchor  = 'leftTop'
            }
            $fence.expandedH = $h
            $fence.rolledUp = $false
            if (-not ($fence.view.PSObject.Properties.Name -contains 'showLabels')) {
                $fence.view | Add-Member -NotePropertyName showLabels -NotePropertyValue $false
            } else { $fence.view.showLabels = $false }
            if (-not $fence.appearance) {
                $fence.appearance = [pscustomobject]@{ titleOnHover = $true }
            } elseif (-not ($fence.appearance.PSObject.Properties.Name -contains 'titleOnHover')) {
                $fence.appearance | Add-Member -NotePropertyName titleOnHover -NotePropertyValue $true
            } else { $fence.appearance.titleOnHover = $true }
        }
        if ($s.title -eq 'Left Shelves') { $left = $fence }
    }

    # 4) Route every icon already fenced into Left Shelves.
    foreach ($f in @($layout.fences)) {
        if ($f -eq $left) { continue }
        $items = @($f.items)
        if ($items.Count) {
            $left.items = @($left.items) + $items
            $f.items = @()
        }
    }
    if (-not $defaultLeftId -and $left) { $defaultLeftId = $left.id }
}

# New desktop icons land on the left shelf too.
if ($cfg.rules -and $defaultLeftId) {
    $cfg.rules.defaultTarget = [pscustomobject]@{ fence = $defaultLeftId }
}

$json = $cfg | ConvertTo-Json -Depth 30
[System.IO.File]::WriteAllText($cfgPath, $json, [System.Text.UTF8Encoding]::new($false))
Write-Host "config.json updated (backup: $cfgPath.bak-invisible)"

# --- 5. Launch ---------------------------------------------------------------
$exe = Join-Path $root 'target\debug\pecofence.exe'
if ($NoLaunch) { Write-Host "Done (-NoLaunch)."; return }
if (-not (Test-Path $exe)) { throw "Debug build not found: $exe (run: cargo build -p pecofence)" }
Start-Process $exe
Write-Host "Launched $exe"
