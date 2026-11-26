$ErrorActionPreference = 'Stop'

$courseRoot = 'D:\OneDrive\01. IELTS\00.Academic English'
$metadataRoot = 'D:\Work\aet\.course-staging\2026-09-06-more\metadata'

function Copy-UpdatedFile {
    param(
        [Parameter(Mandatory)] [string] $Source,
        [Parameter(Mandatory)] [string] $Destination
    )

    if (-not (Test-Path -LiteralPath $Destination -PathType Leaf)) {
        throw "Expected tracker is missing: $Destination"
    }
    Copy-Item -LiteralPath $Source -Destination $Destination -Force
}

Copy-UpdatedFile "$metadataRoot\root\CURRENT.md" "$courseRoot\CURRENT.md"
Copy-UpdatedFile "$metadataRoot\core\core-manifest.csv" "$courseRoot\00. Core\core-manifest.csv"
Copy-UpdatedFile "$metadataRoot\core\README.md" "$courseRoot\00. Core\README.md"
Copy-UpdatedFile "$metadataRoot\lecture23\source-post-index.md" "$courseRoot\Lectures\23. lecture-23\00. class-info\source-post-index.md"
Copy-UpdatedFile "$metadataRoot\lecture23\homework-lecture-23.md" "$courseRoot\Lectures\23. lecture-23\00. class-info\homework-lecture-23.md"
Copy-UpdatedFile "$metadataRoot\lecture23\class-notes.md" "$courseRoot\Lectures\23. lecture-23\00. class-info\class-notes.md"
Copy-UpdatedFile "$metadataRoot\ielts-reading\reading-tracker.md" "$courseRoot\IELTS Test\01. reading\reading-tracker.md"
Copy-UpdatedFile "$metadataRoot\ielts-writing\writing-tracker.md" "$courseRoot\IELTS Test\03. writing\writing-tracker.md"
Copy-UpdatedFile "$metadataRoot\ielts-root\README.md" "$courseRoot\IELTS Test\README.md"
Copy-UpdatedFile "$metadataRoot\sentence04\review-checklist.md" "$courseRoot\Sentence\04. tenses-and-time-shifts\review-checklist.md"
Copy-UpdatedFile "$metadataRoot\writing19\review-checklist.md" "$courseRoot\Writing\19. two-part-question-essay\review-checklist.md"
Copy-UpdatedFile "$metadataRoot\writing21\review-checklist.md" "$courseRoot\Writing\21. ielts-writing-task-1\review-checklist.md"

$dailyHighlights = "$courseRoot\Daily Reading\Original PDF\51.In Many Countries the Biggest Barrier to Prosperity Is in the Home\highlights.md"
if (Test-Path -LiteralPath $dailyHighlights) {
    throw "Refusing to overwrite new Daily Reading tracker: $dailyHighlights"
}
Copy-Item -LiteralPath "$metadataRoot\daily\highlights.md" -Destination $dailyHighlights

Write-Output 'Updated 12 existing trackers and added the Daily Reading 51 tracker.'
