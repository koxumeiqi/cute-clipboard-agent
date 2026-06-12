param(
  [string]$ExePath = ".\src-tauri\target\debug\cute-clipboard-agent.exe"
)

$ErrorActionPreference = "Stop"

function Test-DevServerReady {
  try {
    $response = Invoke-WebRequest -Uri "http://127.0.0.1:1420/index.html" -UseBasicParsing -TimeoutSec 1
    return $response.StatusCode -eq 200
  } catch {
    return $false
  }
}

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Get-Process cute-clipboard-agent -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
@(Get-NetTCPConnection -LocalPort 1420 -ErrorAction SilentlyContinue | Select-Object -ExpandProperty OwningProcess -Unique) |
  ForEach-Object { Stop-Process -Id $_ -Force -ErrorAction SilentlyContinue }

$devServerProcess = Start-Process -FilePath "npm.cmd" `
  -ArgumentList @("run", "dev") `
  -WorkingDirectory $repoRoot `
  -WindowStyle Hidden `
  -RedirectStandardOutput (Join-Path $repoRoot "vite-windowdiag.out.log") `
  -RedirectStandardError (Join-Path $repoRoot "vite-windowdiag.err.log") `
  -PassThru

$deadline = [DateTime]::UtcNow.AddSeconds(20)
while (-not (Test-DevServerReady)) {
  if ([DateTime]::UtcNow -gt $deadline) {
    throw "Timed out waiting for Vite dev server"
  }
  Start-Sleep -Milliseconds 200
}

$process = Start-Process -FilePath (Resolve-Path $ExePath) -PassThru

try {
  Start-Sleep -Seconds 8
  $appProcess = Get-Process -Id $process.Id -ErrorAction SilentlyContinue |
    Select-Object Id, ProcessName, MainWindowTitle, MainWindowHandle, Path, StartTime
  $appWindows = powershell -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot "list-app-windows.ps1") -ProcessId $process.Id
  $visibleWindows = powershell -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot "list-visible-windows.ps1")
  [pscustomobject]@{
    appProcess = $appProcess
    appWindows = if ($appWindows) { $appWindows | ConvertFrom-Json } else { @() }
    visibleWindows = if ($visibleWindows) { $visibleWindows | ConvertFrom-Json } else { @() }
    viteReady = Test-DevServerReady
  } | ConvertTo-Json -Depth 8
}
finally {
  if ($process -and -not $process.HasExited) {
    Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
  }
  if ($devServerProcess -and -not $devServerProcess.HasExited) {
    Stop-Process -Id $devServerProcess.Id -Force -ErrorAction SilentlyContinue
  }
}
