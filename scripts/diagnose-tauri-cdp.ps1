param(
  [int]$DebugPort = 9224,
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
$devServerProcess = $null
if (-not (Test-DevServerReady)) {
  $devServerProcess = Start-Process -FilePath "npm.cmd" `
    -ArgumentList @("run", "dev") `
    -WorkingDirectory $repoRoot `
    -WindowStyle Hidden `
    -RedirectStandardOutput (Join-Path $repoRoot "vite-diag-cdp.out.log") `
    -RedirectStandardError (Join-Path $repoRoot "vite-diag-cdp.err.log") `
    -PassThru

  $deadline = [DateTime]::UtcNow.AddSeconds(20)
  while (-not (Test-DevServerReady)) {
    if ([DateTime]::UtcNow -gt $deadline) {
      throw "Timed out waiting for Vite dev server"
    }
    Start-Sleep -Milliseconds 200
  }
}

Get-Process cute-clipboard-agent -ErrorAction SilentlyContinue | Stop-Process -Force
$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$DebugPort"
$process = Start-Process -FilePath (Resolve-Path $ExePath) -WindowStyle Hidden -PassThru

try {
  Start-Sleep -Seconds 5
  $portConnections = @(Get-NetTCPConnection -LocalPort $DebugPort -ErrorAction SilentlyContinue)
  $targetList = $null
  $cdpError = $null
  try {
    $targetList = (Invoke-WebRequest -Uri "http://127.0.0.1:$DebugPort/json/list" -UseBasicParsing -TimeoutSec 2).Content
  } catch {
    $cdpError = $_.Exception.Message
  }

  [pscustomobject]@{
    appExited = $process.HasExited
    appProcessId = $process.Id
    viteReady = Test-DevServerReady
    cdpPort = $DebugPort
    cdpPortConnectionCount = $portConnections.Count
    cdpError = $cdpError
    cdpTargets = if ($targetList) { $targetList | ConvertFrom-Json } else { @() }
  } | ConvertTo-Json -Depth 8
}
finally {
  if ($process -and -not $process.HasExited) {
    Stop-Process -Id $process.Id -Force
  }
  if ($devServerProcess -and -not $devServerProcess.HasExited) {
    Stop-Process -Id $devServerProcess.Id -Force
  }
}
