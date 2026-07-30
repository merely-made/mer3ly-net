[CmdletBinding()]
param(
    [string]$OutputPath = "content/github-metadata.json"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$mer3lyRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
if ([IO.Path]::IsPathRooted($OutputPath) -or $OutputPath -match "(^|[\\/])\.\.([\\/]|$)") {
    throw "OutputPath must be relative to the Mer3ly repository"
}
$outputFull = [IO.Path]::GetFullPath((Join-Path $mer3lyRoot $OutputPath))
$contentRoot = [IO.Path]::GetFullPath((Join-Path $mer3lyRoot "content"))
$contentPrefix = $contentRoot.TrimEnd(
    [IO.Path]::DirectorySeparatorChar,
    [IO.Path]::AltDirectorySeparatorChar
) + [IO.Path]::DirectorySeparatorChar
if (-not $outputFull.StartsWith($contentPrefix, [StringComparison]::OrdinalIgnoreCase)) {
    throw "OutputPath must remain below content"
}

& gh auth status *> $null
if ($LASTEXITCODE -ne 0) {
    throw "GitHub CLI is not authenticated; existing metadata cache retained"
}

$authorityRaw = & cargo run --quiet --manifest-path (Join-Path $mer3lyRoot "Cargo.toml") `
    --bin authority -- public-repositories 2>$null
if ($LASTEXITCODE -ne 0) {
    throw "authority validation failed; existing metadata cache retained"
}
$authority = $authorityRaw | ConvertFrom-Json

$records = [System.Collections.Generic.List[object]]::new()
foreach ($repository in @($authority.repository | Where-Object { $_.public })) {
    $raw = & gh repo view $repository.github_slug --json `
        "isArchived,isFork,primaryLanguage,pushedAt,repositoryTopics,stargazerCount,updatedAt,visibility" `
        2>$null
    if ($LASTEXITCODE -ne 0) {
        throw "GitHub metadata refresh failed for $($repository.github_slug); existing cache retained"
    }
    $metadata = $raw | ConvertFrom-Json
    if ($metadata.visibility -ne "PUBLIC") {
        throw "$($repository.github_slug) is not public; existing cache retained"
    }

    $topics = @(
        $metadata.repositoryTopics |
            ForEach-Object { $_.name } |
            Sort-Object -Unique
    )
    $records.Add([ordered]@{
        id = $repository.id
        github_slug = $repository.github_slug
        updated_at = $metadata.updatedAt
        pushed_at = $metadata.pushedAt
        primary_language = if ($null -eq $metadata.primaryLanguage) {
            $null
        } else {
            [string]$metadata.primaryLanguage.name
        }
        stargazer_count = [uint64]$metadata.stargazerCount
        archived = [bool]$metadata.isArchived
        fork = [bool]$metadata.isFork
        topics = $topics
    })
}

$cache = [ordered]@{
    schema = "mer3ly.github-metadata/v1"
    generated_at_utc = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
    repository = @($records)
}
$json = $cache | ConvertTo-Json -Depth 8
if ($json -match "(?i)([A-Z]:\\\\|file://|/Users/|\\\\Users\\\\|viewerPermission|sshUrl|token)") {
    throw "public metadata sanitization failed; existing cache retained"
}

$temporaryDirectory = Join-Path $mer3lyRoot ".tmp"
New-Item -ItemType Directory -Force -Path $temporaryDirectory | Out-Null
$temporaryPath = Join-Path $temporaryDirectory "github-metadata.$PID.json"

try {
    Set-Content -LiteralPath $temporaryPath -Value $json -Encoding utf8
    & cargo run --quiet --manifest-path (Join-Path $mer3lyRoot "Cargo.toml") `
        --bin authority -- validate-metadata $mer3lyRoot $temporaryPath *> $null
    if ($LASTEXITCODE -ne 0) {
        throw "Rust validation rejected refreshed metadata"
    }
    Move-Item -LiteralPath $temporaryPath -Destination $outputFull -Force
} catch {
    if (Test-Path -LiteralPath $temporaryPath) {
        [IO.File]::Delete($temporaryPath)
    }
    throw "metadata refresh failed; existing cache retained: $($_.Exception.Message)"
}

Write-Output "wrote $OutputPath"
Write-Output "metadata: $($records.Count) public repositories refreshed atomically"
