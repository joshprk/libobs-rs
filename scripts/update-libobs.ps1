$ErrorActionPreference = 'Stop'

function Get-PackageVersion {
	param($cargoTomlPath)
	$content = Get-Content -Raw -LiteralPath $cargoTomlPath
	$m = [regex]::Match($content, '(?ms)^\[package\].*?^version\s*=\s*"(.*?)"')
	if (-not $m.Success) {
		throw "Could not find package version in $cargoTomlPath"
	}
	return $m.Groups[1].Value
}

function Set-PackageVersion {
	param($cargoTomlPath, $newVersion)
	$content = Get-Content -Raw -LiteralPath $cargoTomlPath
	# replace the version in the [package] section only
	$newContent = [regex]::Replace($content, '(?ms)(^\[package\].*?^version\s*=\s*")(?<ver>[^"]+)(")', "`$1$newVersion`$3")
	if ($content -eq $newContent) {
		throw "Failed to update package version in $cargoTomlPath"
	}
	Set-Content -LiteralPath $cargoTomlPath -Value $newContent -NoNewline
}

function Bump-Patch {
	param($version)
	# Expect semantic version like x.y.z (allow prerelease/build but bump patch part only)
	if ($version -notmatch '^(\d+)\.(\d+)\.(\d+)(.*)$') {
		throw "Version '$version' is not in expected format x.y.z..."
	}
	$major = [int]$matches[1]
	$minor = [int]$matches[2]
	$patch = [int]$matches[3] + 1
	$rest = $matches[4]
	return "$major.$minor.$patch$rest"
}

function Update-WorkspaceVersion {
	param($workspacePath, $crateName, $oldVersion, $newVersion)
	$content = Get-Content -Raw -LiteralPath $workspacePath

	# Look for lines like: crateName = { path = "libobs", version = "0.1.2" }
	$pattern = [regex]::Escape($crateName) + '\s*=\s*\{[^}]*\bversion\s*=\s*"' + [regex]::Escape($oldVersion) + '"'
	if ($content -notmatch $pattern) {
		# If not found exactly that way, try a more lenient replacement of version attribute close to crateName
		$pattern2 = '(' + [regex]::Escape($crateName) + '\s*=\s*\{[^}]*\bversion\s*=\s*")(?<ver>[^"]+)(")'
		if ($content -match $pattern2) {
			$newContent = [regex]::Replace($content, $pattern2, "`$1$newVersion`$3")
			if ($newContent -ne $content) {
				Set-Content -LiteralPath $workspacePath -Value $newContent -NoNewline
				return $true
			}
		}
		# No crate-specific version entry found; attempt a general replace of the exact oldVersion occurrences related to this crate name.
		# This is conservative: only replace when crateName appears on the same line.
		$lines = Get-Content -LiteralPath $workspacePath
		$changed = $false
		for ($i = 0; $i -lt $lines.Length; $i++) {
			if ($lines[$i] -match [regex]::Escape($crateName) -and $lines[$i] -match [regex]::Escape($oldVersion)) {
				$lines[$i] = $lines[$i] -replace [regex]::Escape($oldVersion), $newVersion
				$changed = $true
			}
		}
		if ($changed) {
			Set-Content -LiteralPath $workspacePath -Value ($lines -join "`n") -NoNewline
			return $true
		}
		# Nothing changed
		return $false
	} else {
		# Direct replace
		$pattern2 = '(' + [regex]::Escape($crateName) + '\s*=\s*\{[^}]*\bversion\s*=\s*")(?<ver>[^"]+)(")'
		$newContent = [regex]::Replace($content, $pattern2, "`$1$newVersion`$3")
		Set-Content -LiteralPath $workspacePath -Value $newContent -NoNewline
		return $true
	}
}

# Main
$scriptRoot = $PSScriptRoot
$repoRoot = Resolve-Path -LiteralPath (Join-Path $scriptRoot '..') | Select-Object -ExpandProperty Path
$workspaceCargo = Join-Path $repoRoot 'Cargo.toml'

# Step 1: run update_headers.ps1 in libobs
$updateHeadersPath = Join-Path $repoRoot 'libobs\scripts\update_headers.ps1'
if (-not (Test-Path $updateHeadersPath)) {
	throw "update_headers.ps1 not found at $updateHeadersPath"
}
Write-Host "Running update_headers.ps1..."
Push-Location (Join-Path $repoRoot 'libobs')
& $updateHeadersPath
Pop-Location

# Crate list in desired publish order
$crates = @('libobs','libobs-wrapper','libobs-sources')
$results = @()

foreach ($crate in $crates) {
	$cargoPath = Join-Path $repoRoot ($crate + '\Cargo.toml')
	if (-not (Test-Path $cargoPath)) {
		throw "Cargo.toml for $crate not found at $cargoPath"
	}
	Write-Host "Processing $crate ..."
	$oldVersion = Get-PackageVersion -cargoTomlPath $cargoPath
	$newVersion = Bump-Patch -version $oldVersion
	Set-PackageVersion -cargoTomlPath $cargoPath -newVersion $newVersion

	# update workspace Cargo.toml
	$updated = Update-WorkspaceVersion -workspacePath $workspaceCargo -crateName $crate -oldVersion $oldVersion -newVersion $newVersion
	if (-not $updated) {
		Write-Host "Warning: did not find workspace version entry for $crate in $workspaceCargo; manual check may be required."
	}

	$results += [pscustomobject]@{Crate=$crate; OldVersion=$oldVersion; NewVersion=$newVersion}
}

# Summary
Write-Host "Version bump summary:"
foreach ($r in $results) {
	Write-Host ("{0}: {1} -> {2}" -f $r.Crate, $r.OldVersion, $r.NewVersion)
}

$answer = Read-Host "Publish all packages in order? (libobs, libobs-wrapper, libobs-sources) Enter 'y' to publish"
if ($answer -match '^[Yy]$') {
	foreach ($crate in $crates) {
		$crateDir = Join-Path $repoRoot $crate
		Write-Host "Publishing $crate from $crateDir ..."
		Push-Location $crateDir
		# Run cargo publish and let errors stop the script
		& cargo publish
		Pop-Location
	}
	Write-Host "Publish flow completed."
} else {
	Write-Host "Publish skipped. You can publish manually when ready."
}