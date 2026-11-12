# Rellena los manifiestos de winget/plantilla con una versión ya PUBLICADA:
# baja su instalador de la release, calcula el SHA-256 y deja los manifiestos en
# winget/manifests/a/AtunFrito/TunaFonts/<versión>/, listos para validar y enviar.
#
#   ./winget/preparar.ps1 0.2.0
param([Parameter(Mandatory)][string]$Version)
$ErrorActionPreference = "Stop"
$v = $Version.TrimStart("v")
$url = "https://github.com/atunfrito/tuna-fonts/releases/download/v$v/tuna-fonts_${v}_x64-setup.exe"
$tmp = Join-Path ([IO.Path]::GetTempPath()) "tuna-fonts_${v}_x64-setup.exe"
Invoke-WebRequest $url -OutFile $tmp
$sha = (Get-FileHash $tmp -Algorithm SHA256).Hash.ToUpper()
Remove-Item $tmp
$fecha = ([datetime](Invoke-RestMethod "https://api.github.com/repos/atunfrito/tuna-fonts/releases/tags/v$v").published_at).ToString("yyyy-MM-dd")
$salida = Join-Path $PSScriptRoot "manifests/a/AtunFrito/TunaFonts/$v"
New-Item -ItemType Directory -Force $salida | Out-Null
foreach ($f in Get-ChildItem (Join-Path $PSScriptRoot "plantilla") -Filter *.yaml) {
  $texto = (Get-Content $f.FullName -Raw).Replace("{VERSION}", $v).Replace("{SHA256}", $sha).Replace("{FECHA}", $fecha)
  [IO.File]::WriteAllText((Join-Path $salida $f.Name), $texto)
}
"SHA-256 $sha"
"manifiestos en $salida"
"siguiente: winget validate --manifest `"$salida`""
