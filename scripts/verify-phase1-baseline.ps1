[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$manifestPath = Join-Path $repoRoot 'baseline\phase1\manifest.json'
$manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json

function Assert-Equal {
    param(
        [Parameter(Mandatory)]
        [string]$Label,
        [Parameter(Mandatory)]
        [AllowEmptyString()]
        [string]$Expected,
        [Parameter(Mandatory)]
        [AllowEmptyString()]
        [string]$Actual
    )

    if ($Expected -ne $Actual) {
        throw "$Label mismatch.`nExpected: $Expected`nActual:   $Actual"
    }
}

function Invoke-Checked {
    param(
        [Parameter(Mandatory)]
        [string]$FilePath,
        [Parameter(Mandatory)]
        [string[]]$ArgumentList
    )

    Write-Host "`n> $FilePath $($ArgumentList -join ' ')" -ForegroundColor Cyan
    & $FilePath @ArgumentList
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed with exit code ${LASTEXITCODE}: $FilePath $($ArgumentList -join ' ')"
    }
}

function Get-Sha256 {
    param([Parameter(Mandatory)][string]$Path)

    (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Replace-RequiredPdfField {
    param(
        [Parameter(Mandatory)]
        [string]$Text,
        [Parameter(Mandatory)]
        [string]$Pattern,
        [Parameter(Mandatory)]
        [string]$Replacement,
        [Parameter(Mandatory)]
        [string]$Label
    )

    $matches = [regex]::Matches($Text, $Pattern)
    if ($matches.Count -ne 1) {
        throw "Expected exactly one $Label field, found $($matches.Count)."
    }

    [regex]::Replace($Text, $Pattern, $Replacement)
}

function Get-NormalizedPdfSha256 {
    param([Parameter(Mandatory)][string]$Path)

    $latin1 = [Text.Encoding]::GetEncoding(28591)
    $text = $latin1.GetString([IO.File]::ReadAllBytes($Path))
    $text = Replace-RequiredPdfField $text "/ModDate \(D:\d{14}[+-]\d{2}'\d{2}\)" "/ModDate (D:00000000000000+00'00)" 'Info.ModDate'
    $text = Replace-RequiredPdfField $text "/CreationDate \(D:\d{14}[+-]\d{2}'\d{2}\)" "/CreationDate (D:00000000000000+00'00)" 'Info.CreationDate'
    $text = Replace-RequiredPdfField $text '<xmp:ModifyDate>[^<]+</xmp:ModifyDate>' '<xmp:ModifyDate>0000-00-00T00:00:00+00:00</xmp:ModifyDate>' 'XMP.xmp:ModifyDate'
    $text = Replace-RequiredPdfField $text '<xmp:CreateDate>[^<]+</xmp:CreateDate>' '<xmp:CreateDate>0000-00-00T00:00:00+00:00</xmp:CreateDate>' 'XMP.xmp:CreateDate'
    $text = Replace-RequiredPdfField $text '<xmpMM:InstanceID>[^<]+</xmpMM:InstanceID>' '<xmpMM:InstanceID>AAAAAAAAAAAAAAAAAAAAAA==</xmpMM:InstanceID>' 'XMP.xmpMM:InstanceID'
    $text = Replace-RequiredPdfField $text '(/ID \[\([^)]+\) \()[^)]+(\)\])' '$1AAAAAAAAAAAAAAAAAAAAAA==$2' 'Trailer.ID[1]'

    $bytes = $latin1.GetBytes($text)
    [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
}

function Get-RelativeFiles {
    param(
        [Parameter(Mandatory)]
        [string]$Root,
        [switch]$ExcludePdf
    )

    @(
        Get-ChildItem -LiteralPath $Root -Recurse -File |
            Where-Object { -not $ExcludePdf -or $_.Extension -ne '.pdf' } |
            ForEach-Object {
                $_.FullName.Substring($Root.Length + 1).Replace('\', '/')
            } |
            Sort-Object
    )
}

Push-Location $repoRoot
try {
    Write-Host 'AET Phase 1 baseline verification' -ForegroundColor Green
    Write-Host "Pre-change commit: $($manifest.baseline.pre_change_commit)"
    Write-Host "Pre-change tree:   $($manifest.baseline.pre_change_tree)"

    foreach ($command in 'git', 'rustc', 'cargo', 'typst') {
        if ($null -eq (Get-Command $command -ErrorAction SilentlyContinue)) {
            throw "Required command is not available on PATH: $command"
        }
    }

    Assert-Equal 'rustc version' $manifest.tool_versions.rustc ((& rustc --version) -join "`n")
    Assert-Equal 'cargo version' $manifest.tool_versions.cargo ((& cargo --version) -join "`n")
    Assert-Equal 'Typst version' $manifest.tool_versions.typst ((& typst --version) -join "`n")

    Write-Host "`nVerifying canonical fixture inputs..." -ForegroundColor Green
    foreach ($fixture in $manifest.fixture_inputs) {
        $path = Join-Path $repoRoot ([string]$fixture.path)
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Canonical fixture input is missing: $($fixture.path)"
        }
        Assert-Equal "$($fixture.path) byte length" ([string]$fixture.bytes) ([string](Get-Item -LiteralPath $path).Length)
        Assert-Equal "$($fixture.path) SHA-256" ([string]$fixture.sha256) (Get-Sha256 $path)
    }

    $checksumPath = Join-Path $repoRoot ([string]$manifest.deterministic_outputs.checksum_file)
    $checksumEntries = @(
        Get-Content -LiteralPath $checksumPath |
            Where-Object { $_.Trim().Length -gt 0 }
    )
    Assert-Equal 'snapshot checksum entry count' ([string]$manifest.deterministic_outputs.file_count) ([string]$checksumEntries.Count)
    foreach ($line in $checksumEntries) {
        if ($line -notmatch '^(?<hash>[0-9a-f]{64})  (?<path>.+)$') {
            throw "Invalid snapshot checksum line: $line"
        }
        $snapshotPath = Join-Path $repoRoot $Matches.path
        Assert-Equal "$($Matches.path) snapshot SHA-256" $Matches.hash (Get-Sha256 $snapshotPath)
    }

    Invoke-Checked cargo @('check', '--workspace', '--target-dir', '.codex-target')
    Invoke-Checked cargo @('test', '--workspace', '--target-dir', '.codex-target')

    $dist = [IO.Path]::GetFullPath((Join-Path $repoRoot 'dist'))
    $expectedDist = $repoRoot.TrimEnd('\') + '\dist'
    Assert-Equal 'generated output directory' $expectedDist $dist
    if (Test-Path -LiteralPath $dist) {
        Remove-Item -LiteralPath $dist -Recurse -Force
    }

    Invoke-Checked cargo @('run', '-p', 'aet-cli', '--', 'validate', 'data/articles/vietnam-two-child-policy')
    Invoke-Checked cargo @('run', '-p', 'aet-cli', '--', 'build', 'data/articles/vietnam-two-child-policy')
    Invoke-Checked cargo @('run', '-p', 'aet-cli', '--', 'validate', 'data/articles/dengue-bangladesh')
    Invoke-Checked cargo @('run', '-p', 'aet-cli', '--', 'build', 'data/articles/dengue-bangladesh')
    Invoke-Checked cargo @('run', '-p', 'aet-cli', '--', 'build-topic', 'health-medicine')

    Write-Host "`nComparing deterministic generated outputs..." -ForegroundColor Green
    $snapshotRoot = Join-Path $repoRoot ([string]$manifest.deterministic_outputs.snapshot_root)
    $snapshotFiles = Get-RelativeFiles $snapshotRoot
    $generatedFiles = Get-RelativeFiles $dist -ExcludePdf
    $fileSetDifference = @(Compare-Object -ReferenceObject $snapshotFiles -DifferenceObject $generatedFiles)
    if ($fileSetDifference.Count -gt 0) {
        throw "Deterministic generated file set changed:`n$($fileSetDifference | Out-String)"
    }
    foreach ($relative in $snapshotFiles) {
        $expected = Join-Path $snapshotRoot $relative
        $actual = Join-Path $dist $relative
        Assert-Equal "$relative generated SHA-256" (Get-Sha256 $expected) (Get-Sha256 $actual)
    }

    Write-Host "`nComparing normalized PDF outputs..." -ForegroundColor Green
    foreach ($pdf in $manifest.pdf_outputs) {
        $path = Join-Path $repoRoot ([string]$pdf.path)
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Generated PDF is missing: $($pdf.path)"
        }
        Assert-Equal "$($pdf.path) byte length" ([string]$pdf.bytes) ([string](Get-Item -LiteralPath $path).Length)
        Assert-Equal "$($pdf.path) normalized SHA-256" ([string]$pdf.normalized_sha256) (Get-NormalizedPdfSha256 $path)
    }

    Write-Host "`nPASS: AET Phase 1 baseline reproduced exactly." -ForegroundColor Green
    Write-Host "Deterministic files: $($snapshotFiles.Count)"
    Write-Host "Normalized PDFs:     $($manifest.pdf_outputs.Count)"
    Write-Host "Post-merge tag:      $($manifest.baseline.tag_after_merge)"
}
finally {
    Pop-Location
}
