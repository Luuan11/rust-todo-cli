param(
    [int]$Runs = 30,
    [int[]]$Sizes = @(0, 1000, 10000)
)

$ErrorActionPreference = "Stop"

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public static class ProcMem {
    [StructLayout(LayoutKind.Sequential)]
    public struct Counters {
        public uint cb;
        public uint PageFaultCount;
        public UIntPtr PeakWorkingSetSize;
        public UIntPtr WorkingSetSize;
        public UIntPtr QuotaPeakPagedPoolUsage;
        public UIntPtr QuotaPagedPoolUsage;
        public UIntPtr QuotaPeakNonPagedPoolUsage;
        public UIntPtr QuotaNonPagedPoolUsage;
        public UIntPtr PagefileUsage;
        public UIntPtr PeakPagefileUsage;
    }

    [DllImport("psapi.dll", SetLastError = true)]
    static extern bool GetProcessMemoryInfo(IntPtr process, out Counters counters, uint size);

    public static ulong PeakWorkingSet(IntPtr handle) {
        Counters c;
        c.cb = (uint)Marshal.SizeOf(typeof(Counters));
        if (!GetProcessMemoryInfo(handle, out c, c.cb)) {
            throw new System.ComponentModel.Win32Exception();
        }
        return c.PeakWorkingSetSize.ToUInt64();
    }
}
"@

$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    cargo build --release --quiet
    if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }
}
finally {
    Pop-Location
}

$exe = Join-Path $root "target\release\todo.exe"
$work = Join-Path ([IO.Path]::GetTempPath()) ("todo-bench-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $work | Out-Null

function New-Fixture([int]$size, [string]$path) {
    $priorities = "low", "medium", "high"
    $sb = [Text.StringBuilder]::new()
    [void]$sb.Append("{`"next_id`":$size,`"tasks`":[")
    for ($i = 1; $i -le $size; $i++) {
        if ($i -gt 1) { [void]$sb.Append(",") }
        $done = if ($i % 4 -eq 0) { "true" } else { "false" }
        [void]$sb.Append("{`"id`":$i,`"text`":`"task number $i`",`"done`":$done,`"priority`":`"$($priorities[$i % 3])`"}")
    }
    [void]$sb.Append("]}")
    [IO.File]::WriteAllText($path, $sb.ToString())
}

function Measure-Run([string[]]$arguments, [string]$file) {
    $psi = [Diagnostics.ProcessStartInfo]::new($exe)
    $psi.Arguments = $arguments -join " "
    $psi.UseShellExecute = $false
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.EnvironmentVariables["TODO_FILE"] = $file
    $psi.EnvironmentVariables["NO_COLOR"] = "1"

    $sw = [Diagnostics.Stopwatch]::StartNew()
    $p = [Diagnostics.Process]::Start($psi)
    $null = $p.StandardOutput.ReadToEnd()
    $null = $p.StandardError.ReadToEnd()
    $p.WaitForExit()
    $sw.Stop()

    if ($p.ExitCode -ne 0) { throw "todo $($psi.Arguments) exited with $($p.ExitCode)" }
    $peak = [ProcMem]::PeakWorkingSet($p.Handle)
    $p.Dispose()
    [pscustomobject]@{ Ms = $sw.Elapsed.TotalMilliseconds; Peak = $peak }
}

function Get-Median([double[]]$values) {
    $sorted = $values | Sort-Object
    $sorted[[int][math]::Floor($sorted.Count / 2)]
}

$results = foreach ($size in $Sizes) {
    $fixture = Join-Path $work "fixture-$size.json"
    $file = Join-Path $work "todo-$size.json"
    New-Fixture $size $fixture

    foreach ($cmd in @(@("list"), @("add", "benchmark", "task"), @("done", "1"))) {
        if ($cmd[0] -eq "done" -and $size -eq 0) { continue }
        $samples = for ($r = 0; $r -lt $Runs; $r++) {
            Copy-Item $fixture $file -Force
            Measure-Run $cmd $file
        }
        $ms = [double[]]($samples | ForEach-Object Ms)
        [pscustomobject]@{
            Tasks      = $size
            Command    = $cmd[0]
            "Median ms" = "{0:N2}" -f (Get-Median $ms)
            "Min ms"    = "{0:N2}" -f ($ms | Measure-Object -Minimum).Minimum
            "Peak RAM"  = "{0:N2} MB" -f ((($samples | ForEach-Object Peak) | Measure-Object -Maximum).Maximum / 1MB)
        }
    }
}

Remove-Item $work -Recurse -Force

$size = (Get-Item $exe).Length
"Binary size: {0:N0} KB ({1})" -f ($size / 1KB), $exe
"Runs per scenario: $Runs"
$results | Format-Table -AutoSize
