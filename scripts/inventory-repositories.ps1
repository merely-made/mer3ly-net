[CmdletBinding()]
param(
    [string]$WorkspaceRoot = (Join-Path $PSScriptRoot "..\..\.."),
    [string]$OutputPath
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Invoke-GhJson {
    param([Parameter(Mandatory)][string[]]$Arguments)

    $raw = & gh @Arguments 2>$null
    if ($LASTEXITCODE -ne 0) {
        throw "GitHub query failed: gh $($Arguments -join ' ')"
    }
    return $raw | ConvertFrom-Json
}

function ConvertTo-GitHubSlug {
    param([string]$Remote)

    if ([string]::IsNullOrWhiteSpace($Remote)) {
        return $null
    }
    $match = [regex]::Match(
        $Remote.Trim(),
        "github\.com(?::|/)(?<owner>[^/]+)/(?<repo>[^/\s]+?)(?:\.git)?$"
    )
    if (-not $match.Success) {
        return $null
    }
    $repository = $match.Groups["repo"].Value -replace "\.git$", ""
    return "$($match.Groups["owner"].Value)/$repository"
}

function Get-LocalRepository {
    param(
        [Parameter(Mandatory)][string]$Workspace,
        [Parameter(Mandatory)][pscustomobject]$Target,
        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [System.Collections.Generic.List[string]]$Drift
    )

    if ([string]::IsNullOrWhiteSpace($Target.local_locator)) {
        return $null
    }

    $workspaceFull = [IO.Path]::GetFullPath($Workspace)
    $localFull = [IO.Path]::GetFullPath(
        (Join-Path $workspaceFull ($Target.local_locator -replace "/", [IO.Path]::DirectorySeparatorChar))
    )
    $workspacePrefix = $workspaceFull.TrimEnd(
        [IO.Path]::DirectorySeparatorChar,
        [IO.Path]::AltDirectorySeparatorChar
    ) + [IO.Path]::DirectorySeparatorChar
    if (-not $localFull.StartsWith($workspacePrefix, [StringComparison]::OrdinalIgnoreCase)) {
        throw "local locator escaped the workspace: $($Target.local_locator)"
    }
    if (-not (Test-Path -LiteralPath (Join-Path $localFull ".git"))) {
        $Drift.Add("$($Target.id): local checkout missing at $($Target.local_locator)")
        return [ordered]@{
            locator = $Target.local_locator
            present = $false
        }
    }

    $origin = (& git -C $localFull remote get-url origin 2>$null | Select-Object -First 1)
    $originSlug = ConvertTo-GitHubSlug $origin
    $acceptedOrigins = @($Target.current_slug) + @($Target.source_aliases)
    $originAccepted = $originSlug -in $acceptedOrigins
    if (-not $originAccepted) {
        $Drift.Add(
            "$($Target.id): local origin $originSlug is outside current slug and declared aliases"
        )
    }

    $branch = (& git -C $localFull branch --show-current 2>$null | Select-Object -First 1)
    $head = (& git -C $localFull rev-parse HEAD 2>$null | Select-Object -First 1)
    $dirtyCount = @(& git -C $localFull status --porcelain 2>$null).Count
    $oldOwnerPaths = @(
        & git -C $localFull grep -l -E "github\.com/mark-ik|mark-ik/" 2>$null
    )
    $oldOwnerManifestPaths = @(
        $oldOwnerPaths | Where-Object { $_ -like "*Cargo.toml" }
    )

    if (
        $null -ne $Target.expected_old_owner_files -and
        $oldOwnerPaths.Count -ne [int]$Target.expected_old_owner_files
    ) {
        $Drift.Add(
            "$($Target.id): old-owner tracked-file count is $($oldOwnerPaths.Count), expected $($Target.expected_old_owner_files)"
        )
    }
    if (
        $null -ne $Target.expected_old_owner_manifests -and
        $oldOwnerManifestPaths.Count -ne [int]$Target.expected_old_owner_manifests
    ) {
        $Drift.Add(
            "$($Target.id): old-owner Cargo manifest count is $($oldOwnerManifestPaths.Count), expected $($Target.expected_old_owner_manifests)"
        )
    }

    return [ordered]@{
        locator = $Target.local_locator
        present = $true
        origin_slug = $originSlug
        origin_matches_authority = $originAccepted
        branch = $branch
        head = $head
        dirty_entries = $dirtyCount
        old_owner_tracked_files = $oldOwnerPaths.Count
        old_owner_cargo_manifests = $oldOwnerManifestPaths.Count
    }
}

function Get-PackageAssociations {
    param([Parameter(Mandatory)][string[]]$Owners)

    $types = @("container", "npm", "maven", "rubygems", "nuget")
    $counts = @{}
    $failures = [System.Collections.Generic.List[string]]::new()
    $publicUi = @{}

    foreach ($owner in $Owners) {
        foreach ($type in $types) {
            $endpoint = if ($owner -eq "merely-made") {
                "orgs/$owner/packages?package_type=$type&per_page=100"
            } else {
                "users/$owner/packages?package_type=$type&per_page=100"
            }
            $raw = & gh api $endpoint 2>$null
            if ($LASTEXITCODE -ne 0) {
                $failures.Add("$owner/$type")
                continue
            }
            foreach ($package in @($raw | ConvertFrom-Json)) {
                $slug = $package.repository.full_name
                if ([string]::IsNullOrWhiteSpace($slug)) {
                    continue
                }
                if (-not $counts.ContainsKey($slug)) {
                    $counts[$slug] = 0
                }
                $counts[$slug] += 1
            }
        }

        $publicUrl = if ($owner -eq "merely-made") {
            "https://github.com/orgs/$owner/packages"
        } else {
            "https://github.com/$owner`?tab=packages"
        }
        try {
            $publicPage = (Invoke-WebRequest -UseBasicParsing -Uri $publicUrl).Content
            $publicUi[$owner] = if ($publicPage -match "Get started with GitHub Packages") {
                "none-publicly-listed"
            } else {
                "packages-listed-or-unknown"
            }
        } catch {
            $publicUi[$owner] = "query-failed"
        }
    }

    return [ordered]@{
        counts = $counts
        query_failures = @($failures)
        public_ui = $publicUi
    }
}

$mer3lyRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$workspaceFull = [IO.Path]::GetFullPath($WorkspaceRoot)

& gh auth status *> $null
if ($LASTEXITCODE -ne 0) {
    throw "GitHub CLI is not authenticated"
}

$basisRaw = & cargo run --quiet --manifest-path (Join-Path $mer3lyRoot "Cargo.toml") `
    --bin authority -- inventory-targets 2>$null
if ($LASTEXITCODE -ne 0) {
    throw "authority validation failed before inventory"
}
$basis = $basisRaw | ConvertFrom-Json

if ([string]::IsNullOrWhiteSpace($OutputPath)) {
    $OutputPath = $basis.inventory_receipt
}
if ([IO.Path]::IsPathRooted($OutputPath) -or $OutputPath -match "(^|[\\/])\.\.([\\/]|$)") {
    throw "OutputPath must be a safe path relative to the Mer3ly repository"
}
$outputFull = [IO.Path]::GetFullPath((Join-Path $mer3lyRoot $OutputPath))
$receiptsRoot = [IO.Path]::GetFullPath((Join-Path $mer3lyRoot "docs\receipts"))
$receiptsPrefix = $receiptsRoot.TrimEnd(
    [IO.Path]::DirectorySeparatorChar,
    [IO.Path]::AltDirectorySeparatorChar
) + [IO.Path]::DirectorySeparatorChar
if (-not $outputFull.StartsWith($receiptsPrefix, [StringComparison]::OrdinalIgnoreCase)) {
    throw "OutputPath must remain below docs/receipts"
}

$packageInventory = Get-PackageAssociations @("mark-ik", "merely-made")
$drift = [System.Collections.Generic.List[string]]::new()
$repositoryReceipts = [System.Collections.Generic.List[object]]::new()

foreach ($target in $basis.targets) {
    $metadata = Invoke-GhJson -Arguments @(
        "repo",
        "view",
        $target.current_slug,
        "--json",
        "nameWithOwner,visibility,isArchived,isFork,defaultBranchRef,licenseInfo,updatedAt,url"
    )
    $canonicalSlug = $metadata.nameWithOwner
    $defaultBranch = $metadata.defaultBranchRef.name
    $head = (& gh api "repos/$canonicalSlug/commits/$defaultBranch" --jq ".sha" 2>$null)
    if ($LASTEXITCODE -ne 0) {
        throw "could not read default-branch head for $canonicalSlug"
    }

    $actions = Invoke-GhJson -Arguments @("api", "repos/$canonicalSlug/actions/workflows")
    $workflowCount = [int]$actions.total_count

    $pagesRaw = & gh api "repos/$canonicalSlug/pages" 2>$null
    if ($LASTEXITCODE -eq 0) {
        $pages = $pagesRaw | ConvertFrom-Json
        $sourcePath = if ($pages.source.path -eq "/") { "root" } else {
            ($pages.source.path.Trim("/") -replace "[^A-Za-z0-9._-]", "-")
        }
        $pagesStatus = "$($pages.build_type)-$($pages.source.branch)-$sourcePath"
        $pagesReceipt = [ordered]@{
            enabled = $true
            build_type = $pages.build_type
            source_branch = $pages.source.branch
            source_path = $pages.source.path
            custom_domain = $pages.cname
            https_enforced = [bool]$pages.https_enforced
        }
    } else {
        $pagesStatus = "none"
        $pagesReceipt = [ordered]@{ enabled = $false }
    }

    $packageCount = if ($packageInventory.counts.ContainsKey($canonicalSlug)) {
        [int]$packageInventory.counts[$canonicalSlug]
    } else {
        0
    }
    $owner = $canonicalSlug.Split("/")[0]
    $ownerPublicPackageStatus = $packageInventory.public_ui[$owner]
    $packageStatus = if ($packageCount -gt 0) {
        "present"
    } elseif (
        $packageInventory.query_failures.Count -gt 0 -and
        $ownerPublicPackageStatus -eq "none-publicly-listed"
    ) {
        "none-publicly-listed-api-incomplete"
    } elseif ($packageInventory.query_failures.Count -gt 0) {
        "query-incomplete"
    } else {
        "none-detected"
    }

    if ($canonicalSlug -ne $target.current_slug) {
        $drift.Add(
            "$($target.id): GitHub canonical slug is $canonicalSlug, authority says $($target.current_slug)"
        )
    }
    if ($defaultBranch -ne $target.expected_default_branch) {
        $drift.Add(
            "$($target.id): default branch is $defaultBranch, expected $($target.expected_default_branch)"
        )
    }
    if ($head -ne $target.expected_head) {
        $drift.Add("$($target.id): default-branch head differs from authority")
    }
    if ($workflowCount -ne [int]$target.expected_actions_workflows) {
        $drift.Add(
            "$($target.id): Actions workflow count is $workflowCount, expected $($target.expected_actions_workflows)"
        )
    }
    if ($pagesStatus -ne $target.expected_pages_status) {
        $drift.Add(
            "$($target.id): Pages status is $pagesStatus, expected $($target.expected_pages_status)"
        )
    }
    if ($packageStatus -ne $target.expected_packages_status) {
        $drift.Add(
            "$($target.id): Packages status is $packageStatus, expected $($target.expected_packages_status)"
        )
    }

    $local = Get-LocalRepository -Workspace $workspaceFull -Target $target -Drift $drift
    $license = if ($null -eq $metadata.licenseInfo) {
        "none-detected"
    } else {
        $metadata.licenseInfo.key
    }

    $repositoryReceipts.Add([ordered]@{
        id = $target.id
        canonical_slug = $canonicalSlug
        classification = $target.classification
        batch = $target.batch
        disposition = $target.disposition
        visibility = $metadata.visibility.ToString().ToLowerInvariant()
        archived = [bool]$metadata.isArchived
        fork = [bool]$metadata.isFork
        default_branch = $defaultBranch
        default_branch_head = $head
        license_detection = $license
        updated_at = $metadata.updatedAt
        actions_workflows = $workflowCount
        pages = $pagesReceipt
        github_packages = [ordered]@{
            status = $packageStatus
            associated_count = $packageCount
        }
        declared_source_aliases = @($target.source_aliases)
        local = $local
    })
}

$knownLocators = @{}
foreach ($target in $basis.targets) {
    if (-not [string]::IsNullOrWhiteSpace($target.local_locator)) {
        $knownLocators[$target.local_locator] = $target.id
    }
}
$extraLocalRepositories = [System.Collections.Generic.List[object]]::new()
foreach ($area in @("repos", "crates")) {
    $areaFull = Join-Path $workspaceFull $area
    if (-not (Test-Path -LiteralPath $areaFull)) {
        continue
    }
    foreach ($directory in Get-ChildItem -LiteralPath $areaFull -Force -Directory) {
        if (-not (Test-Path -LiteralPath (Join-Path $directory.FullName ".git"))) {
            continue
        }
        $locator = "$area/$($directory.Name)"
        if ($knownLocators.ContainsKey($locator)) {
            continue
        }
        $origin = (& git -C $directory.FullName remote get-url origin 2>$null | Select-Object -First 1)
        $originSlug = ConvertTo-GitHubSlug $origin
        $classification = if ($originSlug -like "mark-ik/*") {
            "unclassified-mark-ik"
        } elseif ([string]::IsNullOrWhiteSpace($originSlug)) {
            "local-only"
        } else {
            "external-donor"
        }
        if ($classification -eq "unclassified-mark-ik") {
            $drift.Add("${locator}: mark-ik repository is missing from the migration ledger")
        }
        $extraLocalRepositories.Add([ordered]@{
            locator = $locator
            origin_slug = $originSlug
            classification = $classification
        })
    }
}

$testingNames = @()
$testingRoot = Join-Path $workspaceFull "testing"
if (Test-Path -LiteralPath $testingRoot) {
    $testingNames = @(
        Get-ChildItem -LiteralPath $testingRoot -Force -Directory |
            Select-Object -ExpandProperty Name |
            Sort-Object
    )
}
$generatedAreas = @(
    "readme-clips",
    "readme-proposals",
    "scry-shots"
) | Where-Object { Test-Path -LiteralPath (Join-Path $workspaceFull $_) }

$authorityHashes = [ordered]@{}
foreach ($relative in @(
    "content/repositories.toml",
    "content/relations.toml",
    "ops/org-migration.toml"
)) {
    $authorityHashes[$relative] = (
        Get-FileHash -LiteralPath (Join-Path $mer3lyRoot $relative) -Algorithm SHA256
    ).Hash.ToLowerInvariant()
}

$receipt = [ordered]@{
    schema = "mer3ly.repository-inventory/v1"
    generated_at_utc = [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
    generated_by = "scripts/inventory-repositories.ps1"
    authority_sha256 = $authorityHashes
    github_package_queries = [ordered]@{
        owners = @("mark-ik", "merely-made")
        types = @("container", "npm", "maven", "rubygems", "nuget")
        failures = @($packageInventory.query_failures)
        public_ui = $packageInventory.public_ui
    }
    repositories = @($repositoryReceipts)
    unresolved_products = @($basis.unresolved_products)
    local_classification = [ordered]@{
        extra_git_repositories = @($extraLocalRepositories)
        testing_area = [ordered]@{
            locator = "testing"
            classification = "local-testing"
            children = $testingNames
        }
        generated_or_editorial_areas = @($generatedAreas)
    }
    drift = @($drift)
}

$json = $receipt | ConvertTo-Json -Depth 12
if ($json -match "(?i)([A-Z]:\\\\|file://|/Users/|\\\\Users\\\\)") {
    throw "sanitization failed: inventory receipt contains an absolute machine path"
}

$outputDirectory = Split-Path -Parent $outputFull
New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null
Set-Content -LiteralPath $outputFull -Value $json -Encoding utf8

Write-Output "wrote $OutputPath"
Write-Output (
    "inventory: {0} authority repositories, {1} extra local repositories, {2} drift findings" -f
    $repositoryReceipts.Count,
    $extraLocalRepositories.Count,
    $drift.Count
)

if ($drift.Count -gt 0) {
    throw "inventory drift detected; review the sanitized receipt"
}
