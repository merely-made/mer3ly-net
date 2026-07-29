[CmdletBinding()]
param(
    [string]$GitleaksPath = "gitleaks",
    [string]$HistoryReportDirectory,
    [string]$Output = "docs/receipts/org-transfer/2026-07-29_publication_gate.json"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repositoryRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$workspaceRoot = (Resolve-Path -LiteralPath (Join-Path $repositoryRoot "..\..")).Path
$outputFull = [IO.Path]::GetFullPath((Join-Path $repositoryRoot $Output))
$receiptRoot = [IO.Path]::GetFullPath((Join-Path $repositoryRoot "docs\receipts"))
if (-not $outputFull.StartsWith(
        $receiptRoot + [IO.Path]::DirectorySeparatorChar,
        [StringComparison]::OrdinalIgnoreCase
    )) {
    throw "output must remain below docs/receipts"
}

function Invoke-Authority {
    param([Parameter(Mandatory)][string[]]$Arguments)

    $output = & cargo run --quiet --bin authority -- @Arguments 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "authority command failed"
    }
    $output
}

function Invoke-GhJson {
    param([Parameter(Mandatory)][string]$Endpoint)

    $output = & gh api $Endpoint 2>$null
    if ($LASTEXITCODE -ne 0) {
        return $null
    }
    if (-not $output) {
        return $null
    }
    $output | ConvertFrom-Json
}

function Test-GhEndpoint {
    param([Parameter(Mandatory)][string]$Endpoint)

    & gh api $Endpoint --silent 2>$null
    $LASTEXITCODE -eq 0
}

function Get-Count {
    param($Value)

    if ($null -eq $Value) {
        return 0
    }
    @($Value).Count
}

function Get-OptionalProperty {
    param(
        [Parameter(Mandatory)]$Object,
        [Parameter(Mandatory)][string]$Name
    )

    $property = $Object.PSObject.Properties[$Name]
    if ($property) {
        return $property.Value
    }
    $null
}

function Get-GitValue {
    param(
        [Parameter(Mandatory)][string]$Repository,
        [Parameter(Mandatory)][string[]]$Arguments
    )

    $output = & git -C $Repository @Arguments 2>$null
    if ($LASTEXITCODE -ne 0) {
        throw "git command failed for a publication candidate"
    }
    ($output -join "`n").Trim()
}

function Get-OriginSlug {
    param([Parameter(Mandatory)][string]$Repository)

    $remote = Get-GitValue -Repository $Repository -Arguments @(
        "remote", "get-url", "origin"
    )
    $slug = $remote -replace "^https://github\.com/", ""
    $slug = $slug -replace "^git@github\.com:", ""
    $slug = $slug -replace "\.git$", ""
    $slug
}

function Get-ExcludedPathspec {
    param([Parameter(Mandatory)][string]$Id)

    $paths = [System.Collections.Generic.List[string]]::new()
    $paths.Add(".")
    if ($Id -eq "genet") {
        $paths.Add(":(exclude)tests/wpt/**")
        $paths.Add(":(exclude)tests/blink_perf_tests/**")
    }
    if ($Id -eq "wgpu-graft") {
        $paths.Add(":(exclude)patches/glass-gpui/**")
    }
    $paths.ToArray()
}

function Get-TrackedMatchCount {
    param(
        [Parameter(Mandatory)][string]$Repository,
        [Parameter(Mandatory)][string]$Id,
        [Parameter(Mandatory)][string]$Pattern,
        [switch]$Fixed
    )

    $arguments = [System.Collections.Generic.List[string]]::new()
    $arguments.Add("grep")
    $arguments.Add("-I")
    $arguments.Add("-l")
    $arguments.Add($(if ($Fixed) { "-F" } else { "-E" }))
    $arguments.Add($Pattern)
    $arguments.Add("--")
    foreach ($pathspec in Get-ExcludedPathspec -Id $Id) {
        $arguments.Add($pathspec)
    }
    $matches = @(& git -C $Repository @arguments 2>$null)
    if ($LASTEXITCODE -notin 0, 1) {
        throw "git grep failed for a publication candidate"
    }
    $matches.Count
}

function Read-GitleaksReport {
    param([Parameter(Mandatory)][string]$Path)

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        return @()
    }
    $content = Get-Content -Raw -LiteralPath $Path
    if (-not $content -or $content.Trim() -eq "[]") {
        return @()
    }
    @($content | ConvertFrom-Json)
}

function Get-RuleCounts {
    param([array]$Findings = @())

    @(
        $Findings |
            Group-Object RuleID |
            Sort-Object Name |
            ForEach-Object {
                [ordered]@{
                    rule = $_.Name
                    count = $_.Count
                }
            }
    )
}

function Get-HistoryPrefixCounts {
    param([array]$Findings = @())

    @(
        $Findings |
            ForEach-Object {
                $parts = $_.File -split "/"
                $prefix = if ($parts.Count -ge 3) {
                    $parts[0..2] -join "/"
                }
                elseif ($parts.Count -ge 2) {
                    $parts[0..1] -join "/"
                }
                else {
                    $_.File
                }
                $prefix
            } |
            Group-Object |
            Sort-Object Count -Descending |
            Select-Object -First 20 |
            ForEach-Object {
                [ordered]@{
                    prefix = $_.Name
                    count = $_.Count
                }
            }
    )
}

function Invoke-HeadGitleaks {
    param(
        [Parameter(Mandatory)][string]$Repository,
        [Parameter(Mandatory)][string]$ReportPath
    )

    & $GitleaksPath git `
        --no-banner `
        --redact=100 `
        --config (Join-Path $repositoryRoot "scripts\gitleaks-publication-gate.toml") `
        --log-opts="-1" `
        --report-format json `
        --report-path $ReportPath `
        $Repository 2>$null | Out-Null
    if ($LASTEXITCODE -notin 0, 1) {
        throw "Gitleaks HEAD scan failed"
    }
    Read-GitleaksReport -Path $ReportPath
}

if (-not (Get-Command $GitleaksPath -ErrorAction SilentlyContinue) -and
    -not (Test-Path -LiteralPath $GitleaksPath -PathType Leaf)) {
    throw "Gitleaks is required; pass -GitleaksPath"
}

if (-not $HistoryReportDirectory) {
    $HistoryReportDirectory = Join-Path (
        [IO.Path]::GetTempPath()
    ) ("mer3ly-publication-history-" + [guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Force -Path $HistoryReportDirectory | Out-Null
}
$historyReportFull = [IO.Path]::GetFullPath($HistoryReportDirectory)
New-Item -ItemType Directory -Force -Path $historyReportFull | Out-Null

$basis = (Invoke-Authority -Arguments @("inventory-targets", $repositoryRoot) |
    Out-String) | ConvertFrom-Json
$candidates = @($basis.targets | Where-Object disposition -eq "candidate")

$org = Invoke-GhJson -Endpoint "orgs/merely-made"
$membership = Invoke-GhJson -Endpoint "user/memberships/orgs/merely-made"
$orgRulesetsAvailable =
    Test-GhEndpoint -Endpoint "orgs/merely-made/rulesets?per_page=100"
$packagesAvailable =
    Test-GhEndpoint -Endpoint "user/packages?package_type=container&per_page=1"
$orgRepositories = @(
    Invoke-GhJson -Endpoint "orgs/merely-made/repos?per_page=100&type=all"
)
$orgRepositoryDetails = @(
    $orgRepositories | ForEach-Object {
        Invoke-GhJson -Endpoint "repos/$($_.full_name)"
    }
)

$profile = [Environment]::GetFolderPath("UserProfile")
$profileSlash = $profile.Replace("\", "/")
$machineName = [Environment]::MachineName
$configuredEmail = (& git config --global user.email 2>$null | Out-String).Trim()
$privateNetworkPattern = "(10\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}|192\.168\.[0-9]{1,3}\.[0-9]{1,3}|172\.(1[6-9]|2[0-9]|3[01])\.[0-9]{1,3}\.[0-9]{1,3})"

$scanRoot = Join-Path (
    [IO.Path]::GetTempPath()
) ("mer3ly-publication-head-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $scanRoot | Out-Null

$repositories = [System.Collections.Generic.List[object]]::new()
foreach ($target in $candidates) {
    if (-not $target.local_locator) {
        throw "candidate has no local locator"
    }
    $local = [IO.Path]::GetFullPath((Join-Path $workspaceRoot $target.local_locator))
    if (-not $local.StartsWith(
            $workspaceRoot + [IO.Path]::DirectorySeparatorChar,
            [StringComparison]::OrdinalIgnoreCase
        )) {
        throw "candidate locator escaped the workspace"
    }
    if (-not (Test-Path -LiteralPath (Join-Path $local ".git"))) {
        throw "candidate checkout is missing"
    }

    $metadata = Invoke-GhJson -Endpoint "repos/$($target.current_slug)"
    if ($null -eq $metadata) {
        throw "candidate GitHub metadata is unavailable"
    }
    $defaultBranch = $metadata.default_branch
    $branchMetadata = Invoke-GhJson -Endpoint (
        "repos/$($target.current_slug)/branches/$defaultBranch"
    )
    if ($null -eq $branchMetadata) {
        throw "candidate default branch metadata is unavailable"
    }
    $remoteHead = $branchMetadata.commit.sha
    $branch = Get-GitValue -Repository $local -Arguments @(
        "branch", "--show-current"
    )
    $head = Get-GitValue -Repository $local -Arguments @("rev-parse", "HEAD")
    $dirtyEntries = @(& git -C $local status --porcelain=v1).Count
    $origin = Get-OriginSlug -Repository $local

    $headReport = Join-Path $scanRoot "$($target.id).json"
    $headFindings = @(
        Invoke-HeadGitleaks -Repository $local -ReportPath $headReport
    )
    $historyReport = Join-Path $historyReportFull "$($target.id).json"
    if (-not (Test-Path -LiteralPath $historyReport -PathType Leaf)) {
        & $GitleaksPath git `
            --no-banner `
            --redact=100 `
            --report-format json `
            --report-path $historyReport `
            $local 2>$null | Out-Null
        if ($LASTEXITCODE -notin 0, 1) {
            throw "Gitleaks history scan failed"
        }
    }
    $historyFindings = @(Read-GitleaksReport -Path $historyReport)

    $profilePathMatches =
        (Get-TrackedMatchCount -Repository $local -Id $target.id `
            -Pattern $profile -Fixed) +
        (Get-TrackedMatchCount -Repository $local -Id $target.id `
            -Pattern $profileSlash -Fixed)
    $machineNameMatches = Get-TrackedMatchCount `
        -Repository $local -Id $target.id -Pattern $machineName -Fixed
    $emailMatches = if ($configuredEmail) {
        Get-TrackedMatchCount `
            -Repository $local -Id $target.id -Pattern $configuredEmail -Fixed
    }
    else {
        0
    }
    $privateNetworkMatches = Get-TrackedMatchCount `
        -Repository $local -Id $target.id -Pattern $privateNetworkPattern

    $actionsPermissions =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/actions/permissions"
    $secrets =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/actions/secrets?per_page=100"
    $variables =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/actions/variables?per_page=100"
    $environments =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/environments?per_page=100"
    $environmentSecrets = 0
    $environmentVariables = 0
    $environmentProtectionRules = 0
    if ($environments) {
        foreach ($environment in @($environments.environments)) {
            $encodedEnvironment = [Uri]::EscapeDataString($environment.name)
            $environmentSecretResponse = Invoke-GhJson -Endpoint (
                "repos/$($target.current_slug)/environments/" +
                "$encodedEnvironment/secrets?per_page=100"
            )
            $environmentVariableResponse = Invoke-GhJson -Endpoint (
                "repos/$($target.current_slug)/environments/" +
                "$encodedEnvironment/variables?per_page=100"
            )
            if ($environmentSecretResponse) {
                $environmentSecrets += $environmentSecretResponse.total_count
            }
            if ($environmentVariableResponse) {
                $environmentVariables += $environmentVariableResponse.total_count
            }
            $protectionRules = Get-OptionalProperty `
                -Object $environment -Name "protection_rules"
            $environmentProtectionRules += Get-Count -Value $protectionRules
        }
    }
    $keys =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/keys?per_page=100"
    $hooks =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/hooks?per_page=100"
    $collaborators =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/collaborators?affiliation=direct&per_page=100"
    $rulesets =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/rulesets?per_page=100"
    $protection =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/branches/$defaultBranch/protection"
    $pages =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/pages"
    $releases =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/releases?per_page=100"
    $workflows =
        Invoke-GhJson -Endpoint "repos/$($target.current_slug)/actions/workflows?per_page=100"

    $targetName = ($target.target_slug -split "/", 2)[1]
    $targetCollision = $null -ne (
        $orgRepositoryDetails | Where-Object name -eq $targetName
    )
    $metadataSource = Get-OptionalProperty -Object $metadata -Name "source"
    $metadataParent = Get-OptionalProperty -Object $metadata -Name "parent"
    $sourceNetworkId = if ($metadataSource) {
        $metadataSource.id
    }
    else {
        $metadata.id
    }
    $forkNetworkCollision = @(
        $orgRepositoryDetails | Where-Object {
            $candidateSource =
                Get-OptionalProperty -Object $_ -Name "source"
            $candidateNetworkId = if ($candidateSource) {
                $candidateSource.id
            }
            else {
                $_.id
            }
            $candidateNetworkId -eq $sourceNetworkId
        }
    ).Count -gt 0

    $rootTree = @(
        & git -C $local ls-tree --name-only HEAD 2>$null
    )
    $actionMetadata = $rootTree -contains "action.yml" -or
        $rootTree -contains "action.yaml"

    $blockers = [System.Collections.Generic.List[string]]::new()
    if (-not $packagesAvailable) {
        $blockers.Add("authenticated-package-audit-unavailable")
    }
    if (-not $orgRulesetsAvailable) {
        $blockers.Add("target-org-rulesets-api-unavailable")
    }
    if ($targetCollision) {
        $blockers.Add("target-name-collision")
    }
    if ($forkNetworkCollision) {
        $blockers.Add("target-fork-network-collision")
    }
    if ($dirtyEntries -ne 0) {
        $blockers.Add("local-checkout-dirty")
    }
    if ($branch -ne $defaultBranch) {
        $blockers.Add("local-default-branch-mismatch")
    }
    if ($head -ne $remoteHead) {
        $blockers.Add("local-head-not-pushed")
    }
    if ($origin -ne $target.current_slug) {
        $blockers.Add("local-origin-mismatch")
    }
    if ($headFindings.Count -ne 0 -or
        $profilePathMatches -ne 0 -or
        $machineNameMatches -ne 0 -or
        $emailMatches -ne 0) {
        $blockers.Add("current-tree-sensitive-finding")
    }
    if ($pages) {
        $blockers.Add("pages-migration-decision-required")
    }
    if ($actionMetadata) {
        $blockers.Add("marketplace-action-ui-confirmation-required")
    }

    $repositories.Add([ordered]@{
        id = $target.id
        current_slug = $target.current_slug
        target_slug = $target.target_slug
        public_scope = $target.public_scope
        license_status = $target.license_status
        provenance_status = $target.provenance_status
        sensitive_information_status = $target.sensitive_information_status
        history_remediation = $target.history_remediation
        local = [ordered]@{
            present = $true
            origin_matches_authority = $origin -eq $target.current_slug
            branch = $branch
            default_branch = $defaultBranch
            head = $head
            remote_head = $remoteHead
            head_is_pushed = $head -eq $remoteHead
            dirty_entries = $dirtyEntries
        }
        current_tree = [ordered]@{
            gitleaks_findings = $headFindings.Count
            local_profile_path_files = $profilePathMatches
            local_machine_name_files = $machineNameMatches
            configured_contact_address_files = $emailMatches
            private_network_files_after_vendor_exclusions =
                $privateNetworkMatches
        }
        history = [ordered]@{
            gitleaks_findings = $historyFindings.Count
            rule_counts = @(Get-RuleCounts -Findings $historyFindings)
            top_path_prefixes = @(
                Get-HistoryPrefixCounts -Findings $historyFindings
            )
            decision = $target.history_remediation
        }
        github = [ordered]@{
            public = -not $metadata.private
            archived = $metadata.archived
            fork = $metadata.fork
            parent = if ($metadataParent) {
                $metadataParent.full_name
            }
            else {
                $null
            }
            source = if ($metadataSource) {
                $metadataSource.full_name
            }
            else {
                $null
            }
            target_name_collision = $targetCollision
            target_fork_network_collision = $forkNetworkCollision
            workflows = if ($workflows) {
                $workflows.total_count
            }
            else {
                0
            }
            actions = [ordered]@{
                enabled = if ($actionsPermissions) {
                    $actionsPermissions.enabled
                }
                else {
                    $null
                }
                allowed_actions = if ($actionsPermissions) {
                    $actionsPermissions.allowed_actions
                }
                else {
                    $null
                }
                secrets = if ($secrets) { $secrets.total_count } else { 0 }
                variables = if ($variables) { $variables.total_count } else { 0 }
            }
            environments = if ($environments) {
                $environments.total_count
            }
            else {
                0
            }
            environment_secrets = $environmentSecrets
            environment_variables = $environmentVariables
            environment_protection_rules = $environmentProtectionRules
            deploy_keys = Get-Count -Value $keys
            webhooks = Get-Count -Value $hooks
            direct_collaborators = Get-Count -Value $collaborators
            repository_rulesets = Get-Count -Value $rulesets
            default_branch_protected = $null -ne $protection
            releases = Get-Count -Value $releases
            pages = if ($pages) {
                [ordered]@{
                    enabled = $true
                    status = $pages.status
                    build_type = $pages.build_type
                }
            }
            else {
                [ordered]@{ enabled = $false }
            }
            packages = if ($packagesAvailable) {
                "authenticated-query-available"
            }
            else {
                "authenticated-query-unavailable"
            }
            root_action_metadata = $actionMetadata
        }
        blockers = @($blockers)
        gate_status = if ($blockers.Count -eq 0) { "ready" } else { "blocked" }
    })
}

$receipt = [ordered]@{
    schema = "mer3ly.publication-gate/v1"
    generated_at_utc = [DateTime]::UtcNow.ToString("o")
    generated_by = "scripts/audit-publication-gate.ps1"
    authority_sha256 = [ordered]@{
        "content/repositories.toml" = (
            Get-FileHash -Algorithm SHA256 -LiteralPath (
                Join-Path $repositoryRoot "content\repositories.toml"
            )
        ).Hash.ToLowerInvariant()
        "content/relations.toml" = (
            Get-FileHash -Algorithm SHA256 -LiteralPath (
                Join-Path $repositoryRoot "content\relations.toml"
            )
        ).Hash.ToLowerInvariant()
        "ops/org-migration.toml" = (
            Get-FileHash -Algorithm SHA256 -LiteralPath (
                Join-Path $repositoryRoot "ops\org-migration.toml"
            )
        ).Hash.ToLowerInvariant()
    }
    transfer_behavior = [ordered]@{
        sources = @(
            "https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository",
            "https://docs.github.com/en/packages/learn-github-packages/about-permissions-for-github-packages"
        )
        repository_redirects = "git-and-web-redirects-retained"
        pages = "pages-sites-do-not-redirect"
        secrets_webhooks_deploy_keys = "remain-associated"
        packages = "registry-dependent-authenticated-audit-required"
        organization_defaults = "apply-after-transfer"
    }
    target_organization = [ordered]@{
        membership_active = $membership -and $membership.state -eq "active"
        membership_role = if ($membership) { $membership.role } else { $null }
        default_repository_permission = if ($org) {
            $org.default_repository_permission
        }
        else {
            $null
        }
        member_public_repository_creation = if ($org) {
            $org.members_can_create_public_repositories
        }
        else {
            $null
        }
        rulesets_query_available = $orgRulesetsAvailable
        package_query_available = $packagesAvailable
    }
    repositories = @($repositories)
    summary = [ordered]@{
        candidates = $repositories.Count
        ready = @($repositories | Where-Object gate_status -eq "ready").Count
        blocked = @($repositories | Where-Object gate_status -eq "blocked").Count
    }
}

$json = $receipt | ConvertTo-Json -Depth 14
if ($json -match "(?i)([A-Z]:\\\\|file://|/Users/|\\\\Users\\\\)") {
    throw "sanitization failed: receipt contains an absolute machine path"
}
if ($json -match "(?i)(github_pat_|gh[opusr]_[A-Za-z0-9])") {
    throw "sanitization failed: receipt contains a token-shaped value"
}

$outputDirectory = Split-Path -Parent $outputFull
New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null
[IO.File]::WriteAllText(
    $outputFull,
    $json + [Environment]::NewLine,
    [Text.UTF8Encoding]::new($false)
)

"publication gate: {0} candidates, {1} ready, {2} blocked" -f
    $repositories.Count,
    $receipt.summary.ready,
    $receipt.summary.blocked
