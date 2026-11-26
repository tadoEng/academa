$ErrorActionPreference = 'Stop'

$courseRoot = 'D:\OneDrive\01. IELTS\00.Academic English'
$intakeRoot = 'D:\Work\aet\.course-staging\2026-09-06-more'

function Copy-NewFile {
    param(
        [Parameter(Mandatory)] [string] $Source,
        [Parameter(Mandatory)] [string] $Destination
    )

    if (Test-Path -LiteralPath $Destination) {
        throw "Refusing to overwrite existing file: $Destination"
    }

    $destinationDirectory = Split-Path -Parent $Destination
    New-Item -ItemType Directory -Force -Path $destinationDirectory | Out-Null
    Copy-Item -LiteralPath $Source -Destination $Destination
}

$tensesSource = Join-Path $intakeRoot 'tenses-direct.pdf'
$tensesName = 'tenses-and-time-shift-book-2024.pdf'
Copy-NewFile $tensesSource (Join-Path $courseRoot "Sentence\04. tenses-and-time-shifts\$tensesName")
Copy-NewFile $tensesSource (Join-Path $courseRoot "IELTS Test\05. vocabulary-grammar\materials\23. Tenses and Time Shifts\$tensesName")
Copy-NewFile $tensesSource (Join-Path $courseRoot "00. Core\02. error-catalogue\materials\S08-tense\$tensesName")
Copy-NewFile $tensesSource (Join-Path $courseRoot "Lectures\23. lecture-23\01. pre-class-materials\$tensesName")

$readingTitle = '51.In Many Countries the Biggest Barrier to Prosperity Is in the Home'
$readingDaily = Join-Path $courseRoot "Daily Reading\Original PDF\$readingTitle"
$readingIelts = Join-Path $courseRoot "IELTS Test\01. reading\materials\51. In Many Countries the Biggest Barrier to Prosperity Is in the Home"
$readingCore = Join-Path $courseRoot '00. Core\07. listening\materials\51. in-many-countries-the-biggest-barrier-to-prosperity-is-in-the-home'
$readingFiles = @(
    @{ Source = 'dr51-article.pdf'; Name = 'article.pdf' },
    @{ Source = 'dr51-ielts-questions.pdf'; Name = 'ielts-reading-questions.pdf' },
    @{ Source = 'dr51-vocabulary-companion.pdf'; Name = 'vocabulary-companion.pdf' },
    @{ Source = 'dr51-vocabulary-exercises.pdf'; Name = 'vocabulary-exercises.pdf' },
    @{ Source = 'dr51-audio.mp3'; Name = 'audio.mp3' }
)
foreach ($item in $readingFiles) {
    Copy-NewFile (Join-Path $intakeRoot $item.Source) (Join-Path $readingDaily $item.Name)
    Copy-NewFile (Join-Path $intakeRoot $item.Source) (Join-Path $readingCore $item.Name)
}
Copy-NewFile (Join-Path $intakeRoot 'dr51-article.pdf') (Join-Path $readingIelts 'article.pdf')
Copy-NewFile (Join-Path $intakeRoot 'dr51-ielts-questions.pdf') (Join-Path $readingIelts 'ielts-reading-questions.pdf')

$twoPartDirect = Join-Path $intakeRoot 'two-part-direct-questions.pdf'
$twoPartScaffold = Join-Path $intakeRoot 'two-part-children-smartphones.pdf'
$twoPartDirectName = 'two-part-question-direct-questions.pdf'
$twoPartScaffoldName = 'two-part-question-children-smartphones-scaffold.pdf'
$twoPartDestinations = @(
    (Join-Path $courseRoot 'Writing\19. two-part-question-essay'),
    (Join-Path $courseRoot 'IELTS Test\03. writing\materials\12. Two-part Question Essay'),
    (Join-Path $courseRoot '00. Core\03. writing\materials\02. question-types'),
    (Join-Path $courseRoot 'Lectures\23. lecture-23\01. pre-class-materials')
)
foreach ($destinationDirectory in $twoPartDestinations) {
    Copy-NewFile $twoPartDirect (Join-Path $destinationDirectory $twoPartDirectName)
    Copy-NewFile $twoPartScaffold (Join-Path $destinationDirectory $twoPartScaffoldName)
}

$task1Source = Join-Path $intakeRoot 'task1-the-task-and-the-report-2026-09-06.pdf'
$task1Name = 'ielts-writing-task1-the-task-and-the-report-2026-09-06.pdf'
$task1Destinations = @(
    (Join-Path $courseRoot 'Writing\21. ielts-writing-task-1'),
    (Join-Path $courseRoot 'IELTS Test\03. writing\materials\13. IELTS Writing Task 1'),
    (Join-Path $courseRoot '00. Core\03. writing\materials\13. task1-report'),
    (Join-Path $courseRoot 'Lectures\23. lecture-23\01. pre-class-materials')
)
foreach ($destinationDirectory in $task1Destinations) {
    Copy-NewFile $task1Source (Join-Path $destinationDirectory $task1Name)
}

Write-Output 'Copied 28 new role copies without overwriting any existing file.'
