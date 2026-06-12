param(
  [int]$ProcessId
)

$ErrorActionPreference = "Stop"

Add-Type @"
using System;
using System.Text;
using System.Runtime.InteropServices;

public static class WindowListWin32 {
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
[WindowListWin32]::EnumWindows({
  param([IntPtr]$hWnd, [IntPtr]$lParam)
  [uint32]$windowProcessId = 0
  [void][WindowListWin32]::GetWindowThreadProcessId($hWnd, [ref]$windowProcessId)
  if ($windowProcessId -eq $ProcessId) {
    $title = New-Object System.Text.StringBuilder 256
    [void][WindowListWin32]::GetWindowText($hWnd, $title, 256)
    $rect = New-Object WindowListWin32+RECT
    [void][WindowListWin32]::GetWindowRect($hWnd, [ref]$rect)
    $rows.Add([pscustomobject]@{
      Handle = $hWnd.ToString()
      Title = $title.ToString()
      Visible = [WindowListWin32]::IsWindowVisible($hWnd)
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

$rows | ConvertTo-Json -Depth 3
