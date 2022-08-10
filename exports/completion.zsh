#compdef id3

autoload -U is-at-least

_id3() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
":: :_id3_commands" \
"*::: :->id3" \
&& ret=0
    case $state in
    (id3)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:id3-command-$line[1]:"
        case $line[1] in
            (backup)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
&& ret=0
;;
(get)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
":: :_id3__get_commands" \
"*::: :->get" \
&& ret=0

    case $state in
    (get)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:id3-get-command-$line[1]:"
        case $line[1] in
            (title)
_arguments "${_arguments_options[@]}" \
'--format=[Format the output text into JSON or YAML]:FORMAT:(json yaml)' \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
&& ret=0
;;
(artist)
_arguments "${_arguments_options[@]}" \
'--format=[Format the output text into JSON or YAML]:FORMAT:(json yaml)' \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
&& ret=0
;;
(album)
_arguments "${_arguments_options[@]}" \
'--format=[Format the output text into JSON or YAML]:FORMAT:(json yaml)' \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
&& ret=0
;;
(album-artist)
_arguments "${_arguments_options[@]}" \
'--format=[Format the output text into JSON or YAML]:FORMAT:(json yaml)' \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
&& ret=0
;;
(genre-name)
_arguments "${_arguments_options[@]}" \
'--format=[Format the output text into JSON or YAML]:FORMAT:(json yaml)' \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
&& ret=0
;;
(genre-code)
_arguments "${_arguments_options[@]}" \
'--format=[Format the output text into JSON or YAML]:FORMAT:(json yaml)' \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
&& ret=0
;;
(comment)
_arguments "${_arguments_options[@]}" \
'--language=[Filter language]:LANGUAGE: ' \
'--description=[Filter description]:DESCRIPTION: ' \
'--format=[Format of the output text. Required if there are multiple comments]:FORMAT:(json yaml)' \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
&& ret=0
;;
(picture)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
":: :_id3__get__picture_commands" \
"*::: :->picture" \
&& ret=0

    case $state in
    (picture)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:id3-get-picture-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" \
'--format=[Format of the output text]:FORMAT:(json yaml)' \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
&& ret=0
;;
(file)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
':output-picture -- Path to the output picture file:' \
'::picture-type -- Type of picture to export. Required if there are multiple pictures:(Other Icon OtherIcon CoverFront CoverBack Leaflet Media LeadArtist Artist Conductor Band Composer Lyricist RecordingLocation DuringRecording DuringPerformance ScreenCapture BrightFish Illustration BandLogo PublisherLogo)' \
&& ret=0
;;
(dir)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
':input-audio -- Path to the input audio file:' \
':output-directory -- Path to the directory to contain the output pictures:' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'*::subcommand -- The subcommand whose help message to display:' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'*::subcommand -- The subcommand whose help message to display:' \
&& ret=0
;;
        esac
    ;;
esac
;;
(set)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
":: :_id3__set_commands" \
"*::: :->set" \
&& ret=0

    case $state in
    (set)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:id3-set-command-$line[1]:"
        case $line[1] in
            (title)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
':value -- New value to set:' \
&& ret=0
;;
(artist)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
':value -- New value to set:' \
&& ret=0
;;
(album)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
':value -- New value to set:' \
&& ret=0
;;
(album-artist)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
':value -- New value to set:' \
&& ret=0
;;
(genre-code)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
':value -- New value to set:' \
&& ret=0
;;
(comment)
_arguments "${_arguments_options[@]}" \
'--language=[Comment language (ISO 639-2)]:LANGUAGE: ' \
'--description=[Comment description]:DESCRIPTION: ' \
'--format=[Format of the ejected comment (if any)]:FORMAT:(json yaml)' \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
':content -- Content of the comment:' \
&& ret=0
;;
(picture)
_arguments "${_arguments_options[@]}" \
'--mime-type=[Mime type of the picture]:MIME_TYPE: ' \
'--description=[Description of the picture]:DESCRIPTION: ' \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
':target-picture -- Path to the input picture file:' \
':picture-type -- Type of picture:(Other Icon OtherIcon CoverFront CoverBack Leaflet Media LeadArtist Artist Conductor Band Composer Lyricist RecordingLocation DuringRecording DuringPerformance ScreenCapture BrightFish Illustration BandLogo PublisherLogo)' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'*::subcommand -- The subcommand whose help message to display:' \
&& ret=0
;;
        esac
    ;;
esac
;;
(delete)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
":: :_id3__delete_commands" \
"*::: :->delete" \
&& ret=0

    case $state in
    (delete)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:id3-delete-command-$line[1]:"
        case $line[1] in
            (all)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
&& ret=0
;;
(title)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
&& ret=0
;;
(artist)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
&& ret=0
;;
(album)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
&& ret=0
;;
(album-artist)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
&& ret=0
;;
(genre)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
&& ret=0
;;
(comment)
_arguments "${_arguments_options[@]}" \
'--description=[Comment description]:DESCRIPTION: ' \
'--content=[Comment content]:CONTENT: ' \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
&& ret=0
;;
(picture)
_arguments "${_arguments_options[@]}" \
'--no-backup[Don'\''t create backup for the target audio file]' \
'-h[Print help information]' \
'--help[Print help information]' \
':target-audio -- Path to the target audio file:' \
":: :_id3__delete__picture_commands" \
"*::: :->picture" \
&& ret=0

    case $state in
    (picture)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:id3-delete-picture-command-$line[2]:"
        case $line[2] in
            (all)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Other)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Icon)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(OtherIcon)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(CoverFront)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(CoverBack)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Leaflet)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Media)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(LeadArtist)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Artist)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Conductor)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Band)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Composer)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Lyricist)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(RecordingLocation)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(DuringRecording)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(DuringPerformance)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(ScreenCapture)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(BrightFish)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(Illustration)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(BandLogo)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(PublisherLogo)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'*::subcommand -- The subcommand whose help message to display:' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'*::subcommand -- The subcommand whose help message to display:' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'*::subcommand -- The subcommand whose help message to display:' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_id3_commands] )) ||
_id3_commands() {
    local commands; commands=(
'backup:Run backup without modification' \
'get:Show or export metadata' \
'set:Modify metadata' \
'delete:Delete metadata' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'id3 commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Artist_commands] )) ||
_id3__delete__picture__Artist_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Artist commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Band_commands] )) ||
_id3__delete__picture__Band_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Band commands' commands "$@"
}
(( $+functions[_id3__delete__picture__BandLogo_commands] )) ||
_id3__delete__picture__BandLogo_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture BandLogo commands' commands "$@"
}
(( $+functions[_id3__delete__picture__BrightFish_commands] )) ||
_id3__delete__picture__BrightFish_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture BrightFish commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Composer_commands] )) ||
_id3__delete__picture__Composer_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Composer commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Conductor_commands] )) ||
_id3__delete__picture__Conductor_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Conductor commands' commands "$@"
}
(( $+functions[_id3__delete__picture__CoverBack_commands] )) ||
_id3__delete__picture__CoverBack_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture CoverBack commands' commands "$@"
}
(( $+functions[_id3__delete__picture__CoverFront_commands] )) ||
_id3__delete__picture__CoverFront_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture CoverFront commands' commands "$@"
}
(( $+functions[_id3__delete__picture__DuringPerformance_commands] )) ||
_id3__delete__picture__DuringPerformance_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture DuringPerformance commands' commands "$@"
}
(( $+functions[_id3__delete__picture__DuringRecording_commands] )) ||
_id3__delete__picture__DuringRecording_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture DuringRecording commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Icon_commands] )) ||
_id3__delete__picture__Icon_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Icon commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Illustration_commands] )) ||
_id3__delete__picture__Illustration_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Illustration commands' commands "$@"
}
(( $+functions[_id3__delete__picture__LeadArtist_commands] )) ||
_id3__delete__picture__LeadArtist_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture LeadArtist commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Leaflet_commands] )) ||
_id3__delete__picture__Leaflet_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Leaflet commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Lyricist_commands] )) ||
_id3__delete__picture__Lyricist_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Lyricist commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Media_commands] )) ||
_id3__delete__picture__Media_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Media commands' commands "$@"
}
(( $+functions[_id3__delete__picture__Other_commands] )) ||
_id3__delete__picture__Other_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture Other commands' commands "$@"
}
(( $+functions[_id3__delete__picture__OtherIcon_commands] )) ||
_id3__delete__picture__OtherIcon_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture OtherIcon commands' commands "$@"
}
(( $+functions[_id3__delete__picture__PublisherLogo_commands] )) ||
_id3__delete__picture__PublisherLogo_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture PublisherLogo commands' commands "$@"
}
(( $+functions[_id3__delete__picture__RecordingLocation_commands] )) ||
_id3__delete__picture__RecordingLocation_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture RecordingLocation commands' commands "$@"
}
(( $+functions[_id3__delete__picture__ScreenCapture_commands] )) ||
_id3__delete__picture__ScreenCapture_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture ScreenCapture commands' commands "$@"
}
(( $+functions[_id3__delete__album_commands] )) ||
_id3__delete__album_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete album commands' commands "$@"
}
(( $+functions[_id3__get__album_commands] )) ||
_id3__get__album_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get album commands' commands "$@"
}
(( $+functions[_id3__set__album_commands] )) ||
_id3__set__album_commands() {
    local commands; commands=()
    _describe -t commands 'id3 set album commands' commands "$@"
}
(( $+functions[_id3__delete__album-artist_commands] )) ||
_id3__delete__album-artist_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete album-artist commands' commands "$@"
}
(( $+functions[_id3__get__album-artist_commands] )) ||
_id3__get__album-artist_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get album-artist commands' commands "$@"
}
(( $+functions[_id3__set__album-artist_commands] )) ||
_id3__set__album-artist_commands() {
    local commands; commands=()
    _describe -t commands 'id3 set album-artist commands' commands "$@"
}
(( $+functions[_id3__delete__all_commands] )) ||
_id3__delete__all_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete all commands' commands "$@"
}
(( $+functions[_id3__delete__picture__all_commands] )) ||
_id3__delete__picture__all_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture all commands' commands "$@"
}
(( $+functions[_id3__delete__artist_commands] )) ||
_id3__delete__artist_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete artist commands' commands "$@"
}
(( $+functions[_id3__get__artist_commands] )) ||
_id3__get__artist_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get artist commands' commands "$@"
}
(( $+functions[_id3__set__artist_commands] )) ||
_id3__set__artist_commands() {
    local commands; commands=()
    _describe -t commands 'id3 set artist commands' commands "$@"
}
(( $+functions[_id3__backup_commands] )) ||
_id3__backup_commands() {
    local commands; commands=()
    _describe -t commands 'id3 backup commands' commands "$@"
}
(( $+functions[_id3__delete__comment_commands] )) ||
_id3__delete__comment_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete comment commands' commands "$@"
}
(( $+functions[_id3__get__comment_commands] )) ||
_id3__get__comment_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get comment commands' commands "$@"
}
(( $+functions[_id3__set__comment_commands] )) ||
_id3__set__comment_commands() {
    local commands; commands=()
    _describe -t commands 'id3 set comment commands' commands "$@"
}
(( $+functions[_id3__delete_commands] )) ||
_id3__delete_commands() {
    local commands; commands=(
'all:Remove the whole ID3 tag from the audio' \
'title:' \
'artist:' \
'album:' \
'album-artist:' \
'genre:' \
'comment:' \
'picture:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'id3 delete commands' commands "$@"
}
(( $+functions[_id3__get__picture__dir_commands] )) ||
_id3__get__picture__dir_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get picture dir commands' commands "$@"
}
(( $+functions[_id3__get__picture__file_commands] )) ||
_id3__get__picture__file_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get picture file commands' commands "$@"
}
(( $+functions[_id3__delete__genre_commands] )) ||
_id3__delete__genre_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete genre commands' commands "$@"
}
(( $+functions[_id3__get__genre-code_commands] )) ||
_id3__get__genre-code_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get genre-code commands' commands "$@"
}
(( $+functions[_id3__set__genre-code_commands] )) ||
_id3__set__genre-code_commands() {
    local commands; commands=()
    _describe -t commands 'id3 set genre-code commands' commands "$@"
}
(( $+functions[_id3__get__genre-name_commands] )) ||
_id3__get__genre-name_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get genre-name commands' commands "$@"
}
(( $+functions[_id3__get_commands] )) ||
_id3__get_commands() {
    local commands; commands=(
'title:' \
'artist:' \
'album:' \
'album-artist:' \
'genre-name:' \
'genre-code:' \
'comment:' \
'picture:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'id3 get commands' commands "$@"
}
(( $+functions[_id3__delete__help_commands] )) ||
_id3__delete__help_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete help commands' commands "$@"
}
(( $+functions[_id3__delete__picture__help_commands] )) ||
_id3__delete__picture__help_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete picture help commands' commands "$@"
}
(( $+functions[_id3__get__help_commands] )) ||
_id3__get__help_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get help commands' commands "$@"
}
(( $+functions[_id3__get__picture__help_commands] )) ||
_id3__get__picture__help_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get picture help commands' commands "$@"
}
(( $+functions[_id3__help_commands] )) ||
_id3__help_commands() {
    local commands; commands=()
    _describe -t commands 'id3 help commands' commands "$@"
}
(( $+functions[_id3__set__help_commands] )) ||
_id3__set__help_commands() {
    local commands; commands=()
    _describe -t commands 'id3 set help commands' commands "$@"
}
(( $+functions[_id3__get__picture__list_commands] )) ||
_id3__get__picture__list_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get picture list commands' commands "$@"
}
(( $+functions[_id3__delete__picture_commands] )) ||
_id3__delete__picture_commands() {
    local commands; commands=(
'all:Delete all pictures' \
'Other:' \
'Icon:' \
'OtherIcon:' \
'CoverFront:' \
'CoverBack:' \
'Leaflet:' \
'Media:' \
'LeadArtist:' \
'Artist:' \
'Conductor:' \
'Band:' \
'Composer:' \
'Lyricist:' \
'RecordingLocation:' \
'DuringRecording:' \
'DuringPerformance:' \
'ScreenCapture:' \
'BrightFish:' \
'Illustration:' \
'BandLogo:' \
'PublisherLogo:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'id3 delete picture commands' commands "$@"
}
(( $+functions[_id3__get__picture_commands] )) ||
_id3__get__picture_commands() {
    local commands; commands=(
'list:List descriptions, mime types, picture types, and sizes of all pictures' \
'file:Export a single picture to a file' \
'dir:Export all single pictures to a directory' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'id3 get picture commands' commands "$@"
}
(( $+functions[_id3__set__picture_commands] )) ||
_id3__set__picture_commands() {
    local commands; commands=()
    _describe -t commands 'id3 set picture commands' commands "$@"
}
(( $+functions[_id3__set_commands] )) ||
_id3__set_commands() {
    local commands; commands=(
'title:' \
'artist:' \
'album:' \
'album-artist:' \
'genre-code:' \
'comment:' \
'picture:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'id3 set commands' commands "$@"
}
(( $+functions[_id3__delete__title_commands] )) ||
_id3__delete__title_commands() {
    local commands; commands=()
    _describe -t commands 'id3 delete title commands' commands "$@"
}
(( $+functions[_id3__get__title_commands] )) ||
_id3__get__title_commands() {
    local commands; commands=()
    _describe -t commands 'id3 get title commands' commands "$@"
}
(( $+functions[_id3__set__title_commands] )) ||
_id3__set__title_commands() {
    local commands; commands=()
    _describe -t commands 'id3 set title commands' commands "$@"
}

_id3 "$@"
