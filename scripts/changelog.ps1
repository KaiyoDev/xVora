# Maintain xVora per-version user changelogs (welcome bullets + Whats-new).
# Canonical store: changelogs/{version}.external.json
# Also updates: CURRENT.external.*, root CHANGELOG.md, manifest.json
#
# Usage:
#   .\scripts\changelog.ps1 add -Category features -Description "Short user-facing summary."
#   .\scripts\changelog.ps1 from-git -Since HEAD~10
#   .\scripts\changelog.ps1 sync
#   .\scripts\changelog.ps1 show
#
# Call on EVERY user-facing change before commit. Do not push from this script.

[CmdletBinding()]
param(
  [Parameter(Position = 0)]
  [ValidateSet("add", "sync", "from-git", "show", "bump")]
  [string]$Command = "show",

  [ValidateSet("features", "fixes", "breaking", "performance", "docs", "chore")]
  [string]$Category = "features",

  [string]$Description = "",

  [switch]$Breaking,

  [string]$Since = "",

  [string]$Version = ""
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
if (-not (Test-Path (Join-Path $Root "Cargo.toml"))) {
  $Root = (Get-Location).Path
}
$Changelogs = Join-Path $Root "changelogs"
$VersionToml = Join-Path $Root "crates\codegen\xvora-version\Cargo.toml"

function Get-CurrentVersion {
  if ($Version) { return $Version.Trim() }
  $raw = Get-Content $VersionToml -Raw
  if ($raw -match 'version\s*=\s*"([^"]+)"') { return $Matches[1] }
  throw "Could not read version from $VersionToml"
}

function Get-JsonPath([string]$ver) {
  Join-Path $Changelogs "$ver.external.json"
}
function Get-MdPath([string]$ver) {
  Join-Path $Changelogs "$ver.external.md"
}

function Read-Entries([string]$path) {
  if (-not (Test-Path $path)) { return @() }
  $raw = Get-Content $path -Raw -Encoding UTF8
  if ([string]::IsNullOrWhiteSpace($raw)) { return @() }
  $arr = $raw | ConvertFrom-Json
  if ($null -eq $arr) { return @() }
  return @($arr)
}

function Write-Entries([string]$path, $entries) {
  $dir = Split-Path $path -Parent
  if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir | Out-Null }
  $list = @($entries)
  $json = ConvertTo-Json -InputObject $list -Depth 6
  [System.IO.File]::WriteAllText($path, ($json.TrimEnd() + "`n"), [System.Text.UTF8Encoding]::new($false))
}

function Entries-ToMarkdown([string]$ver, $entries) {
  $lines = New-Object System.Collections.Generic.List[string]
  # H1 = version only so Release Notes modal shows green version header.
  $lines.Add("# $ver")
  $lines.Add("")
  $order = @("breaking", "features", "fixes", "performance", "docs", "chore")
  $titles = @{
    breaking    = "Breaking"
    features    = "Features"
    fixes       = "Bug Fixes"
    performance = "Performance"
    docs        = "Docs"
    chore       = "Chore"
  }
  $buckets = @{}
  foreach ($k in $order) { $buckets[$k] = New-Object System.Collections.Generic.List[object] }
  foreach ($e in $entries) {
    $cat = [string]$e.category
    if ([string]::IsNullOrWhiteSpace($cat)) { $cat = "features" }
    if ($e.breaking_change -eq $true) { $cat = "breaking" }
    if (-not $buckets.ContainsKey($cat)) { $cat = "features" }
    $buckets[$cat].Add($e)
  }
  foreach ($key in $order) {
    $list = $buckets[$key]
    if ($list.Count -eq 0) { continue }
    $lines.Add("## $($titles[$key])")
    $lines.Add("")
    foreach ($e in $list) {
      $lines.Add("- $($e.description)")
    }
    $lines.Add("")
  }
  return (($lines -join "`n").TrimEnd() + "`n")
}

function Write-RootChangelog($manifest) {
  $lines = New-Object System.Collections.Generic.List[string]
  $lines.Add("# Changelog")
  $lines.Add("")
  $lines.Add("All notable user-facing changes to **xVora** are documented here and in")
  $lines.Add("[``changelogs/``](./changelogs/) (JSON + markdown per version).")
  $lines.Add("")
  $lines.Add("**How users see updates**")
  $lines.Add("")
  $lines.Add("1. Welcome screen Changelog bullets")
  $lines.Add("2. Toast **Whats new** when the installed version differs from last launch")
  $lines.Add("3. Slash ``/release-notes`` (full markdown when available)")
  $lines.Add("")
  $lines.Add("**Maintainer**")
  $lines.Add("")
  $lines.Add("``````powershell")
  $lines.Add(".\scripts\changelog.ps1 add -Category features -Description `"...`"")
  $lines.Add(".\scripts\changelog.ps1 sync")
  $lines.Add("``````")
  $lines.Add("")
  $lines.Add("---")
  $lines.Add("")
  foreach ($v in $manifest.versions) {
    $ver = $v.version
    $date = $v.date
    $lines.Add("## [$ver] - $date")
    $lines.Add("")
    $mdPath = Get-MdPath $ver
    if (Test-Path $mdPath) {
      $body = Get-Content $mdPath -Raw -Encoding UTF8
      $bodyLines = $body -split "`r?`n" | Where-Object { $_ -notmatch '^#\s+xVora' }
      $bodyText = ($bodyLines -join "`n").TrimEnd()
      if ($bodyText) {
        $lines.Add($bodyText)
        $lines.Add("")
      }
    }
  }
  $out = Join-Path $Root "CHANGELOG.md"
  $text = ($lines -join "`n").TrimEnd() + "`n"
  [System.IO.File]::WriteAllText($out, $text, [System.Text.UTF8Encoding]::new($false))
}

function Sync-All {
  $ver = Get-CurrentVersion
  if (-not (Test-Path $Changelogs)) {
    New-Item -ItemType Directory -Path $Changelogs | Out-Null
  }
  $jsonPath = Get-JsonPath $ver
  $entries = @(Read-Entries $jsonPath)
  Write-Entries $jsonPath $entries
  $md = Entries-ToMarkdown $ver $entries
  [System.IO.File]::WriteAllText((Get-MdPath $ver), $md, [System.Text.UTF8Encoding]::new($false))

  Copy-Item $jsonPath (Join-Path $Changelogs "CURRENT.external.json") -Force
  Copy-Item (Get-MdPath $ver) (Join-Path $Changelogs "CURRENT.external.md") -Force

  $manifestPath = Join-Path $Changelogs "manifest.json"
  $today = (Get-Date).ToString("yyyy-MM-dd")
  $summary = if ($entries.Count -gt 0) { [string]$entries[0].description } else { "" }
  if ($summary.Length -gt 100) { $summary = $summary.Substring(0, 97) + "..." }

  $versionRows = @()
  if (Test-Path $manifestPath) {
    try {
      $existing = Get-Content $manifestPath -Raw -Encoding UTF8 | ConvertFrom-Json
      if ($existing.versions) {
        foreach ($row in @($existing.versions)) {
          if ($row.version -ne $ver) {
            $versionRows += [pscustomobject]@{
              version = [string]$row.version
              date    = [string]$row.date
              json    = [string]$row.json
              md      = [string]$row.md
              summary = [string]$row.summary
            }
          }
        }
      }
    } catch {}
  }
  $versionRows = @(
    [pscustomobject]@{
      version = $ver
      date    = $today
      json    = "$ver.external.json"
      md      = "$ver.external.md"
      summary = $summary
    }
  ) + $versionRows

  $manifest = [pscustomobject]@{
    product         = "xVora"
    repo            = "https://github.com/KaiyoDev/xVora"
    changelog_base  = "https://raw.githubusercontent.com/KaiyoDev/xVora/main/changelogs"
    current_version = $ver
    versions        = $versionRows
  }
  $manJson = ConvertTo-Json -InputObject $manifest -Depth 8
  [System.IO.File]::WriteAllText($manifestPath, ($manJson.TrimEnd() + "`n"), [System.Text.UTF8Encoding]::new($false))

  Write-RootChangelog $manifest
  Write-Host "synced version=$ver entries=$($entries.Count)"
  Write-Host "  changelogs/$ver.external.json"
  Write-Host "  changelogs/CURRENT.external.json"
  Write-Host "  CHANGELOG.md"
}

function Add-Entry {
  if ([string]::IsNullOrWhiteSpace($Description)) {
    throw "add requires -Description"
  }
  $ver = Get-CurrentVersion
  $jsonPath = Get-JsonPath $ver
  $entries = @(Read-Entries $jsonPath)
  foreach ($e in $entries) {
    if ([string]$e.description -eq $Description.Trim()) {
      Write-Host "already present: $Description"
      Sync-All
      return
    }
  }
  $obj = [pscustomobject]@{
    category        = $Category
    description     = $Description.Trim()
    breaking_change = [bool]$Breaking
  }
  $entries = @($obj) + @($entries)
  Write-Entries $jsonPath $entries
  Write-Host "added [$Category] $Description"
  Sync-All
}

function From-Git {
  $range = if ($Since) { $Since } else { "HEAD~20" }
  $log = @(git -C $Root log --oneline $range 2>$null)
  if ($log.Count -eq 0) {
    Write-Host "no commits in range $range"
    return
  }
  $ver = Get-CurrentVersion
  $jsonPath = Get-JsonPath $ver
  $entries = @(Read-Entries $jsonPath)
  $existing = @{}
  foreach ($e in $entries) { $existing[[string]$e.description] = $true }

  $added = 0
  foreach ($line in $log) {
    if ($line -notmatch '^[a-f0-9]+\s+(.+)$') { continue }
    $msg = $Matches[1].Trim()
    $cat = "features"
    if ($msg -match '^(fix)(\(|:)') { $cat = "fixes" }
    elseif ($msg -match '^(docs)(\(|:)') { $cat = "docs" }
    elseif ($msg -match '^(chore|ci|build|test)(\(|:)') { $cat = "chore" }
    elseif ($msg -match '^(feat)(\(|:)') { $cat = "features" }
    $desc = $msg -replace '^(feat|fix|docs|chore|ci|build|test|refactor|perf)(\([^)]*\))?:\s*', ''
    if ($existing.ContainsKey($desc)) { continue }
    $entries = @(
      [pscustomobject]@{
        category        = $cat
        description     = $desc
        breaking_change = $false
      }
    ) + $entries
    $existing[$desc] = $true
    $added++
  }
  Write-Entries $jsonPath $entries
  Write-Host "from-git: added $added entries from $range"
  Sync-All
}

function Show-Status {
  $ver = Get-CurrentVersion
  $jsonPath = Get-JsonPath $ver
  $entries = @(Read-Entries $jsonPath)
  Write-Host "version: $ver"
  Write-Host "file:    changelogs/$ver.external.json"
  Write-Host "entries: $($entries.Count)"
  foreach ($e in $entries) {
    $b = if ($e.breaking_change) { " [BREAKING]" } else { "" }
    Write-Host ("  - [{0}] {1}{2}" -f $e.category, $e.description, $b)
  }
}

switch ($Command) {
  "add"      { Add-Entry }
  "sync"     { Sync-All }
  "from-git" { From-Git }
  "show"     { Show-Status }
  "bump"     {
    if (-not $Version) { throw "bump requires -Version x.y.z" }
    Write-Host "Remember to set version in crates/codegen/xvora-version/Cargo.toml first."
    Sync-All
  }
  default    { Show-Status }
}
