param(
  [string]$ExePath = ".\src-tauri\target\debug\cute-clipboard-agent.exe",
  [switch]$OpenHistoryOnStart,
  [switch]$MovePetAfterOpenHistory,
  [switch]$CloseHistoryFromRust,
  [switch]$SkipDevServer,
  [int]$WebviewReadyDelayMs = 2500,
  [int]$RemoteDebuggingPort = 9226
)

$ErrorActionPreference = "Stop"

Add-Type @"
using System;
using System.Text;
using System.Runtime.InteropServices;

public static class Win32E2E {
  public delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);

  [DllImport("user32.dll")]
  public static extern bool EnumWindows(EnumWindowsProc lpEnumFunc, IntPtr lParam);

  [DllImport("user32.dll")]
  public static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint processId);

  [DllImport("user32.dll", CharSet = CharSet.Unicode)]
  public static extern int GetWindowText(IntPtr hWnd, StringBuilder text, int count);

  [DllImport("user32.dll")]
  public static extern bool IsWindowVisible(IntPtr hWnd);

  [DllImport("user32.dll")]
  public static extern bool GetWindowRect(IntPtr hWnd, out RECT rect);

  [DllImport("user32.dll")]
  public static extern bool SetForegroundWindow(IntPtr hWnd);

  [DllImport("user32.dll")]
  public static extern bool PostMessage(IntPtr hWnd, uint msg, IntPtr wParam, IntPtr lParam);

  [DllImport("user32.dll")]
  public static extern bool ReleaseCapture();

  [DllImport("user32.dll")]
  public static extern IntPtr SendMessage(IntPtr hWnd, uint msg, IntPtr wParam, IntPtr lParam);

  [DllImport("user32.dll")]
  public static extern void mouse_event(uint dwFlags, uint dx, uint dy, uint dwData, UIntPtr dwExtraInfo);

  [DllImport("user32.dll")]
  public static extern bool SetCursorPos(int x, int y);

  public const uint WM_CLOSE = 0x0010;
  public const uint WM_NCLBUTTONDOWN = 0x00A1;
  public const int HTCAPTION = 0x0002;
  public const uint MOUSEEVENTF_MOVE = 0x0001;
  public const uint MOUSEEVENTF_LEFTDOWN = 0x0002;
  public const uint MOUSEEVENTF_LEFTUP = 0x0004;

  [StructLayout(LayoutKind.Sequential)]
  public struct RECT {
    public int Left;
    public int Top;
    public int Right;
    public int Bottom;
  }
}
"@

function Get-AppWindows([int]$ProcessId) {
  $windows = New-Object System.Collections.Generic.List[object]
  [Win32E2E]::EnumWindows({
    param([IntPtr]$hWnd, [IntPtr]$lParam)
    [uint32]$windowProcessId = 0
    [void][Win32E2E]::GetWindowThreadProcessId($hWnd, [ref]$windowProcessId)
    if ($windowProcessId -eq $ProcessId) {
      $title = New-Object System.Text.StringBuilder 256
      [void][Win32E2E]::GetWindowText($hWnd, $title, $title.Capacity)
      $rect = New-Object Win32E2E+RECT
      [void][Win32E2E]::GetWindowRect($hWnd, [ref]$rect)
      $windows.Add([pscustomobject]@{
        Handle = $hWnd
        Title = $title.ToString()
        Visible = [Win32E2E]::IsWindowVisible($hWnd)
        Left = $rect.Left
        Top = $rect.Top
        Right = $rect.Right
        Bottom = $rect.Bottom
        Width = $rect.Right - $rect.Left
        Height = $rect.Bottom - $rect.Top
      })
    }
    return $true
  }, [IntPtr]::Zero) | Out-Null
  return $windows
}

function Wait-Until([scriptblock]$Condition, [int]$TimeoutMs = 8000, [string]$Message = "condition") {
  $deadline = [DateTime]::UtcNow.AddMilliseconds($TimeoutMs)
  do {
    $value = & $Condition
    if ($value) {
      return $value
    }
    Start-Sleep -Milliseconds 150
  } while ([DateTime]::UtcNow -lt $deadline)
  throw "Timed out waiting for $Message"
}

function Test-DevServerReady {
  try {
    $response = Invoke-WebRequest -Uri "http://127.0.0.1:1420/index.html" -UseBasicParsing -TimeoutSec 1
    return $response.StatusCode -eq 200
  } catch {
    return $false
  }
}

function Click-At([int]$X, [int]$Y, [int]$Count = 1) {
  [void][Win32E2E]::SetCursorPos($X, $Y)
  for ($i = 0; $i -lt $Count; $i++) {
    [Win32E2E]::mouse_event([Win32E2E]::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 60
    [Win32E2E]::mouse_event([Win32E2E]::MOUSEEVENTF_LEFTUP, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 90
  }
}

function Drag-FromTo([int]$StartX, [int]$StartY, [int]$EndX, [int]$EndY) {
  [void][Win32E2E]::SetCursorPos($StartX, $StartY)
  Start-Sleep -Milliseconds 100
  [Win32E2E]::mouse_event([Win32E2E]::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, [UIntPtr]::Zero)
  Start-Sleep -Milliseconds 120
  $steps = 18
  $lastX = $StartX
  $lastY = $StartY
  for ($i = 1; $i -le $steps; $i++) {
    $nextX = [int]($StartX + (($EndX - $StartX) * $i / $steps))
    $nextY = [int]($StartY + (($EndY - $StartY) * $i / $steps))
    $deltaX = $nextX - $lastX
    $deltaY = $nextY - $lastY
    [Win32E2E]::mouse_event([Win32E2E]::MOUSEEVENTF_MOVE, [uint32]$deltaX, [uint32]$deltaY, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 45
    $lastX = $nextX
    $lastY = $nextY
  }
  [Win32E2E]::mouse_event([Win32E2E]::MOUSEEVENTF_LEFTUP, 0, 0, 0, [UIntPtr]::Zero)
}

function Drag-WindowByCaption([IntPtr]$Handle, [int]$StartX, [int]$StartY, [int]$EndX, [int]$EndY) {
  [void][Win32E2E]::SetForegroundWindow($Handle)
  [void][Win32E2E]::SetCursorPos($StartX, $StartY)
  Start-Sleep -Milliseconds 100
  [void][Win32E2E]::ReleaseCapture()
  [void][Win32E2E]::PostMessage($Handle, [Win32E2E]::WM_NCLBUTTONDOWN, [IntPtr][Win32E2E]::HTCAPTION, [IntPtr]::Zero)
  Start-Sleep -Milliseconds 120
  [void][Win32E2E]::SetCursorPos($EndX, $EndY)
  Start-Sleep -Milliseconds 300
  [Win32E2E]::mouse_event([Win32E2E]::MOUSEEVENTF_LEFTUP, 0, 0, 0, [UIntPtr]::Zero)
}

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$devServerProcess = $null
$devServerPidsBefore = @()
if (-not $SkipDevServer -and -not (Test-DevServerReady)) {
  $devServerPidsBefore = @(Get-NetTCPConnection -LocalPort 1420 -ErrorAction SilentlyContinue | Select-Object -ExpandProperty OwningProcess -Unique)
  $devServerProcess = Start-Process -FilePath "npm.cmd" `
    -ArgumentList @("run", "dev") `
    -WorkingDirectory $repoRoot `
    -WindowStyle Hidden `
    -RedirectStandardOutput (Join-Path $repoRoot "vite-e2e.out.log") `
    -RedirectStandardError (Join-Path $repoRoot "vite-e2e.err.log") `
    -PassThru
  $null = Wait-Until { Test-DevServerReady } -TimeoutMs 20000 -Message "Vite dev server"
}

$resolvedExe = Resolve-Path $ExePath
Get-Process cute-clipboard-agent -ErrorAction SilentlyContinue | Stop-Process -Force
$previousWebviewArgs = $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS
$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$RemoteDebuggingPort"
$startArgs = @()
if ($OpenHistoryOnStart) {
  $startArgs += "--e2e-open-history"
}
if ($MovePetAfterOpenHistory) {
  $startArgs += "--e2e-move-pet-after-open-history"
}
if ($CloseHistoryFromRust) {
  $startArgs += "--e2e-close-history-after-open"
}
if ($startArgs.Count -gt 0) {
  $process = Start-Process -FilePath $resolvedExe -ArgumentList $startArgs -PassThru
} else {
  $process = Start-Process -FilePath $resolvedExe -PassThru
}

try {
  try {
    $pet = Wait-Until {
      Get-AppWindows $process.Id | Where-Object { $_.Visible -and $_.Title -eq "Cute Clipboard Agent" } | Select-Object -First 1
    } -Message "pet window"
  } catch {
    $windowsAfterStart = Get-AppWindows $process.Id
    throw "Timed out waiting for pet window. AppExited=$($process.HasExited) DevServerReady=$(Test-DevServerReady) Windows=$($windowsAfterStart | ConvertTo-Json -Compress)"
  }

  Start-Sleep -Milliseconds $WebviewReadyDelayMs

  if (-not $OpenHistoryOnStart) {
    $petCenterX = [int]($pet.Left + 84)
    $petCenterY = [int]($pet.Top + 86)
    [void][Win32E2E]::SetForegroundWindow($pet.Handle)
    Start-Sleep -Milliseconds 300
    Click-At $petCenterX $petCenterY 1
    Start-Sleep -Milliseconds 120
    Click-At $petCenterX $petCenterY 1
  }

  try {
    $history = Wait-Until {
      Get-AppWindows $process.Id | Where-Object { $_.Visible -and $_.Title -eq "Clipboard History" } | Select-Object -First 1
    } -Message "history window"
  } catch {
    $windowsAfterClick = Get-AppWindows $process.Id
    throw "Timed out waiting for history window. Windows after double click: $($windowsAfterClick | ConvertTo-Json -Compress)"
  }

  Start-Sleep -Milliseconds 1800
  if ($MovePetAfterOpenHistory) {
    $beforeDrag = $pet
    $afterDrag = Get-AppWindows $process.Id | Where-Object { $_.Visible -and $_.Title -eq "Cute Clipboard Agent" } | Select-Object -First 1
    $moved = ($afterDrag.Left -ne $beforeDrag.Left) -or ($afterDrag.Top -ne $beforeDrag.Top)
  } else {
    $beforeDrag = Get-AppWindows $process.Id | Where-Object { $_.Visible -and $_.Title -eq "Cute Clipboard Agent" } | Select-Object -First 1
    $dragStartX = [int]($beforeDrag.Left + 84)
    $dragStartY = [int]($beforeDrag.Top + 86)
    [void][Win32E2E]::SetCursorPos($dragStartX, $dragStartY)
    Start-Sleep -Milliseconds 300
    Drag-FromTo $dragStartX $dragStartY ($dragStartX + 60) ($dragStartY + 35)
    Start-Sleep -Milliseconds 1400
    $afterDrag = Get-AppWindows $process.Id | Where-Object { $_.Visible -and $_.Title -eq "Cute Clipboard Agent" } | Select-Object -First 1
    $moved = ($afterDrag.Left -ne $beforeDrag.Left) -or ($afterDrag.Top -ne $beforeDrag.Top)
    if (-not $moved) {
      Drag-WindowByCaption $beforeDrag.Handle $dragStartX $dragStartY ($dragStartX + 60) ($dragStartY + 35)
      Start-Sleep -Milliseconds 700
      $afterDrag = Get-AppWindows $process.Id | Where-Object { $_.Visible -and $_.Title -eq "Cute Clipboard Agent" } | Select-Object -First 1
      $moved = ($afterDrag.Left -ne $beforeDrag.Left) -or ($afterDrag.Top -ne $beforeDrag.Top)
    }
  }
  if (-not $moved) {
    $windowsAfterDrag = Get-AppWindows $process.Id
    throw "Pet window did not move while history was open. Before=($($beforeDrag.Left),$($beforeDrag.Top)) After=($($afterDrag.Left),$($afterDrag.Top)) Windows=$($windowsAfterDrag | ConvertTo-Json -Compress)"
  }

  if (-not $CloseHistoryFromRust) {
    node (Join-Path $PSScriptRoot "cdp-click-history-close.cjs") $RemoteDebuggingPort
  }
  Start-Sleep -Milliseconds 700
  $historyAfterClose = Get-AppWindows $process.Id | Where-Object { $_.Title -eq "Clipboard History" } | Select-Object -First 1
  if ($historyAfterClose -and $historyAfterClose.Visible) {
    throw "History window is still visible after WM_CLOSE"
  }

  [pscustomobject]@{
    Passed = $true
    ProcessId = $process.Id
    PetMovedFrom = "$($beforeDrag.Left),$($beforeDrag.Top)"
    PetMovedTo = "$($afterDrag.Left),$($afterDrag.Top)"
    HistoryOpened = $true
    HistoryClosedOrHidden = $true
  } | ConvertTo-Json -Compress
}
finally {
  if ($process -and -not $process.HasExited) {
    Stop-Process -Id $process.Id -Force
  }
if ($devServerProcess) {
    if (-not $devServerProcess.HasExited) {
      Stop-Process -Id $devServerProcess.Id -Force -ErrorAction SilentlyContinue
    }
  }
  $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = $previousWebviewArgs
}
