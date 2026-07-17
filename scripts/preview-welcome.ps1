# Build scripts/preview-welcome.html with embedded logo + changelog.
# Usage:
#   .\scripts\preview-welcome.ps1
#   .\scripts\preview-welcome.ps1 -Open
# No cargo build required.

[CmdletBinding()]
param([switch]$Open)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
if (-not (Test-Path (Join-Path $Root "Cargo.toml"))) { $Root = (Get-Location).Path }

function Read-Text([string]$path) {
  if (-not (Test-Path $path)) { return "" }
  return (Get-Content $path -Raw -Encoding UTF8) -replace "`r`n", "`n" -replace "`r", "`n"
}

function Js-String([string]$s) {
  # Escape for template-literal inside JS
  return ($s -replace '\\', '\\\\' -replace '`', '\`' -replace '\$', '\$')
}

$logoDir = Join-Path $Root "crates\codegen\xvora-pager\assets\logo"
$mdPath = Join-Path $Root "changelogs\CURRENT.external.md"
$manPath = Join-Path $Root "changelogs\manifest.json"

$logos = @{}
foreach ($s in 5, 7, 10, 12) {
  $p = Join-Path $logoDir ("logo{0:D2}.txt" -f $s)
  $t = (Read-Text $p).TrimEnd("`n")
  $logos["$s"] = $t
}
$md = (Read-Text $mdPath).TrimEnd("`n")
$ver = "0.2.0"
if (Test-Path $manPath) {
  try { $ver = (Get-Content $manPath -Raw | ConvertFrom-Json).current_version } catch {}
}

$tplPath = Join-Path $PSScriptRoot "preview-welcome.html"
$html = Read-Text $tplPath

# Replace placeholders inside the EMBED object
$html = $html.Replace('PLACEHOLDER_LOGO05', (Js-String $logos['5']))
$html = $html.Replace('PLACEHOLDER_LOGO07', (Js-String $logos['7']))
$html = $html.Replace('PLACEHOLDER_LOGO10', (Js-String $logos['10']))
$html = $html.Replace('PLACEHOLDER_LOGO12', (Js-String $logos['12']))
$html = $html.Replace('PLACEHOLDER_MD', (Js-String $md))

$outPath = Join-Path $PSScriptRoot "preview-welcome.out.html"
[System.IO.File]::WriteAllText($outPath, $html, [System.Text.UTF8Encoding]::new($false))

Write-Host "Wrote $outPath"
Write-Host "  version=$ver  logo07 lines=$((($logos['7'] -split "`n").Count))"
Write-Host "Open: $outPath"

if ($Open) {
  Start-Process $outPath
}
