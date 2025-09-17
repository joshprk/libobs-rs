param (
    [Parameter(Mandatory = $true)]
    [string]$binary,

    [Parameter(Mandatory = $false)]
    [string]$argumentHex = "",

    [Parameter(Mandatory = $true)]
    [int]$processPid,

    [Parameter(Mandatory = $false)]
    [switch]$restart = $false
)

Write-Host "Waiting for process with PID $processPid to exit..."
try {
    $process = Get-Process -Id $processPid -ErrorAction Stop
    $process.WaitForExit()
    Write-Host "Process with PID $processPid has exited."
}
catch {
    Write-Host "Process with PID $processPid is not running or already exited."
}

$binaryDirectory = [System.IO.Path]::GetDirectoryName($binary)
$obsNewDir = Join-Path -Path $binaryDirectory -ChildPath "obs_new"

if (Test-Path $obsNewDir -PathType Container) {
    Write-Host "Found obs_new directory, copying all contents to binary directory"

    try {
        $files = Get-ChildItem -Path $obsNewDir -Recurse
        foreach ($file in $files) {
            $relativePath = $file.FullName.Substring($obsNewDir.Length + 1)
            $destination = Join-Path -Path $binaryDirectory -ChildPath $relativePath

            # Create directory structure if needed
            if ($file.PSIsContainer) {
                if (-Not (Test-Path $destination -PathType Container)) {
                    New-Item -ItemType Directory -Path $destination -Force | Out-Null
                }
                continue
            }

            # Remove target file if it exists
            if (Test-Path $destination) {
                try {
                    Remove-Item -Path $destination -Force
                }
                catch {
                    Write-Host "Failed to remove existing file ${destination}: $_"
                    exit 1
                }
            }

            # Copy the file
            try {
                Copy-Item -Path $file.FullName -Destination $destination -Force
            }
            catch {
                Write-Host "Failed to copy $($file.FullName) to ${destination}: $_"
                exit 1
            }
        }
        Write-Host "Successfully copied all contents from obs_new to binary directory"

        # Optionally remove the obs_new directory after successful copy
        try {
            Remove-Item -Path $obsNewDir -Recurse -Force
            Write-Host "Removed obs_new directory after copying contents"
        }
        catch {
            Write-Host "Warning: Could not remove obs_new directory: $_"
        }
    }
    catch {
        Write-Host "Error copying files from obs_new directory: $_"
        exit 1
    }
}
else {
    Write-Host "Warning: obs_new directory not found in $binaryDirectory"
}

if (-not $restart) {
    Write-Host "No binary specified, exiting."
    exit 0
}

# Decode argumentHex to argument array
$arguments = @()
if ($argumentHex -ne "") {
    try {
        # Split the hex string into argument hex segments by '00' (null separator)
        $hexArgs = $argumentHex -split '00'
        $arguments = @()
        foreach ($hexArg in $hexArgs) {
            if ($hexArg -ne "") {
            $bytes = @()
            for ($i = 0; $i -lt $hexArg.Length; $i += 2) {
                $bytes += [Convert]::ToByte($hexArg.Substring($i, 2), 16)
            }
            $decoded = [System.Text.Encoding]::UTF8.GetString($bytes)
            if ($decoded -ne "") {
                $arguments += $decoded
            }
            }
        }
    }
    catch {
        Write-Host "Failed to decode argumentHex: $_. Starting without arguments."
    }
}

Write-Host "Restarting $binary with arguments: $($arguments -join ' ')"
try {
    if ($arguments.Count -eq 0) {
        Start-Process -FilePath $binary
    }
    else {
        Start-Process -FilePath $binary -ArgumentList $arguments
    }
    Write-Host "Successfully restarted $binary"
}
catch {
    Write-Host "Failed to restart ${binary}: $_"
    exit 1
}