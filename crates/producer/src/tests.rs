use super::*;

#[test]
fn an_unknown_build_runs_only_when_allowed() {
    let wrong = "0000000000000000000000000000000000000000000000000000000000000000";
    let Err(refused) = require_build("kernel32.dll", wrong) else { panic!("a wrong hash passed") };
    assert!(refused.reason.contains("Allow other game versions"), "{}", refused.reason);
    allow_other_builds(true);
    assert!(require_build("kernel32.dll", wrong).is_ok());
    allow_other_builds(false);
}

#[test]
fn the_game_meant_is_the_one_whose_options_file_the_launcher_wrote() {
    use manifest::Manifest;
    static TWO: [Manifest; 2] = [
        Manifest { id: "first", options_file: "first-options.txt", log: "monaka_first.log", ..Manifest::GAME },
        Manifest { id: "second", options_file: "second-options.txt", log: "monaka_second.log", ..Manifest::GAME },
    ];
    assert_eq!(meant(&TWO, |file| file == "second-options.txt").log, "monaka_second.log");
    assert_eq!(meant(&TWO, |_| false).id, "first", "none written: the first");
}
