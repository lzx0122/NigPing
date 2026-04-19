param (
    [Parameter(Mandatory=$true)]
    [string]$Username
)

$ImageName = "pingpal-vps-agent"
$Tag = "latest"
$FullImage = "${Username}/${ImageName}:${Tag}"

$Client = $null
$PodmanCmd = Get-Command "podman" -ErrorAction SilentlyContinue
$DockerCmd = Get-Command "docker" -ErrorAction SilentlyContinue

if ($PodmanCmd) {
    podman info *> $null
    if ($LASTEXITCODE -eq 0) {
        $Client = "podman"
        Write-Host "Using Podman..."
    } else {
        Write-Warning "Podman detected but unavailable. Falling back to Docker."
    }
}

if (-not $Client -and $DockerCmd) {
    $Client = "docker"
    Write-Host "Using Docker..."
}

if (-not $Client) {
    Write-Error "Neither a working Podman nor Docker was found."
    exit 1
}

Write-Host "Building Image with ${Client}: ${FullImage}..."
& $Client build -t $FullImage .

if ($LASTEXITCODE -eq 0) {
    Write-Host "Pushing to Docker Hub..."
    & $Client push $FullImage
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Done!"
        Write-Host "Now you can check your Docker Hub repository."
        Write-Host "On your VPS, set DOCKER_IMAGE_NAME=$FullImage in your .env file."
    } else {
        Write-Error "Failed to push image. Make sure you are logged in ($Client login)."
    }
} else {
    Write-Error "Failed to build image."
}
