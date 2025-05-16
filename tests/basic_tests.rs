use remindr::db::Command;

#[test]
fn test_command_creation() {
    let cmd = Command::new("git status");
    assert_eq!(cmd.command, "git status");
    assert!(cmd.id.is_none());
    assert!(cmd.categorization.is_none());
    assert!(cmd.tags.is_none());
    assert!(cmd.context.is_none());
}

#[test]
fn test_version() {
    let version = remindr::version();
    assert!(!version.is_empty());
}

#[test]
fn test_name() {
    let name = remindr::name();
    assert_eq!(name, "remindr");
} 