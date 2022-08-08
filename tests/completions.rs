use clap_complete::Shell;
use clap_utilities::CommandFactoryExtra;
use id3_cli::app::App;

macro_rules! check {
    ($name:ident: $shell:ident => $path:literal) => {
        #[test]
        fn $name() {
            eprintln!(
                "check!({name}: {shell} => {path});",
                name = stringify!($name),
                shell = stringify!($shell),
                path = $path,
            );
            let received =
                App::get_completion_string("id3", Shell::$shell).expect("get completion string");
            let expected = include_str!($path);
            assert!(received == expected, "completion is outdated");
        }
    };
}

check!(bash: Bash => "../exports/completion.bash");
check!(fish: Fish => "../exports/completion.fish");
check!(zsh: Zsh => "../exports/completion.zsh");
check!(powershell: PowerShell => "../exports/completion.ps1");
check!(elvish: Elvish => "../exports/completion.elv");
