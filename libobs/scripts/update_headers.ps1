param(
    [string]$Branch = "", # Allow specifying branch as a parameter
    [string]$Repository = "obsproject/obs-studio"
)

# Function to get the latest release tag from GitHub
function Get-LatestReleaseTag {
    $releases = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repository/releases/latest"
    return $releases.tag_name
}

# Set up paths
$tempDir = Join-Path -Path $env:TEMP -ChildPath "obs-studio-temp"
$targetHeaderDir = Join-Path -Path $PSScriptRoot -ChildPath "../headers/obs"

# Determine which branch/tag to use
if ([string]::IsNullOrEmpty($Branch)) {
    $Branch = Get-LatestReleaseTag
    Write-Host "No branch specified. Using latest release tag: $Branch"
}
else {
    Write-Host "Using specified branch/tag: $Branch"
}

# Clean up temp directory if it exists
if (Test-Path -Path $tempDir) {
    Write-Host "Cleaning up existing temporary directory..."
    Remove-Item -Path $tempDir -Recurse -Force
}

# Clone the repository with depth 1
Write-Host "Cloning obs-studio repository (branch/tag: $Branch)..."
git clone --depth 1 --branch $Branch https://github.com/$Repository.git $tempDir

if (-not $?) {
    Write-Error "Failed to clone the repository. Make sure git is installed and the branch/tag name is correct."
    exit 1
}

# Ensure the target directory exists
if (-not (Test-Path -Path $targetHeaderDir)) {
    Write-Host "Creating target directory for headers..."
    New-Item -Path $targetHeaderDir -ItemType Directory -Force | Out-Null
}
else {
    # Clear existing header files
    Write-Host "Clearing existing header files from target directory..."
    Remove-Item -Path "$targetHeaderDir\*" -Recurse -Force
}

# Copy header files from libobs to the target directory
$sourceHeaderDir = Join-Path -Path $tempDir -ChildPath "libobs"
Write-Host "Copying header files from $sourceHeaderDir to $targetHeaderDir..."

# Get all header files (*.h, *.hpp, etc.)
$headerFiles = Get-ChildItem -Path $sourceHeaderDir -Include @("*.h", "*.hpp") -Recurse
$headerCount = $headerFiles.Count
Write-Host "Found $headerCount header files."

foreach ($file in $headerFiles) {
    $relativePath = $file.FullName.Substring($sourceHeaderDir.Length + 1)
    $destination = Join-Path -Path $targetHeaderDir -ChildPath $relativePath
    
    # Ensure the destination directory exists
    $destinationDir = Split-Path -Path $destination -Parent
    if (-not (Test-Path -Path $destinationDir)) {
        New-Item -Path $destinationDir -ItemType Directory -Force | Out-Null
    }
    
    Copy-Item -Path $file.FullName -Destination $destination -Force
}

Write-Host "Cleaning up temporary directory..."
Remove-Item -Path $tempDir -Recurse -Force

git -C $PSScriptRoot/../ apply $PSScriptRoot/patches/001_gh_action_fix_compile.patch

Write-Host "Header files updated successfully!"
