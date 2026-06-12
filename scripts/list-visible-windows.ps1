$ErrorActionPreference = "Stop"

Add-Type @"
using System;
using System.Text;
using System.Runtime.InteropServices;

public static class VisibleWindowListWin32 {
  public delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);
  [DllImport("user32.dll")]
  public static extern bool EnumWindows(EnumWindowsProc cb, IntPtr lp);
  [DllImport("user32.dll")]
  public static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint pid);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)]
  public static extern int GetWindowText(IntPtr hWnd, StringBuilder text, int count);
  [DllImport("user32.dll")]
  public static extern bool IsWindowVisible(IntPtr hWnd);
  [DllImport("user32.dll")]
  public static extern bool GetWindowRect(IntPtr hWnd, out RECT r);
  [StructLayout(LayoutKind.Sequential)]
  public struct RECT {
    public int Left;
    public int Top;
    public int Right;
    public int Bottom;
  }
}
"@

$rows = New-Object System.Collections.Generic.List[object]
[VisibleWindowListWin32]::EnumWindows({
  param([IntPtr]$hWnd, [IntPtr]$lParam)
  if (-not [VisibleWindowListWin32]::IsWindowVisible($hWnd)) {
    return $true
  }
  [uint32]$windowProcessId = 0
  [void][VisibleWindowListWin32]::GetWindowThreadProcessId($hWnd, [ref]$windowProcessId)
  $title = New-Object System.Text.StringBuilder 256
  [void][VisibleWindowListWin32]::GetWindowText($hWnd, $title, 256)
  $rect = New-Object VisibleWindowListWin32+RECT
  [void][VisibleWindowListWin32]::GetWindowRect($hWnd, [ref]$rect)
  $processName = ""
  try {
    $processName = (Get-Process -Id $windowProcessId -ErrorAction Stop).ProcessName
  } catch {
    $processName = ""
  }
  $rows.Add([pscustomobject]@{
    ProcessId = $windowProcessId
    ProcessName = $processName
    Handle = $hWnd.ToString()
    Title = $title.ToString()
    Left = $rect.Left
    Top = $rect.Top
    Right = $rect.Right
    Bottom = $rect.Bottom
    Width = $rect.Right - $rect.Left
    Height = $rect.Bottom - $rect.Top
  })
  return $true
}, [IntPtr]::Zero) | Out-Null

$rows |
  Where-Object {
    $_.Title -like "*Cute*" -or
    $_.Title -like "*Clipboard*" -or
    $_.Title -like "*cute-clipboard-agent*" -or
    $_.ProcessName -like "*cute*" -or
    $_.ProcessName -like "*msedgewebview2*"
  } |
  ConvertTo-Json -Depth 3
