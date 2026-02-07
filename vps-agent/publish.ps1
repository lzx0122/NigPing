param (
    [Parameter(Mandatory=$true)]
    [string]$Username
)

$ImageName = "nigping-vps-agent"
$Tag = "latest"
$FullImage = "${Username}/${ImageName}:${Tag}"

$Client = "docker"
if (Get-Command "podman" -ErrorAction SilentlyContinue) {
    $Client = "podman"
    Write-Host "Using Podman..."
} elseif (Get-Command "docker" -ErrorAction SilentlyContinue) {
    Write-Host "Using Docker..."
} else {
    Write-Error "Neither Podman nor Docker found."
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
