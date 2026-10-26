# Local release packaging: put the real toolchain's cargo ahead of the rustup
# shims (their POSIX symlinks don't exec under Windows), then run make-windows.
$ErrorActionPreference = "Stop"
$tc = Get-ChildItem -LiteralPath "$env:USERPROFILE\.rustup\toolchains" -Directory |
    Select-Object -First 1 -ExpandProperty Name
$env:PATH = "$env:USERPROFILE\.rustup\toolchains\$tc\bin;" + $env:PATH
$cargo = (Get-Command cargo).Source
Write-Output "cargo: $cargo"
& cargo --version
& "$PSScriptRoot\make-windows.ps1" -Repository fox10881/PecoFence -Iscc "C:\Users\yahya\AppData\Local\Temp\pecofence-inno-7.1.0\ISCC.exe"
