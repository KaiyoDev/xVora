# Interactive terminal mock of xVora welcome (no cargo build).
# Usage:  .\scripts\preview-tui.ps1
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
if (-not (Test-Path (Join-Path $Root "Cargo.toml"))) { $Root = (Get-Location).Path }
Set-Location $Root
node "$Root\scripts\preview-tui.mjs"
