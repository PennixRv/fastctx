mod common;

use common::{McpSession, mcp_text, normalized};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[test]
fn jobs_kill_manages_a_persistent_job_and_is_idempotent() {
    let temp = tempfile::tempdir().unwrap();
    write_shell_settings(temp.path(), true);
    let job_id = start_persistent_job(temp.path(), "sleep 60");
    let mut cleanup = BackgroundJobCleanup::new(temp.path(), &job_id);

    let listed = isolated_command(temp.path()).arg("jobs").output().unwrap();
    assert_success(&listed);
    let listed = String::from_utf8(listed.stdout).unwrap();
    assert!(listed.contains(&job_id), "{listed}");
    assert!(listed.contains("sleep 60"), "{listed}");

    let killed = isolated_command(temp.path())
        .args(["jobs", "kill", &job_id])
        .output()
        .unwrap();
    assert_success(&killed);
    assert_eq!(
        String::from_utf8(killed.stdout).unwrap(),
        format!("Job {job_id} killed.\n")
    );
    let idempotent = isolated_command(temp.path())
        .args(["jobs", "kill", &job_id])
        .output()
        .unwrap();
    assert_success(&idempotent);
    let idempotent = String::from_utf8(idempotent.stdout).unwrap();
    assert!(
        idempotent.starts_with(&format!("Job {job_id} had already exited with code ")),
        "{idempotent}"
    );
    let empty = isolated_command(temp.path()).arg("jobs").output().unwrap();
    assert_success(&empty);
    assert_eq!(
        String::from_utf8(empty.stdout).unwrap(),
        "No running jobs.\n"
    );
    cleanup.disarm();
}

#[test]
fn unapply_stops_a_real_persistent_job_before_removing_fastctx_data() {
    let temp = tempfile::tempdir().unwrap();
    write_shell_settings(temp.path(), true);
    let applied = isolated_command(temp.path())
        .args(["apply", "--guidance", "managed", "--yes"])
        .output()
        .unwrap();
    assert_success(&applied);

    let job_id = start_persistent_job(temp.path(), "sleep 60");
    let mut cleanup = BackgroundJobCleanup::new(temp.path(), &job_id);

    let running = isolated_command(temp.path()).arg("jobs").output().unwrap();
    assert_success(&running);
    let running = String::from_utf8(running.stdout).unwrap();
    assert!(running.contains(&job_id), "{running}");
    assert!(running.contains("sleep 60"), "{running}");

    let removed = isolated_command(temp.path())
        .args(["unapply", "--yes"])
        .output()
        .unwrap();
    assert_success(&removed);
    let output = String::from_utf8(removed.stdout).unwrap();
    assert!(
        output.contains("Stop      1 running background job before removal"),
        "{output}"
    );
    assert!(
        output.contains("Stopped 1 running background job before removal."),
        "{output}"
    );
    assert!(!temp.path().join(".fastctx").exists());
    cleanup.disarm();
}

#[test]
fn codex_home_env_selects_the_profile_without_moving_fastctx_state() {
    let temp = profile_test_home();
    let profile = temp.path().join("relocated-codex-profile");

    let applied = isolated_command(temp.path())
        .args(["apply", "--guidance", "managed", "--yes"])
        .env("CODEX_HOME", &profile)
        .output()
        .unwrap();
    assert_success(&applied);
    assert!(profile.join("config.toml").is_file());
    assert!(profile.join("AGENTS.md").is_file());
    assert!(!profile.join(".codex").exists());
    assert!(!temp.path().join(".codex").exists());
    assert!(temp.path().join(".fastctx/config.toml").is_file());

    let status = isolated_command(temp.path())
        .arg("status")
        .env("CODEX_HOME", &profile)
        .output()
        .unwrap();
    assert_success(&status);
    let status = String::from_utf8(status.stdout).unwrap();
    assert!(status.contains("[PASS] Codex profile"), "{status}");
    assert!(status.contains(&normalized(&profile)), "{status}");
    assert!(status.contains("source: env"), "{status}");

    let removed = isolated_command(temp.path())
        .args(["unapply", "--yes"])
        .env("CODEX_HOME", &profile)
        .output()
        .unwrap();
    assert_success(&removed);
    assert!(!profile.exists());
    assert!(!temp.path().join(".fastctx").exists());
}

#[test]
fn codex_home_flag_overrides_the_live_environment_for_all_control_commands() {
    let temp = profile_test_home();
    let environment_profile = temp.path().join("environment-profile");
    let flag_profile = temp.path().join("flag-profile");

    let applied = isolated_command(temp.path())
        .arg("apply")
        .arg("--codex-home")
        .arg(&flag_profile)
        .arg("--yes")
        .env("CODEX_HOME", &environment_profile)
        .output()
        .unwrap();
    assert_success(&applied);
    assert!(flag_profile.join("config.toml").is_file());
    assert!(!environment_profile.exists());

    for subcommand in ["status", "doctor"] {
        let status = isolated_command(temp.path())
            .arg(subcommand)
            .arg("--codex-home")
            .arg(&flag_profile)
            .env("CODEX_HOME", &environment_profile)
            .output()
            .unwrap();
        assert_success(&status);
        let status = String::from_utf8(status.stdout).unwrap();
        assert!(status.contains(&normalized(&flag_profile)), "{status}");
        assert!(status.contains("source: flag"), "{status}");
        assert!(
            !status.contains(&normalized(&environment_profile)),
            "{status}"
        );
    }

    let switched_status = isolated_command(temp.path())
        .arg("status")
        .env("CODEX_HOME", &environment_profile)
        .output()
        .unwrap();
    assert_success(&switched_status);
    let switched_status = String::from_utf8(switched_status.stdout).unwrap();
    assert!(switched_status.contains("source: env"), "{switched_status}");
    assert!(
        switched_status.contains("[INFO] Applied state"),
        "{switched_status}"
    );
    assert!(
        switched_status.contains("saved Apply receipt targets"),
        "{switched_status}"
    );

    let mismatched_apply = isolated_command(temp.path())
        .args(["apply", "--yes"])
        .env("CODEX_HOME", &environment_profile)
        .output()
        .unwrap();
    assert!(!mismatched_apply.status.success());
    let mismatch = String::from_utf8(mismatched_apply.stderr).unwrap();
    assert!(
        mismatch.contains("does not match the last Apply receipt"),
        "{mismatch}"
    );
    assert!(mismatch.contains("source: env"), "{mismatch}");
    assert!(!environment_profile.exists());

    let mismatched_unapply = isolated_command(temp.path())
        .args(["unapply", "--yes"])
        .env("CODEX_HOME", &environment_profile)
        .output()
        .unwrap();
    assert!(!mismatched_unapply.status.success());
    let mismatch = String::from_utf8(mismatched_unapply.stderr).unwrap();
    assert!(
        mismatch.contains("does not match the last Apply receipt"),
        "{mismatch}"
    );
    assert!(mismatch.contains("source: env"), "{mismatch}");
    assert!(flag_profile.join("config.toml").is_file());
    assert!(temp.path().join(".fastctx/config.toml").is_file());

    let removed = isolated_command(temp.path())
        .arg("unapply")
        .arg("--codex-home")
        .arg(&flag_profile)
        .arg("--yes")
        .env("CODEX_HOME", &environment_profile)
        .output()
        .unwrap();
    assert_success(&removed);
    assert!(!flag_profile.exists());
    assert!(!environment_profile.exists());
}

#[test]
fn apply_status_and_unapply_cover_both_shell_states() {
    for fastshell in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        write_shell_settings(temp.path(), fastshell);
        let applied = isolated_command(temp.path())
            .args(["apply", "--guidance", "managed", "--yes"])
            .output()
            .unwrap();
        assert_success(&applied);
        let codex = temp.path().join(".codex");
        let config = std::fs::read_to_string(codex.join("config.toml")).unwrap();
        assert!(config.contains("[mcp_servers.fastctx]"), "{config}");
        let document = config.parse::<toml_edit::DocumentMut>().unwrap();
        let args = document["mcp_servers"]["fastctx"]["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect::<Vec<_>>();
        let mut expected_args = vec!["serve"];
        if fastshell {
            expected_args.push("--enable-shell");
        }
        assert_eq!(args, expected_args, "{config}");
        assert!(!config.contains("mcp__fastctx"), "{config}");
        assert!(!config.contains("[features"), "{config}");
        assert!(!config.contains("approval_mode"), "{config}");

        let agents = std::fs::read_to_string(codex.join("AGENTS.md")).unwrap();
        assert_eq!(agents.matches("<!-- fastctx:begin -->").count(), 1);
        assert_eq!(agents.matches("<!-- fastctx:end -->").count(), 1);
        assert_eq!(agents.contains("### Shell commands"), fastshell);
        assert!(agents.contains("### Batch replacement"), "{agents}");
        assert!(agents.contains("FastCtx's `replace`"), "{agents}");
        for removed in ["copy", "cut", "paste", "clips", "drop"] {
            assert!(!agents.contains(&format!("`{removed}`")));
        }
        let status = isolated_command(temp.path())
            .arg("status")
            .output()
            .unwrap();
        assert_success(&status);
        let status = String::from_utf8_lossy(&status.stdout);
        assert!(status.contains("[PASS] MCP server contract"), "{status}");
        assert!(status.contains("[INFO] Model tool surface"), "{status}");
        assert!(status.contains("Unverified:"), "{status}");
        let prefix = if fastshell { "[PASS]" } else { "[INFO]" };
        assert!(status.contains(&format!("{prefix} fastshell")), "{status}");
        assert!(!status.contains("fastshell bash"), "{status}");
        assert!(!status.contains("fastedit"), "{status}");

        let removed = isolated_command(temp.path())
            .args(["unapply", "--yes"])
            .output()
            .unwrap();
        assert_success(&removed);
        assert!(!temp.path().join(".codex").exists());
        assert!(!temp.path().join(".fastctx").exists());
    }
}

#[test]
fn status_accepts_equivalent_timeout_numbers_and_rejects_real_drift() {
    let temp = tempfile::tempdir().unwrap();
    write_shell_settings(temp.path(), false);
    assert_success(
        &isolated_command(temp.path())
            .args(["apply", "--yes"])
            .output()
            .unwrap(),
    );
    let path = temp.path().join(".codex/config.toml");
    let original = std::fs::read_to_string(&path).unwrap();
    for (startup, tool, drift_key) in [
        ("120.0", "300.0", None),
        ("120", "300", None),
        ("120.5", "300.0", Some("startup_timeout_sec")),
        ("120.0", "300.5", Some("tool_timeout_sec")),
        ("\"120\"", "300.0", Some("startup_timeout_sec")),
        ("nan", "300.0", Some("startup_timeout_sec")),
        ("120.0", "inf", Some("tool_timeout_sec")),
    ] {
        let config = original
            .replace(
                "startup_timeout_sec = 120",
                &format!("startup_timeout_sec = {startup}"),
            )
            .replace(
                "tool_timeout_sec = 300",
                &format!("tool_timeout_sec = {tool}"),
            );
        std::fs::write(&path, config).unwrap();
        let output = isolated_command(temp.path())
            .arg("status")
            .output()
            .unwrap();
        let status = String::from_utf8_lossy(&output.stdout);
        if let Some(key) = drift_key {
            assert!(!output.status.success(), "{startup}/{tool}: {status}");
            assert!(status.contains("[FAIL] Applied state"), "{status}");
            assert!(
                status.contains(&format!("mcp_servers.fastctx.{key}")),
                "{status}"
            );
        } else {
            assert_success(&output);
            assert!(status.contains("[PASS] Applied state"), "{status}");
        }
    }
    std::fs::write(path, original).unwrap();
    assert_success(
        &isolated_command(temp.path())
            .args(["unapply", "--yes"])
            .output()
            .unwrap(),
    );
}

#[test]
fn guidance_can_be_disabled_managed_separately_and_never_removes_pennix_content() {
    let temp = tempfile::tempdir().unwrap();
    let codex = temp.path().join(".codex");
    let applied = isolated_command(temp.path())
        .args(["apply", "--yes"])
        .output()
        .unwrap();
    assert_success(&applied);
    assert!(codex.join("config.toml").is_file());
    assert!(!codex.join("AGENTS.md").exists());

    let guidance_status = isolated_command(temp.path())
        .args(["guidance", "status"])
        .output()
        .unwrap();
    assert_success(&guidance_status);
    assert!(
        String::from_utf8_lossy(&guidance_status.stdout).contains("absent (disabled)"),
        "{}",
        String::from_utf8_lossy(&guidance_status.stdout)
    );

    let foreign = "<!-- fastctx:begin -->\nforeign\n<!-- fastctx:end -->\n";
    std::fs::write(codex.join("AGENTS.md"), foreign).unwrap();
    let refused = isolated_command(temp.path())
        .args(["guidance", "apply", "--yes"])
        .output()
        .unwrap();
    assert!(!refused.status.success());
    assert!(
        String::from_utf8_lossy(&refused.stderr).contains("does not own"),
        "{}",
        String::from_utf8_lossy(&refused.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(codex.join("AGENTS.md")).unwrap(),
        foreign
    );

    std::fs::write(codex.join("AGENTS.md"), "# user rules\n").unwrap();
    let guidance_applied = isolated_command(temp.path())
        .args(["guidance", "apply", "--yes"])
        .output()
        .unwrap();
    assert_success(&guidance_applied);
    let agents = std::fs::read_to_string(codex.join("AGENTS.md")).unwrap();
    assert_eq!(
        agents.matches("<!-- fastctx:begin -->").count(),
        1,
        "{agents}"
    );

    let guidance_removed = isolated_command(temp.path())
        .args(["guidance", "remove", "--yes"])
        .output()
        .unwrap();
    assert_success(&guidance_removed);
    assert_eq!(
        std::fs::read_to_string(codex.join("AGENTS.md")).unwrap(),
        "# user rules\n"
    );

    let pennix = concat!(
        "# user rules\n\n",
        "<!-- pennix-fastctx:begin -->\n",
        "## FastCtx Routing\n\n",
        "- Use `$pennix-fastctx-routing`.\n",
        "<!-- pennix-fastctx:end -->\n"
    );
    std::fs::write(codex.join("AGENTS.md"), pennix).unwrap();
    let removed = isolated_command(temp.path())
        .args(["unapply", "--yes"])
        .output()
        .unwrap();
    assert_success(&removed);
    assert_eq!(
        std::fs::read_to_string(codex.join("AGENTS.md")).unwrap(),
        pennix
    );
    assert!(!temp.path().join(".fastctx").exists());
}

#[test]
fn apply_migrates_owned_three_server_config_and_legacy_agents_blocks_atomically() {
    let temp = tempfile::tempdir().unwrap();
    let codex = temp.path().join(".codex");
    std::fs::create_dir_all(&codex).unwrap();
    write_legacy_extension_settings(temp.path(), true, true);

    let installed = temp.path().join(".fastctx/bin").join(if cfg!(windows) {
        "fastctx.exe"
    } else {
        "fastctx"
    });
    let legacy_read = temp.path().join(".fastread/bin").join(if cfg!(windows) {
        "fastread.exe"
    } else {
        "fastread"
    });
    let config = format!(
        concat!(
            "# user prefix\n",
            "[mcp_servers.fastread]\n",
            "command = '{legacy_read}'\n",
            "startup_timeout_sec = 120\n",
            "[mcp_servers.fastread.env]\n",
            "FASTREAD_TOKEN_BUDGET = '8500'\n\n",
            "[mcp_servers.fastctx]\n",
            "command = '{installed}'\n",
            "startup_timeout_sec = 120\n",
            "[mcp_servers.fastctx.env]\n",
            "FASTCTX_TOKEN_BUDGET = '8500'\n\n",
            "[mcp_servers.fastshell]\n",
            "command = '{installed}'\n",
            "args = ['shell-serve']\n",
            "startup_timeout_sec = 120\n",
            "[mcp_servers.fastshell.env]\n",
            "FASTSHELL_TOKEN_BUDGET = '8500'\n\n",
            "[mcp_servers.fastedit]\n",
            "command = '{installed}'\n",
            "args = ['edit-serve']\n",
            "startup_timeout_sec = 120\n",
            "[mcp_servers.fastedit.env]\n",
            "FASTEDIT_TOKEN_BUDGET = '8500'\n\n",
            "[mcp_servers.user_owned]\n",
            "command = 'keep-me'\n\n",
            "[features.code_mode]\n",
            "direct_only_tool_namespaces = ['user', 'mcp__fastread', 'mcp__fastctx', 'mcp__fastshell', 'mcp__fastedit']\n"
        ),
        legacy_read = normalized(&legacy_read),
        installed = normalized(&installed),
    );
    std::fs::write(codex.join("config.toml"), config).unwrap();
    let agents = concat!(
        "# user rules\n\n",
        "<!-- fastread:begin -->\n",
        "## Local file inspection\n\n",
        "The fastread MCP tools are the first-class way to read, search, and find\n",
        "local files: `mcp__fastread__read`, `mcp__fastread__grep`,\n",
        "`mcp__fastread__glob` — prefer them over `cat`/`Get-Content`,\n",
        "`rg`/`findstr`/`Select-String`, and `dir`/`ls -R`. Pass absolute paths. The\n",
        "last line of every result says `Complete` or `Partial` — continue only with\n",
        "the exact parameters a `Partial` note provides.\n",
        "<!-- fastread:end -->\n\n",
        "<!-- fastctx:begin -->\n### Bulk edits and moving code\nUse mcp__fastctx__copy then mcp__fastctx__paste.\n<!-- fastctx:end -->\n\n",
        "user suffix\n"
    );
    std::fs::write(codex.join("AGENTS.md"), agents).unwrap();

    let output = isolated_command(temp.path())
        .args(["apply", "--guidance", "managed", "--yes"])
        .output()
        .unwrap();
    assert_success(&output);
    let preview = String::from_utf8_lossy(&output.stdout);
    for removed in [
        "- [mcp_servers.fastread]",
        "- [mcp_servers.fastshell]",
        "- [mcp_servers.fastedit]",
        "- direct_only_tool_namespaces -= \"mcp__fastread\"",
        "- direct_only_tool_namespaces -= \"mcp__fastshell\"",
        "- direct_only_tool_namespaces -= \"mcp__fastedit\"",
        "- <!-- fastread:begin --> … <!-- fastread:end -->",
    ] {
        assert!(preview.contains(removed), "missing {removed}:\n{preview}");
    }

    let config = std::fs::read_to_string(codex.join("config.toml")).unwrap();
    assert!(config.contains("# user prefix"), "{config}");
    assert!(config.contains("[mcp_servers.user_owned]"), "{config}");
    assert!(config.contains("command = 'keep-me'"), "{config}");
    assert!(config.contains("[mcp_servers.fastctx]"), "{config}");
    assert!(
        config.contains("args = [\"serve\", \"--enable-shell\"]"),
        "{config}"
    );
    for legacy in [
        "[mcp_servers.fastread]",
        "[mcp_servers.fastshell]",
        "[mcp_servers.fastedit]",
        "mcp__fastread",
        "mcp__fastshell",
        "mcp__fastedit",
    ] {
        assert!(!config.contains(legacy), "{legacy} survived:\n{config}");
    }
    assert_eq!(config.matches("mcp__fastctx").count(), 1, "{config}");

    let agents = std::fs::read_to_string(codex.join("AGENTS.md")).unwrap();
    assert!(agents.starts_with("# user rules\n\n"), "{agents}");
    assert!(agents.ends_with("\nuser suffix\n"), "{agents}");
    assert!(!agents.contains("<!-- fastread:begin -->"), "{agents}");
    assert_eq!(agents.matches("<!-- fastctx:begin -->").count(), 1);
    assert!(agents.contains("### Shell commands"), "{agents}");
    assert!(agents.contains("### Batch replacement"), "{agents}");
    assert!(agents.contains("FastCtx's `replace`"), "{agents}");
    assert!(!agents.contains("mcp__fastctx__copy"), "{agents}");
    assert!(!agents.contains("mcp__fastctx__paste"), "{agents}");
}

#[test]
fn fastshell_preflight_failure_leaves_every_target_byte_untouched() {
    let temp = tempfile::tempdir().unwrap();
    let codex = temp.path().join(".codex");
    std::fs::create_dir_all(&codex).unwrap();
    let config = b"# user config\ntool_output_token_limit = 10000\n";
    let agents = b"# user agents\n";
    std::fs::write(codex.join("config.toml"), config).unwrap();
    std::fs::write(codex.join("AGENTS.md"), agents).unwrap();
    write_shell_settings(temp.path(), true);
    let missing = normalized(&temp.path().join("missing-bash"));
    let output = isolated_command(temp.path())
        .args(["apply", "--yes"])
        .env("FASTCTX_BASH", &missing)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("fastshell is enabled"), "{error}");
    assert!(error.contains("Invalid FASTCTX_BASH"), "{error}");
    assert_eq!(std::fs::read(codex.join("config.toml")).unwrap(), config);
    assert_eq!(std::fs::read(codex.join("AGENTS.md")).unwrap(), agents);
    assert!(!temp.path().join(".fastctx/bin").exists());
}

#[test]
fn a_non_tty_apply_without_yes_refuses_a_shared_limit_conflict_without_writes() {
    let temp = tempfile::tempdir().unwrap();
    let codex = temp.path().join(".codex");
    std::fs::create_dir_all(&codex).unwrap();
    let config = b"tool_output_token_limit = 7000\n";
    std::fs::write(codex.join("config.toml"), config).unwrap();
    let output = isolated_command(temp.path())
        .args(["apply", "--tier", "extra-high"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("Re-run with --yes"), "{error}");
    let preview = String::from_utf8_lossy(&output.stdout);
    assert!(preview.contains("100000"), "{preview}");
    assert_eq!(std::fs::read(codex.join("config.toml")).unwrap(), config);
    assert!(!temp.path().join(".fastctx").exists());
}

#[test]
fn trusted_0223_receipt_removes_only_its_namespace_and_new_receipt_never_owns_it() {
    let temp = profile_test_home();
    assert_success(
        &isolated_command(temp.path())
            .args(["apply", "--yes"])
            .output()
            .unwrap(),
    );
    let config_path = temp.path().join(".codex/config.toml");
    let receipt_path = temp.path().join(".fastctx/config.toml");
    let config = std::fs::read_to_string(&config_path).unwrap();
    let host_policy = concat!(
        "\n# user host policy\n[features.code_mode]\n",
        "direct_only_tool_namespaces = [\n 'alpha',\n # retained comment\n 'mcp__fastctx',\n 'omega',\n] # retained tail\n",
    );
    std::fs::write(&config_path, format!("{config}{host_policy}")).unwrap();
    let mut receipt: toml_edit::DocumentMut = std::fs::read_to_string(&receipt_path)
        .unwrap()
        .parse()
        .unwrap();
    receipt["applied"]["version"] = toml_edit::value("0.2.23");
    std::fs::write(&receipt_path, receipt.to_string()).unwrap();
    let upgraded = isolated_command(temp.path())
        .args(["apply", "--yes"])
        .output()
        .unwrap();
    assert_success(&upgraded);
    let config = std::fs::read_to_string(&config_path).unwrap();
    assert!(!config.contains("mcp__fastctx"), "{config}");
    for preserved in [
        "'alpha'",
        "'omega'",
        "# retained comment",
        "# retained tail",
        "# user host policy",
    ] {
        assert!(config.contains(preserved), "{config}");
    }
    assert!(
        String::from_utf8_lossy(&upgraded.stdout)
            .contains("direct_only_tool_namespaces -= \"mcp__fastctx\"")
    );
    let user_policy = config.replace("'alpha'", "'mcp__fastctx'");
    std::fs::write(&config_path, &user_policy).unwrap();
    assert_success(
        &isolated_command(temp.path())
            .args(["apply", "--yes"])
            .output()
            .unwrap(),
    );
    assert_eq!(std::fs::read_to_string(&config_path).unwrap(), user_policy);
    let removed = isolated_command(temp.path())
        .args(["unapply", "--yes"])
        .output()
        .unwrap();
    assert_success(&removed);
    assert!(
        std::fs::read_to_string(&config_path)
            .unwrap()
            .contains("'mcp__fastctx'")
    );
    assert!(!String::from_utf8_lossy(&removed.stdout).contains("direct_only_tool_namespaces -="));
}

#[test]
fn conflicting_0223_receipt_blocks_namespace_migration_without_any_write() {
    let temp = profile_test_home();
    assert_success(
        &isolated_command(temp.path())
            .args(["apply", "--yes"])
            .output()
            .unwrap(),
    );
    let config_path = temp.path().join(".codex/config.toml");
    let receipt_path = temp.path().join(".fastctx/config.toml");
    let config = std::fs::read_to_string(&config_path).unwrap();
    let config = format!(
        "{config}\n[features.code_mode]\ndirect_only_tool_namespaces = ['mcp__fastctx', 'mcp__fastctx']\n"
    );
    std::fs::write(&config_path, &config).unwrap();
    let mut receipt: toml_edit::DocumentMut = std::fs::read_to_string(&receipt_path)
        .unwrap()
        .parse()
        .unwrap();
    receipt["applied"]["version"] = toml_edit::value("0.2.23");
    std::fs::write(&receipt_path, receipt.to_string()).unwrap();
    let receipt = std::fs::read(&receipt_path).unwrap();
    for action in ["apply", "unapply"] {
        let result = isolated_command(temp.path())
            .args([action, "--yes"])
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(
            String::from_utf8_lossy(&result.stderr).contains("namespace ownership"),
            "{result:?}"
        );
        assert_eq!(std::fs::read_to_string(&config_path).unwrap(), config);
        assert_eq!(std::fs::read(&receipt_path).unwrap(), receipt);
    }
}

#[cfg(unix)]
#[test]
fn an_unsafe_legacy_receipt_cannot_authorize_namespace_removal() {
    use std::os::unix::fs::PermissionsExt;
    let temp = profile_test_home();
    assert_success(
        &isolated_command(temp.path())
            .args(["apply", "--yes"])
            .output()
            .unwrap(),
    );
    let config_path = temp.path().join(".codex/config.toml");
    let receipt_path = temp.path().join(".fastctx/config.toml");
    let config = format!(
        "{}\n[features.code_mode]\ndirect_only_tool_namespaces = ['mcp__fastctx']\n",
        std::fs::read_to_string(&config_path).unwrap()
    );
    std::fs::write(&config_path, &config).unwrap();
    let mut receipt: toml_edit::DocumentMut = std::fs::read_to_string(&receipt_path)
        .unwrap()
        .parse()
        .unwrap();
    receipt["applied"]["version"] = toml_edit::value("0.2.23");
    std::fs::write(&receipt_path, receipt.to_string()).unwrap();
    std::fs::set_permissions(&receipt_path, std::fs::Permissions::from_mode(0o666)).unwrap();
    let before = std::fs::read(&receipt_path).unwrap();
    for action in ["apply", "unapply"] {
        let result = isolated_command(temp.path())
            .args([action, "--yes"])
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(
            String::from_utf8_lossy(&result.stderr).contains("Unsafe legacy ownership receipt")
        );
        assert_eq!(std::fs::read_to_string(&config_path).unwrap(), config);
        assert_eq!(std::fs::read(&receipt_path).unwrap(), before);
    }
}

#[test]
fn an_unknown_receipt_does_not_claim_a_user_namespace() {
    let temp = profile_test_home();
    assert_success(
        &isolated_command(temp.path())
            .args(["apply", "--yes"])
            .output()
            .unwrap(),
    );
    let config_path = temp.path().join(".codex/config.toml");
    let receipt_path = temp.path().join(".fastctx/config.toml");
    let config = format!(
        "{}\n[features.code_mode]\ndirect_only_tool_namespaces = ['mcp__fastctx', 'user'] # retained\n",
        std::fs::read_to_string(&config_path).unwrap()
    );
    std::fs::write(&config_path, &config).unwrap();
    let mut receipt: toml_edit::DocumentMut = std::fs::read_to_string(&receipt_path)
        .unwrap()
        .parse()
        .unwrap();
    receipt["applied"]["version"] = toml_edit::value("0.2.22");
    std::fs::write(&receipt_path, receipt.to_string()).unwrap();
    assert_success(
        &isolated_command(temp.path())
            .args(["apply", "--yes"])
            .output()
            .unwrap(),
    );
    assert_eq!(std::fs::read_to_string(&config_path).unwrap(), config);
    assert_success(
        &isolated_command(temp.path())
            .args(["unapply", "--yes"])
            .output()
            .unwrap(),
    );
    let removed = std::fs::read_to_string(&config_path).unwrap();
    assert!(removed.contains("['mcp__fastctx', 'user'] # retained"));
}

#[test]
fn stamped_v4_guidance_requires_explicit_acceptance_and_preserves_user_content() {
    use sha2::Digest;
    let temp = profile_test_home();
    assert_success(
        &isolated_command(temp.path())
            .args(["apply", "--guidance", "managed", "--yes"])
            .output()
            .unwrap(),
    );
    let agents_path = temp.path().join(".codex/AGENTS.md");
    let config_path = temp.path().join(".codex/config.toml");
    let receipt_path = temp.path().join(".fastctx/config.toml");
    let v5 = concat!(
        "Route local files to FastCtx (URI refs as absolute paths), directly or via\n",
        "host-native orchestration. Preserve results and Complete/Partial details;\n",
        "recover truncation with bounded direct calls."
    );
    let v4 = fastctx::control::agents::section(false).replace(v5,
        "Use FastCtx file tools directly for local-file operations, including when a\nlocal reference is URI-shaped; pass the equivalent plain absolute filesystem path.");
    assert!(!v4.contains("native tool orchestration"));
    let old = format!("# user prefix\n\n{v4}\nuser suffix\n");
    std::fs::write(&agents_path, &old).unwrap();
    let mut receipt: toml_edit::DocumentMut = std::fs::read_to_string(&receipt_path)
        .unwrap()
        .parse()
        .unwrap();
    receipt["applied"]["agents_contract_id"] = toml_edit::value("guidance-v4");
    receipt["applied"]["codex_agents"]["applied_sha256"] = toml_edit::value("00".repeat(32));
    std::fs::write(&receipt_path, receipt.to_string()).unwrap();
    let rejected = isolated_command(temp.path())
        .args(["guidance", "apply", "--yes"])
        .output()
        .unwrap();
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("guidance-v4"));
    assert_eq!(std::fs::read_to_string(&agents_path).unwrap(), old);
    assert_success(
        &isolated_command(temp.path())
            .args(["apply", "--yes"])
            .output()
            .unwrap(),
    );
    assert_eq!(std::fs::read_to_string(&agents_path).unwrap(), old);
    let status = isolated_command(temp.path())
        .args(["guidance", "status"])
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&status.stdout).contains("superseded guidance-v4"),
        "{status:?}"
    );

    let mut receipt: toml_edit::DocumentMut = std::fs::read_to_string(&receipt_path)
        .unwrap()
        .parse()
        .unwrap();
    receipt["applied"]["codex_agents"]["applied_sha256"] =
        toml_edit::value(hex::encode(sha2::Sha256::digest(old.as_bytes())));
    std::fs::write(&receipt_path, receipt.to_string()).unwrap();
    assert_success(
        &isolated_command(temp.path())
            .args(["guidance", "apply", "--yes"])
            .output()
            .unwrap(),
    );
    let current = std::fs::read_to_string(&agents_path).unwrap();
    assert!(current.starts_with("# user prefix\n\n"));
    assert!(current.ends_with("user suffix\n"));
    assert!(current.contains(v5));
    let old_with_user = format!("{old}additional user rule\n");
    let current = std::fs::read_to_string(&agents_path).unwrap();
    std::fs::write(&agents_path, format!("{current}additional user rule\n")).unwrap();
    let config = std::fs::read(&config_path).unwrap();
    assert_success(
        &isolated_command(temp.path())
            .args(["guidance", "apply", "--yes"])
            .output()
            .unwrap(),
    );
    let current = std::fs::read_to_string(&agents_path).unwrap();
    assert!(current.starts_with("# user prefix\n\n"));
    assert!(current.ends_with("user suffix\nadditional user rule\n"));
    assert!(current.contains(v5));
    assert_eq!(std::fs::read(&config_path).unwrap(), config);
    assert!(
        std::fs::read_to_string(&receipt_path)
            .unwrap()
            .contains("guidance-v5")
    );

    std::fs::write(&agents_path, &old_with_user).unwrap();
    let receipt = std::fs::read(&receipt_path).unwrap();
    for args in [
        vec!["guidance", "apply", "--yes"],
        vec!["apply", "--guidance", "managed", "--yes"],
    ] {
        let rejected = isolated_command(temp.path()).args(args).output().unwrap();
        assert!(!rejected.status.success());
        assert!(String::from_utf8_lossy(&rejected.stderr).contains("guidance-v4"));
        assert_eq!(
            std::fs::read_to_string(&agents_path).unwrap(),
            old_with_user
        );
        assert_eq!(std::fs::read(&receipt_path).unwrap(), receipt);
    }
}

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_fastctx"))
}

fn isolated_command(home: &Path) -> Command {
    let mut command = command();
    command
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env_remove("CODEX_HOME")
        .env("TMPDIR", home)
        .env("TMP", home)
        .env("TEMP", home);
    command
}

fn profile_test_home() -> tempfile::TempDir {
    // Canonicalize the base so the temp path's Windows drive-letter case is stable regardless
    // of the shell cwd: the binary echoes back the path it receives while normalized()
    // canonicalizes for comparison, so both sides must agree on drive case (2026-07-22).
    let base = dunce::canonicalize(std::env::current_dir().unwrap()).unwrap();
    tempfile::Builder::new()
        .prefix("fastctx-codex-home-")
        .tempdir_in(base)
        .unwrap()
}

fn start_persistent_job(home: &Path, command: &str) -> String {
    let mut server = isolated_command(home);
    server.args(["serve", "--enable-shell"]);
    let mut session = McpSession::start(server);
    let started = session.call(
        "run_background",
        serde_json::json!({
            "command": command,
            "cwd": normalized(home),
            "login_shell": false
        }),
    );
    let body = mcp_text(&started)
        .strip_prefix("(Complete: job ")
        .and_then(|value| value.strip_suffix(".)"))
        .expect("run_background must return its stable start terminal");
    let (job_id, log_path) = body
        .split_once(" started; log at ")
        .expect("run_background must return its stable job id and log path");
    assert!(Path::new(log_path).is_absolute(), "{log_path}");
    assert!(session.close().success());
    job_id.to_string()
}

struct BackgroundJobCleanup {
    home: PathBuf,
    job_id: Option<String>,
}

impl BackgroundJobCleanup {
    fn new(home: &Path, job_id: &str) -> Self {
        Self {
            home: home.to_path_buf(),
            job_id: Some(job_id.to_string()),
        }
    }

    fn disarm(&mut self) {
        self.job_id = None;
    }
}

impl Drop for BackgroundJobCleanup {
    fn drop(&mut self) {
        if let Some(job_id) = self.job_id.take() {
            let _ = isolated_command(&self.home)
                .args(["jobs", "kill", &job_id])
                .output();
        }
    }
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn write_shell_settings(home: &Path, fastshell: bool) {
    let directory = home.join(".fastctx");
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(
        directory.join("config.toml"),
        format!("schema_version = 1\n\n[fastshell]\nenabled = {fastshell}\n"),
    )
    .unwrap();
}

fn write_legacy_extension_settings(home: &Path, fastshell: bool, fastedit: bool) {
    let directory = home.join(".fastctx");
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(
        directory.join("config.toml"),
        format!(
            "schema_version = 1\n\n[fastshell]\nenabled = {fastshell}\n\n[fastedit]\nenabled = {fastedit}\n"
        ),
    )
    .unwrap();
}
