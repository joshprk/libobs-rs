param(
    [switch]$SkipHeaders = $false
)

# Strict error handling
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest


function Get-PackageVersion {
    param (
        [string]$CargoTomlPath
    )
    
    $content = Get-Content -Path $CargoTomlPath -Raw
    
    # Look for version in [package] section
    if ($content -match '(?m)^\[package\][\r\n]+((?:.*[\r\n]+)*?)^version\s*=\s*"([^"]+)"') {
        return $Matches[2]
    }
    
    throw "Could not find version in $CargoTomlPath"
}

function Set-PackageVersion {
    param (
        [string]$CargoTomlPath,
        [string]$NewVersion
    )
    
    $content = Get-Content -Path $CargoTomlPath -Raw
    
    # Update version in [package] section only
    $updatedContent = $content -replace '(?m)(^\[package\][\r\n]+((?:.*[\r\n]+)*?)^version\s*=\s*")[^"]+(")', "`${1}$NewVersion`$3"
    
    if ($updatedContent -eq $content) {
        throw "Failed to update version in $CargoTomlPath"
    }
    
    Set-Content -Path $CargoTomlPath -Value $updatedContent -NoNewline
    Write-Host "Updated $CargoTomlPath to version $NewVersion"
}

function Bump-Patch {
    param (
        [string]$Version
    )
    
    if ($Version -match '^(\d+)\.(\d+)\.(\d+)$') {
        $major = $Matches[1]
        $minor = $Matches[2]
        $patch = [int]$Matches[3] + 1
        return "$major.$minor.$patch"
    }
    
    throw "Invalid version format: $Version"
}

function Update-CrateVersion {
    param (
        [string]$CratePath,
        [string]$CrateName
    )
    
    $cargoToml = Join-Path $CratePath "Cargo.toml"
    
    Write-Host "`nUpdating $CrateName..."
    $currentVersion = Get-PackageVersion -CargoTomlPath $cargoToml
    $newVersion = Bump-Patch -Version $currentVersion
    
    Write-Host "  $currentVersion -> $newVersion"
    Set-PackageVersion -CargoTomlPath $cargoToml -NewVersion $newVersion
    
    return $newVersion
}

# Main script
try {
    $rootDir = Split-Path -Parent $PSScriptRoot
    
    # Step 1: Update headers
    if (-not $SkipHeaders) {
        Write-Host "Step 1: Updating headers..."
        $updateHeadersScript = Join-Path $rootDir "libobs\scripts\update_headers.ps1"
        & $updateHeadersScript
        if ($LASTEXITCODE -ne 0) {
            throw "update_headers.ps1 failed"
        }
    } else {
        Write-Host "Step 1: Skipping header updates (SkipHeaders flag set)"
    }
    
    # Step 2-4: Update versions for each crate
    Write-Host "`nStep 2-4: Updating crate versions..."
    
    $libobsVersion = Update-CrateVersion -CratePath (Join-Path $rootDir "libobs") -CrateName "libobs"
    $libobsWrapperVersion = Update-CrateVersion -CratePath (Join-Path $rootDir "libobs-wrapper") -CrateName "libobs-wrapper"
    $libobsSourcesVersion = Update-CrateVersion -CratePath (Join-Path $rootDir "libobs-sources") -CrateName "libobs-sources"
    
    # Update workspace Cargo.toml
    Write-Host "`nUpdating workspace Cargo.toml..."
    $workspaceCargoToml = Join-Path $rootDir "Cargo.toml"
    $workspaceContent = Get-Content -Path $workspaceCargoToml -Raw
    
    # Update each dependency version in workspace (matches format: version = "x.y.z")
    $workspaceContent = $workspaceContent -replace '(?m)(^libobs\s*=\s*\{[^\}]*version\s*=\s*")[^"]+(")', "`${1}$libobsVersion`$2"
    $workspaceContent = $workspaceContent -replace '(?m)(^libobs-wrapper\s*=\s*\{[^\}]*version\s*=\s*")[^"]+(")', "`${1}$libobsWrapperVersion`$2"
    $workspaceContent = $workspaceContent -replace '(?m)(^libobs-sources\s*=\s*\{[^\}]*version\s*=\s*")[^"]+(")', "`${1}$libobsSourcesVersion`$2"
    
    Set-Content -Path $workspaceCargoToml -Value $workspaceContent -NoNewline
    Write-Host "Updated workspace Cargo.toml"
    
    # Show summary
    Write-Host "`n========================================="
    Write-Host "Version Update Summary:"
    Write-Host "  libobs:         $libobsVersion"
    Write-Host "  libobs-wrapper: $libobsWrapperVersion"
    Write-Host "  libobs-sources: $libobsSourcesVersion"
    Write-Host "========================================="
    
    # Step 5: Ask user for confirmation
    Write-Host "`nDo you want to publish all packages? (y/n): " -NoNewline
    $response = Read-Host
    
    if ($response -ne 'y' -and $response -ne 'Y') {
        Write-Host "Publishing cancelled."
        exit 0
    }
    
    # Publish packages in order
    Write-Host "`nPublishing packages..."
    
    Write-Host "`nPublishing libobs..."
    Set-Location (Join-Path $rootDir "libobs")
    cargo publish
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to publish libobs"
    }
    
    Write-Host "`nWaiting for libobs to be available on crates.io..."
    Start-Sleep -Seconds 30
    
    Write-Host "`nPublishing libobs-wrapper..."
    Set-Location (Join-Path $rootDir "libobs-wrapper")
    cargo publish
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to publish libobs-wrapper"
    }
    
    Write-Host "`nWaiting for libobs-wrapper to be available on crates.io..."
    Start-Sleep -Seconds 30
    
    Write-Host "`nPublishing libobs-sources..."
    Set-Location (Join-Path $rootDir "libobs-sources")
    cargo publish
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to publish libobs-sources"
    }
    
    Write-Host "`n========================================="
    Write-Host "All packages published successfully!"
    Write-Host "========================================="
    
} catch {
    Write-Error "Error: $_"
    exit 1
}
