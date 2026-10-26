# Local equivalent of setup-inno-ci.ps1: download the pinned Inno compiler,
# verify its SHA-256, and silent-install to %TEMP% (no system-wide install).
$ErrorActionPreference = "Stop"
$lock = Get-Content -LiteralPath "$PSScriptRoot/../packaging/inno/toolchain.json" -Raw | ConvertFrom-Json
$temp = [IO.Path]::GetTempPath()
$download = Join-Path $temp "innosetup-$($lock.version).exe"
$destination = Join-Path $temp "pecofence-inno-$($lock.version)"
$compiler = Join-Path $destination "ISCC.exe"
if (Test-Path -LiteralPath $compiler -PathType Leaf) {
  $v = & $compiler --version
  if ($v -ceq $lock.version) { Write-Output "Inno $v already at $compiler"; exit 0 }
}
Invoke-WebRequest -Uri $lock.url -OutFile $download
$hash = (Get-FileHash -LiteralPath $download -Algorithm SHA256).Hash
if ($hash -ine $lock.sha256) { throw "Inno Setup SHA-256 mismatch: $hash" }
Write-Output "SHA-256 verified: $hash"
$p = Start-Process -FilePath $download -ArgumentList @('/VERYSILENT', '/SUPPRESSMSGBOXES', '/NORESTART', '/CURRENTUSER', '/NOICONS', "/DIR=`"$destination`"") -WindowStyle Hidden -Wait -PassThru
if ($p.ExitCode -ne 0) { throw "Inno Setup installation failed: $($p.ExitCode)" }
$v = & $compiler --version
if ($LASTEXITCODE -ne 0 -or "$v" -cne $lock.version) { throw "Unexpected Inno compiler: $v" }
Write-Output "ISCC=$compiler"
