use std::process::Command as StdCommand;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_with_live_check() {
    // Start registry live check command as a background process
    let mut live_check_cmd = StdCommand::new("weaver")
        .args([
            "registry",
            "live-check",
            "-r",
            "model",
            "--inactivity-timeout",
            "4",
        ])
        .spawn()
        .expect("Failed to start registry live check process");

    // Allow live check command to initialize
    sleep(Duration::from_secs(2));

    // Run weaver-example command
    let example_cmd = StdCommand::new(env!("CARGO_BIN_EXE_weaver-example"))
        .output()
        .expect("Failed to execute weaver-example process");

    // Check that weaver-example command was successful
    assert!(
        example_cmd.status.success(),
        "weaver-example command failed: {}",
        String::from_utf8_lossy(&example_cmd.stderr)
    );

    // Wait for live check process to terminate due to inactivity timeout
    let status = live_check_cmd
        .wait()
        .expect("Failed to wait for live check process to terminate");

    // Verify the live check command exited ok
    assert!(
        status.success(),
        "Live check command did not exit successfully: {:?}",
        status
    );
}
