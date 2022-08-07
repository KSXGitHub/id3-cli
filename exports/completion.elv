
use builtin;
use str;

set edit:completion:arg-completer[id3] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'id3'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'id3'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand backup 'Run backup without modification'
            cand get 'Show or export metadata'
            cand set 'Modify metadata'
            cand delete 'Delete metadata'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'id3;backup'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand title ''
            cand artist ''
            cand album ''
            cand album-artist ''
            cand genre-name ''
            cand genre-code ''
            cand comment ''
            cand picture ''
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'id3;get;title'= {
            cand --format 'Format the output text into JSON or YAML'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;artist'= {
            cand --format 'Format the output text into JSON or YAML'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;album'= {
            cand --format 'Format the output text into JSON or YAML'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;album-artist'= {
            cand --format 'Format the output text into JSON or YAML'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;genre-name'= {
            cand --format 'Format the output text into JSON or YAML'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;genre-code'= {
            cand --format 'Format the output text into JSON or YAML'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;comment'= {
            cand --language 'Filter language'
            cand --description 'Filter description'
            cand --format 'Format of the output text. Required if there are multiple comments'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;picture'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand list 'List descriptions, mime types, picture types, and sizes of all pictures'
            cand file 'Export a single picture to a file'
            cand dir 'Export all single pictures to a directory'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'id3;get;picture;list'= {
            cand --format 'Format of the output text'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;picture;file'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;picture;dir'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;get;picture;help'= {
        }
        &'id3;get;help'= {
        }
        &'id3;set'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand title ''
            cand artist ''
            cand album ''
            cand album-artist ''
            cand genre-code ''
            cand comment ''
            cand picture ''
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'id3;set;title'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;set;artist'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;set;album'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;set;album-artist'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;set;genre-code'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;set;comment'= {
            cand --language 'Comment language (ISO 639-2)'
            cand --description 'Comment description'
            cand --format 'Format of the ejected comment (if any)'
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;set;picture'= {
            cand --mime-type 'Mime type of the picture'
            cand --description 'Description of the picture'
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;set;help'= {
        }
        &'id3;delete'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand all 'Remove the whole ID3 tag from the audio'
            cand title ''
            cand artist ''
            cand album ''
            cand album-artist ''
            cand genre ''
            cand comment ''
            cand picture ''
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'id3;delete;all'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;title'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;artist'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;album'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;album-artist'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;genre'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;comment'= {
            cand --description 'Comment description'
            cand --content 'Comment content'
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture'= {
            cand --no-backup 'Don''t create backup for the target audio file'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand all 'Delete all pictures'
            cand Other 'Other'
            cand Icon 'Icon'
            cand OtherIcon 'OtherIcon'
            cand CoverFront 'CoverFront'
            cand CoverBack 'CoverBack'
            cand Leaflet 'Leaflet'
            cand Media 'Media'
            cand LeadArtist 'LeadArtist'
            cand Artist 'Artist'
            cand Conductor 'Conductor'
            cand Band 'Band'
            cand Composer 'Composer'
            cand Lyricist 'Lyricist'
            cand RecordingLocation 'RecordingLocation'
            cand DuringRecording 'DuringRecording'
            cand DuringPerformance 'DuringPerformance'
            cand ScreenCapture 'ScreenCapture'
            cand BrightFish 'BrightFish'
            cand Illustration 'Illustration'
            cand BandLogo 'BandLogo'
            cand PublisherLogo 'PublisherLogo'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'id3;delete;picture;all'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Other'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Icon'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;OtherIcon'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;CoverFront'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;CoverBack'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Leaflet'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Media'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;LeadArtist'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Artist'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Conductor'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Band'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Composer'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Lyricist'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;RecordingLocation'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;DuringRecording'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;DuringPerformance'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;ScreenCapture'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;BrightFish'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;Illustration'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;BandLogo'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;PublisherLogo'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'id3;delete;picture;help'= {
        }
        &'id3;delete;help'= {
        }
        &'id3;help'= {
        }
    ]
    $completions[$command]
}
