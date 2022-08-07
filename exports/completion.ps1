
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'id3' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'id3'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'id3' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('backup', 'backup', [CompletionResultType]::ParameterValue, 'Run backup without modification')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Show or export metadata')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Modify metadata')
            [CompletionResult]::new('delete', 'delete', [CompletionResultType]::ParameterValue, 'Delete metadata')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'id3;backup' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('title', 'title', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('artist', 'artist', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('album', 'album', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('album-artist', 'album-artist', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('genre-name', 'genre-name', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('genre-code', 'genre-code', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('comment', 'comment', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('picture', 'picture', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'id3;get;title' {
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Format the output text into JSON or YAML')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;artist' {
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Format the output text into JSON or YAML')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;album' {
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Format the output text into JSON or YAML')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;album-artist' {
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Format the output text into JSON or YAML')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;genre-name' {
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Format the output text into JSON or YAML')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;genre-code' {
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Format the output text into JSON or YAML')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;comment' {
            [CompletionResult]::new('--language', 'language', [CompletionResultType]::ParameterName, 'Filter language')
            [CompletionResult]::new('--description', 'description', [CompletionResultType]::ParameterName, 'Filter description')
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Format of the output text. Required if there are multiple comments')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;picture' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List descriptions, mime types, picture types, and sizes of all pictures')
            [CompletionResult]::new('file', 'file', [CompletionResultType]::ParameterValue, 'Export a single picture to a file')
            [CompletionResult]::new('dir', 'dir', [CompletionResultType]::ParameterValue, 'Export all single pictures to a directory')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'id3;get;picture;list' {
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Format of the output text')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;picture;file' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;picture;dir' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;get;picture;help' {
            break
        }
        'id3;get;help' {
            break
        }
        'id3;set' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('title', 'title', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('artist', 'artist', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('album', 'album', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('album-artist', 'album-artist', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('genre-code', 'genre-code', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('comment', 'comment', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('picture', 'picture', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'id3;set;title' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;set;artist' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;set;album' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;set;album-artist' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;set;genre-code' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;set;comment' {
            [CompletionResult]::new('--language', 'language', [CompletionResultType]::ParameterName, 'Comment language (ISO 639-2)')
            [CompletionResult]::new('--description', 'description', [CompletionResultType]::ParameterName, 'Comment description')
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Format of the ejected comment (if any)')
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;set;picture' {
            [CompletionResult]::new('--mime-type', 'mime-type', [CompletionResultType]::ParameterName, 'Mime type of the picture')
            [CompletionResult]::new('--description', 'description', [CompletionResultType]::ParameterName, 'Description of the picture')
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;set;help' {
            break
        }
        'id3;delete' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('all', 'all', [CompletionResultType]::ParameterValue, 'Remove the whole ID3 tag from the audio')
            [CompletionResult]::new('title', 'title', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('artist', 'artist', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('album', 'album', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('album-artist', 'album-artist', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('genre', 'genre', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('comment', 'comment', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('picture', 'picture', [CompletionResultType]::ParameterValue, '')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'id3;delete;all' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;title' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;artist' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;album' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;album-artist' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;genre' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;comment' {
            [CompletionResult]::new('--description', 'description', [CompletionResultType]::ParameterName, 'Comment description')
            [CompletionResult]::new('--content', 'content', [CompletionResultType]::ParameterName, 'Comment content')
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture' {
            [CompletionResult]::new('--no-backup', 'no-backup', [CompletionResultType]::ParameterName, 'Don''t create backup for the target audio file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('all', 'all', [CompletionResultType]::ParameterValue, 'Delete all pictures')
            [CompletionResult]::new('Other', 'Other', [CompletionResultType]::ParameterValue, 'Other')
            [CompletionResult]::new('Icon', 'Icon', [CompletionResultType]::ParameterValue, 'Icon')
            [CompletionResult]::new('OtherIcon', 'OtherIcon', [CompletionResultType]::ParameterValue, 'OtherIcon')
            [CompletionResult]::new('CoverFront', 'CoverFront', [CompletionResultType]::ParameterValue, 'CoverFront')
            [CompletionResult]::new('CoverBack', 'CoverBack', [CompletionResultType]::ParameterValue, 'CoverBack')
            [CompletionResult]::new('Leaflet', 'Leaflet', [CompletionResultType]::ParameterValue, 'Leaflet')
            [CompletionResult]::new('Media', 'Media', [CompletionResultType]::ParameterValue, 'Media')
            [CompletionResult]::new('LeadArtist', 'LeadArtist', [CompletionResultType]::ParameterValue, 'LeadArtist')
            [CompletionResult]::new('Artist', 'Artist', [CompletionResultType]::ParameterValue, 'Artist')
            [CompletionResult]::new('Conductor', 'Conductor', [CompletionResultType]::ParameterValue, 'Conductor')
            [CompletionResult]::new('Band', 'Band', [CompletionResultType]::ParameterValue, 'Band')
            [CompletionResult]::new('Composer', 'Composer', [CompletionResultType]::ParameterValue, 'Composer')
            [CompletionResult]::new('Lyricist', 'Lyricist', [CompletionResultType]::ParameterValue, 'Lyricist')
            [CompletionResult]::new('RecordingLocation', 'RecordingLocation', [CompletionResultType]::ParameterValue, 'RecordingLocation')
            [CompletionResult]::new('DuringRecording', 'DuringRecording', [CompletionResultType]::ParameterValue, 'DuringRecording')
            [CompletionResult]::new('DuringPerformance', 'DuringPerformance', [CompletionResultType]::ParameterValue, 'DuringPerformance')
            [CompletionResult]::new('ScreenCapture', 'ScreenCapture', [CompletionResultType]::ParameterValue, 'ScreenCapture')
            [CompletionResult]::new('BrightFish', 'BrightFish', [CompletionResultType]::ParameterValue, 'BrightFish')
            [CompletionResult]::new('Illustration', 'Illustration', [CompletionResultType]::ParameterValue, 'Illustration')
            [CompletionResult]::new('BandLogo', 'BandLogo', [CompletionResultType]::ParameterValue, 'BandLogo')
            [CompletionResult]::new('PublisherLogo', 'PublisherLogo', [CompletionResultType]::ParameterValue, 'PublisherLogo')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'id3;delete;picture;all' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Other' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Icon' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;OtherIcon' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;CoverFront' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;CoverBack' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Leaflet' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Media' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;LeadArtist' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Artist' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Conductor' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Band' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Composer' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Lyricist' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;RecordingLocation' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;DuringRecording' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;DuringPerformance' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;ScreenCapture' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;BrightFish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;Illustration' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;BandLogo' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;PublisherLogo' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'id3;delete;picture;help' {
            break
        }
        'id3;delete;help' {
            break
        }
        'id3;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
