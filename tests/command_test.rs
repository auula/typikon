use typikon::command::Command;

#[test]
fn test_command_from_str() {
    assert!(matches!(Command::from_str("build"), Command::Build));
    assert!(matches!(Command::from_str("theme"), Command::Theme));
    assert!(matches!(Command::from_str("init"), Command::Init));

    let unknown_command = Command::from_str("unknown");
    assert!(matches!(unknown_command, Command::Unknown(_)));
}
