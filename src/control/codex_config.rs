//! Ownership-aware Codex TOML editing with `toml_edit` preserving all unowned content.

use crate::control::settings::{Tier, ToolBudgets};
use std::ops::Range;
use std::str::FromStr;
use toml_edit::{Array, DocumentMut, Item, Table, Value, value};

const FASTCTX_NAMESPACE: &str = "mcp__fastctx";
const LEGACY_FASTREAD_NAMESPACE: &str = "mcp__fastread";
const LEGACY_FASTSHELL_NAMESPACE: &str = "mcp__fastshell";
const LEGACY_FASTEDIT_NAMESPACE: &str = "mcp__fastedit";
const STARTUP_TIMEOUT_SECONDS: i64 = 120;
/// MCP tool timeout written by Apply so 240-second tool waits retain a 60-second return margin.
pub(crate) const TOOL_TIMEOUT_SECONDS: i64 = 300;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct LegacyRemoval {
    fastread: bool,
    fastshell: bool,
    fastedit: bool,
}

/// Expected Codex configuration after Apply.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedConfig {
    /// Stable absolute command path.
    pub command: String,
    /// Host output tier.
    pub tier: Tier,
    /// Effective host output limit; may be Guarded independently from the selected tier.
    pub host_limit: i64,
    /// Effective global FastCtx budget; may be Guarded independently from the selected tier.
    pub fastctx_budget: usize,
    /// Five long-output tools' relative budgets.
    pub tool_budgets: ToolBudgets,
    /// Whether Apply should publish the optional shell tool group.
    pub fastshell_enabled: bool,
}

/// Conflict on a shared key in an Apply plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenLimitConflict {
    /// Current user setting.
    pub current: i64,
    /// Setting required by the FastCtx tier.
    pub requested: i64,
}

/// Immutable Codex-config edit result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplyEdit {
    /// Complete new file bytes.
    pub bytes: Vec<u8>,
    /// Whether the shared token key originally existed.
    pub previous_token_limit_present: bool,
    /// Previous value of the shared token key.
    pub previous_token_limit: Option<i64>,
    /// A present and differing shared key requires additional confirmation.
    pub conflict: Option<TokenLimitConflict>,
}

/// Parses Codex TOML and produces the post-Apply bytes.
pub fn apply(original: &[u8], expected: &ExpectedConfig) -> Result<ApplyEdit, String> {
    let (migrated, legacy) = strip_owned_legacy_servers(original, expected)?;
    let mut document = parse(&migrated)?;
    let requested_limit = expected.host_limit;
    let existing_limit = document.get("tool_output_token_limit");
    let previous_token_limit_present = existing_limit.is_some();
    let previous_token_limit = existing_limit
        .map(|item| {
            item.as_integer().ok_or_else(|| {
                "Codex config key tool_output_token_limit is not an integer. Repair it manually and retry."
                    .to_string()
            })
        })
        .transpose()?;
    let conflict = previous_token_limit
        .filter(|current| *current != requested_limit)
        .map(|current| TokenLimitConflict {
            current,
            requested: requested_limit,
        });

    let mcp_servers = ensure_table(&mut document, "mcp_servers")?;
    let mut fastctx_table = build_fastctx_table(expected);
    if let Some(existing) = mcp_servers.get("fastctx").and_then(Item::as_table) {
        *fastctx_table.decor_mut() = existing.decor().clone();
    }
    mcp_servers.insert("fastctx", Item::Table(fastctx_table));

    if (legacy.fastread || legacy.fastshell || legacy.fastedit)
        && let Some(array) = namespace_array_mut(&mut document)?
    {
        if legacy.fastread {
            remove_namespace(array, LEGACY_FASTREAD_NAMESPACE);
        }
        if legacy.fastshell {
            remove_namespace(array, LEGACY_FASTSHELL_NAMESPACE);
        }
        if legacy.fastedit {
            remove_namespace(array, LEGACY_FASTEDIT_NAMESPACE);
        }
    }
    set_integer(&mut document, "tool_output_token_limit", requested_limit)?;

    Ok(ApplyEdit {
        bytes: document.to_string().into_bytes(),
        previous_token_limit_present,
        previous_token_limit,
        conflict,
    })
}

/// Removes FastCtx-owned configuration in reverse; the shared token key is restored only when explicitly allowed.
pub fn unapply(
    original: &[u8],
    restore_token_limit: bool,
    previous_token_limit_present: bool,
    previous_token_limit: Option<i64>,
) -> Result<Vec<u8>, String> {
    let mut document = parse(original)?;
    if document.get("mcp_servers").is_some() {
        let emptied = {
            let mcp_servers = document
                .get_mut("mcp_servers")
                .and_then(Item::as_table_mut)
                .ok_or_else(|| {
                    "Codex config key mcp_servers is not a table. Repair it manually and retry."
                        .to_string()
                })?;
            mcp_servers.remove("fastctx");
            mcp_servers.is_empty()
        };
        // Remove an mcp_servers table created solely by Apply so no empty shell remains;
        // this also keeps drift-free reverse removal byte-exact with Apply (2026-07-12).
        if emptied {
            document.remove("mcp_servers");
        }
    }

    if restore_token_limit {
        if previous_token_limit_present {
            let previous = previous_token_limit.ok_or_else(|| {
                "The Apply receipt says tool_output_token_limit existed but does not contain its previous value. Re-apply before restoring it."
                    .to_string()
            })?;
            set_integer(&mut document, "tool_output_token_limit", previous)?;
        } else {
            document.remove("tool_output_token_limit");
        }
    }
    Ok(document.to_string().into_bytes())
}

/// Reads the current integer tool_output_token_limit, returning None when absent, non-integer, or unparseable.
/// Unapply uses this for ownership: restore the shared key only while it still equals the value written by Apply.
pub fn current_token_limit(original: &[u8]) -> Option<i64> {
    parse(original)
        .ok()?
        .get("tool_output_token_limit")
        .and_then(Item::as_integer)
}

/// Returns whether a managed server table exists in a valid Codex config.
pub fn has_server(original: &[u8], name: &str) -> bool {
    parse(original)
        .ok()
        .and_then(|document| {
            document
                .get("mcp_servers")
                .and_then(Item::as_table_like)
                .and_then(|table| table.get(name))
                .map(|_| ())
        })
        .is_some()
}

/// Returns whether the direct-only namespace array contains an entry.
pub fn has_namespace(original: &[u8], namespace: &str) -> bool {
    namespace_count(original, namespace) > 0
}

pub(crate) fn namespace_count(original: &[u8], namespace: &str) -> usize {
    parse(original)
        .ok()
        .and_then(|document| {
            document
                .get("features")
                .and_then(Item::as_table_like)
                .and_then(|table| table.get("code_mode"))
                .and_then(Item::as_table_like)
                .and_then(|table| table.get("direct_only_tool_namespaces"))
                .and_then(Item::as_array)
                .map(|array| {
                    array
                        .iter()
                        .filter(|entry| entry.as_str() == Some(namespace))
                        .count()
                })
        })
        .unwrap_or(0)
}

/// Called only after the control layer proves ownership from the 0.2.23 receipt.
pub(crate) fn remove_owned_direct_only_namespace(original: &[u8]) -> Result<Vec<u8>, String> {
    let mut document = parse(original)?;
    if let Some(array) = namespace_array_mut(&mut document)? {
        remove_namespace(array, FASTCTX_NAMESPACE);
    }
    Ok(document.to_string().into_bytes())
}

/// Checks each Codex configuration item against the Apply receipt.
pub fn drift(original: &[u8], expected: &ExpectedConfig) -> Result<Vec<String>, String> {
    drift_with_limits(
        original,
        expected,
        expected.host_limit,
        expected.fastctx_budget,
        Some(TOOL_TIMEOUT_SECONDS),
    )
}

/// Checks managed configuration against the exact numeric values recorded by an Apply receipt.
/// Older receipts omit `tool_timeout_sec`; that missing ownership evidence must not create false drift.
pub fn drift_applied(
    original: &[u8],
    expected: &ExpectedConfig,
    host_limit: i64,
    fastctx_budget: usize,
    tool_timeout_sec: Option<i64>,
) -> Result<Vec<String>, String> {
    drift_with_limits(
        original,
        expected,
        host_limit,
        fastctx_budget,
        tool_timeout_sec,
    )
}

fn timeout_matches(item: Option<&Item>, expected: i64) -> bool {
    // Codex may serialize the same managed timeout as a TOML float.
    item.is_some_and(|item| {
        item.as_integer() == Some(expected)
            || item
                .as_float()
                .is_some_and(|value| value == expected as f64)
    })
}

fn drift_with_limits(
    original: &[u8],
    expected: &ExpectedConfig,
    host_limit: i64,
    fastctx_budget: usize,
    tool_timeout_sec: Option<i64>,
) -> Result<Vec<String>, String> {
    let document = parse(original)?;
    let mut drift = Vec::new();
    let fastctx = document
        .get("mcp_servers")
        .and_then(Item::as_table_like)
        .and_then(|table| table.get("fastctx"))
        .and_then(Item::as_table_like);
    match fastctx {
        Some(table) => {
            if table.get("command").and_then(Item::as_str) != Some(expected.command.as_str()) {
                drift.push("mcp_servers.fastctx.command".to_string());
            }
            if !timeout_matches(table.get("startup_timeout_sec"), STARTUP_TIMEOUT_SECONDS) {
                drift.push("mcp_servers.fastctx.startup_timeout_sec".to_string());
            }
            if let Some(tool_timeout_sec) = tool_timeout_sec
                && !timeout_matches(table.get("tool_timeout_sec"), tool_timeout_sec)
            {
                drift.push("mcp_servers.fastctx.tool_timeout_sec".to_string());
            }
            let env = table
                .get("env")
                .and_then(Item::as_table_like)
                .ok_or_else(|| "mcp_servers.fastctx.env is missing or not a table".to_string());
            match env {
                Ok(env) => check_env(env, expected, fastctx_budget, &mut drift),
                Err(_) => drift.push("mcp_servers.fastctx.env".to_string()),
            }
            let actual_args = table.get("args").and_then(Item::as_array);
            let expected_args = server_args(expected);
            let args_match = actual_args.is_some_and(|args| {
                args.len() == expected_args.len()
                    && args
                        .iter()
                        .zip(expected_args.iter())
                        .all(|(actual, expected)| actual.as_str() == Some(*expected))
            });
            if !args_match {
                drift.push("mcp_servers.fastctx.args".to_string());
            }
        }
        None => drift.push("mcp_servers.fastctx".to_string()),
    }
    if document
        .get("tool_output_token_limit")
        .and_then(Item::as_integer)
        != Some(host_limit)
    {
        drift.push("tool_output_token_limit".to_string());
    }
    Ok(drift)
}

fn parse(original: &[u8]) -> Result<DocumentMut, String> {
    let source = std::str::from_utf8(original).map_err(|error| {
        format!("Codex config is not valid UTF-8 ({error}). Repair the file manually and retry.")
    })?;
    DocumentMut::from_str(source).map_err(|error| {
        format!("Cannot parse Codex config.toml: {error}. Repair it manually and retry.")
    })
}

fn set_integer(document: &mut DocumentMut, key: &str, integer: i64) -> Result<(), String> {
    match document.get_mut(key) {
        Some(item) => {
            let existing = item.as_value_mut().ok_or_else(|| {
                format!("Codex config key {key} is not an integer. Repair it manually and retry.")
            })?;
            if existing.as_integer().is_none() {
                return Err(format!(
                    "Codex config key {key} is not an integer. Repair it manually and retry."
                ));
            }
            let decor = existing.decor().clone();
            let mut replacement = Value::from(integer);
            *replacement.decor_mut() = decor;
            *existing = replacement;
        }
        None => document[key] = value(integer),
    }
    Ok(())
}

fn remove_namespace(array: &mut Array, namespace: &str) {
    let matching = array
        .iter()
        .enumerate()
        .filter_map(|(index, value)| (value.as_str() == Some(namespace)).then_some(index))
        .collect::<Vec<_>>();
    for index in matching.into_iter().rev() {
        remove_array_index_preserving_trailing(array, index);
    }
}

fn remove_array_index_preserving_trailing(array: &mut Array, index: usize) {
    let removed = array.remove(index);
    let decor = removed.decor();
    let comments = format!(
        "{}{}",
        decor.prefix().and_then(|s| s.as_str()).unwrap_or_default(),
        decor.suffix().and_then(|s| s.as_str()).unwrap_or_default()
    );
    if let Some(next) = array.get_mut(index) {
        let prefix = next
            .decor()
            .prefix()
            .and_then(|s| s.as_str())
            .unwrap_or_default();
        let prefix = format!("{comments}{prefix}");
        next.decor_mut().set_prefix(prefix);
    } else {
        let trailing = array.trailing().as_str().unwrap_or_default();
        array.set_trailing(format!("{comments}{trailing}"));
    }
}

fn ensure_table<'a>(document: &'a mut DocumentMut, key: &str) -> Result<&'a mut Table, String> {
    if document.get(key).is_none() {
        document[key] = Item::Table(Table::new());
    }
    document
        .get_mut(key)
        .and_then(Item::as_table_mut)
        .ok_or_else(|| {
            format!("Codex config key {key} is not a table. Repair it manually and retry.")
        })
}

fn namespace_array_mut(document: &mut DocumentMut) -> Result<Option<&mut Array>, String> {
    let item = document
        .get_mut("features")
        .and_then(Item::as_table_like_mut)
        .and_then(|table| table.get_mut("code_mode"))
        .and_then(Item::as_table_like_mut)
        .and_then(|table| table.get_mut("direct_only_tool_namespaces"));
    item.map(|item| item.as_array_mut().ok_or_else(|| "Codex config key features.code_mode.direct_only_tool_namespaces is not an array. Repair it manually and retry.".to_string())).transpose()
}

fn build_fastctx_table(expected: &ExpectedConfig) -> Table {
    let global = expected.fastctx_budget;
    let mut table = Table::new();
    table.insert("command", value(expected.command.clone()));
    let mut args = Array::new();
    for argument in server_args(expected) {
        args.push(argument);
    }
    table.insert("args", Item::Value(Value::Array(args)));
    table.insert("startup_timeout_sec", value(STARTUP_TIMEOUT_SECONDS));
    table.insert("tool_timeout_sec", value(TOOL_TIMEOUT_SECONDS));
    let mut env = Table::new();
    env.insert("FASTCTX_TOKEN_BUDGET", value(global.to_string()));
    insert_tool_budget(
        &mut env,
        "FASTCTX_READ_TOKEN_BUDGET",
        expected.tool_budgets.read.resolve(global),
    );
    insert_tool_budget(
        &mut env,
        "FASTCTX_GREP_TOKEN_BUDGET",
        expected.tool_budgets.grep.resolve(global),
    );
    insert_tool_budget(
        &mut env,
        "FASTCTX_GLOB_TOKEN_BUDGET",
        expected.tool_budgets.glob.resolve(global),
    );
    insert_tool_budget(
        &mut env,
        "FASTCTX_RUN_TOKEN_BUDGET",
        expected.tool_budgets.run.resolve(global),
    );
    insert_tool_budget(
        &mut env,
        "FASTCTX_JOB_OUTPUT_TOKEN_BUDGET",
        expected.tool_budgets.job_output.resolve(global),
    );
    table.insert("env", Item::Table(env));
    table
}

fn owned_legacy_servers(mcp_servers: &Table, expected: &ExpectedConfig) -> LegacyRemoval {
    let fastread = mcp_servers
        .get("fastread")
        .is_some_and(legacy_fastread_table_is_owned);
    let fastshell = mcp_servers.get("fastshell").is_some_and(|item| {
        legacy_optional_table_is_owned(item, expected, "shell-serve", "FASTSHELL_TOKEN_BUDGET")
    });
    let fastedit = mcp_servers.get("fastedit").is_some_and(|item| {
        legacy_optional_table_is_owned(item, expected, "edit-serve", "FASTEDIT_TOKEN_BUDGET")
    });
    LegacyRemoval {
        fastread,
        fastshell,
        fastedit,
    }
}

fn strip_owned_legacy_servers(
    original: &[u8],
    expected: &ExpectedConfig,
) -> Result<(Vec<u8>, LegacyRemoval), String> {
    let source = std::str::from_utf8(original).map_err(|error| {
        format!("Codex config is not valid UTF-8 ({error}). Repair the file manually and retry.")
    })?;
    let document = toml_edit::ImDocument::parse(source).map_err(|error| {
        format!("Cannot parse Codex config.toml: {error}. Repair it manually and retry.")
    })?;
    let Some(mcp_servers) = document.get("mcp_servers").and_then(Item::as_table) else {
        return Ok((original.to_vec(), LegacyRemoval::default()));
    };
    let legacy = owned_legacy_servers(mcp_servers, expected);
    let mut spans = Vec::new();
    for (name, owned) in [
        ("fastread", legacy.fastread),
        ("fastshell", legacy.fastshell),
        ("fastedit", legacy.fastedit),
    ] {
        if owned {
            let item = mcp_servers
                .get(name)
                .expect("an owned legacy table must still exist");
            collect_explicit_table_spans(item, name, &mut spans)?;
        }
    }
    if spans.is_empty() {
        return Ok((original.to_vec(), legacy));
    }
    spans.sort_by_key(|span| span.start);
    let mut merged: Vec<Range<usize>> = Vec::with_capacity(spans.len());
    for span in spans {
        if let Some(previous) = merged.last_mut()
            && span.start <= previous.end
        {
            previous.end = previous.end.max(span.end);
        } else {
            merged.push(span);
        }
    }
    let mut output = Vec::with_capacity(original.len());
    let mut cursor = 0;
    for span in merged {
        output.extend_from_slice(&original[cursor..span.start]);
        cursor = span.end;
    }
    output.extend_from_slice(&original[cursor..]);
    Ok((output, legacy))
}

fn collect_explicit_table_spans(
    item: &Item,
    name: &str,
    spans: &mut Vec<Range<usize>>,
) -> Result<(), String> {
    let table = item.as_table().ok_or_else(|| {
        format!("Cannot safely migrate mcp_servers.{name}: the owned entry is not a table.")
    })?;
    if !table.is_implicit() {
        spans.push(table.span().ok_or_else(|| {
            format!(
                "Cannot safely migrate mcp_servers.{name}: its source range is unavailable. Repair or remove the legacy table manually and retry."
            )
        })?);
    }
    for (_, child) in table.iter() {
        if child.is_table() {
            collect_explicit_table_spans(child, name, spans)?;
        }
    }
    Ok(())
}

fn legacy_optional_table_is_owned(
    item: &Item,
    expected: &ExpectedConfig,
    argument: &str,
    budget_key: &str,
) -> bool {
    let Some(table) = item.as_table_like() else {
        return false;
    };
    if !has_exact_keys(table, &["command", "args", "startup_timeout_sec", "env"])
        || table.get("command").and_then(Item::as_str) != Some(expected.command.as_str())
        || table.get("startup_timeout_sec").and_then(Item::as_integer)
            != Some(STARTUP_TIMEOUT_SECONDS)
    {
        return false;
    }
    let args_match = table
        .get("args")
        .and_then(Item::as_array)
        .is_some_and(|args| {
            args.len() == 1 && args.get(0).and_then(Value::as_str) == Some(argument)
        });
    let env_match = table
        .get("env")
        .and_then(Item::as_table_like)
        .is_some_and(|env| {
            has_exact_keys(env, &[budget_key])
                && positive_integer_string(env.get(budget_key).and_then(Item::as_str))
        });
    args_match && env_match
}

fn legacy_fastread_table_is_owned(item: &Item) -> bool {
    let Some(table) = item.as_table_like() else {
        return false;
    };
    if !has_only_keys(table, &["command", "startup_timeout_sec", "enabled", "env"])
        || !["command", "startup_timeout_sec", "env"]
            .iter()
            .all(|key| table.get(key).is_some())
        || table.get("startup_timeout_sec").and_then(Item::as_integer)
            != Some(STARTUP_TIMEOUT_SECONDS)
        || table
            .get("enabled")
            .is_some_and(|enabled| enabled.as_bool() != Some(false))
    {
        return false;
    }
    let command = table
        .get("command")
        .and_then(Item::as_str)
        .map(|command| command.replace('\\', "/").to_ascii_lowercase());
    let command_matches = command.is_some_and(|command| {
        command.ends_with("/.fastread/bin/fastread")
            || command.ends_with("/.fastread/bin/fastread.exe")
    });
    let env_matches = table
        .get("env")
        .and_then(Item::as_table_like)
        .is_some_and(|env| {
            env.get("FASTREAD_TOKEN_BUDGET").is_some()
                && env.iter().all(|(key, value)| {
                    matches!(
                        key,
                        "FASTREAD_TOKEN_BUDGET"
                            | "FASTREAD_READ_TOKEN_BUDGET"
                            | "FASTREAD_GREP_TOKEN_BUDGET"
                            | "FASTREAD_GLOB_TOKEN_BUDGET"
                    ) && positive_integer_string(value.as_str())
                })
        });
    command_matches && env_matches
}

fn has_exact_keys(table: &dyn toml_edit::TableLike, allowed: &[&str]) -> bool {
    table.len() == allowed.len() && table.iter().all(|(key, _)| allowed.contains(&key))
}

fn has_only_keys(table: &dyn toml_edit::TableLike, allowed: &[&str]) -> bool {
    table.iter().all(|(key, _)| allowed.contains(&key))
}

fn positive_integer_string(value: Option<&str>) -> bool {
    value
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|value| value > 0)
}

fn server_args(expected: &ExpectedConfig) -> Vec<&'static str> {
    let mut args = vec!["serve"];
    if expected.fastshell_enabled {
        args.push("--enable-shell");
    }
    args
}

fn insert_tool_budget(table: &mut Table, key: &str, budget: Option<usize>) {
    if let Some(budget) = budget {
        table.insert(key, value(budget.to_string()));
    }
}

fn check_env(
    env: &dyn toml_edit::TableLike,
    expected: &ExpectedConfig,
    global: usize,
    drift: &mut Vec<String>,
) {
    check_env_value(env, "FASTCTX_TOKEN_BUDGET", Some(global), drift);
    check_env_value(
        env,
        "FASTCTX_READ_TOKEN_BUDGET",
        expected.tool_budgets.read.resolve(global),
        drift,
    );
    check_env_value(
        env,
        "FASTCTX_GREP_TOKEN_BUDGET",
        expected.tool_budgets.grep.resolve(global),
        drift,
    );
    check_env_value(
        env,
        "FASTCTX_GLOB_TOKEN_BUDGET",
        expected.tool_budgets.glob.resolve(global),
        drift,
    );
    check_env_value(
        env,
        "FASTCTX_RUN_TOKEN_BUDGET",
        expected.tool_budgets.run.resolve(global),
        drift,
    );
    check_env_value(
        env,
        "FASTCTX_JOB_OUTPUT_TOKEN_BUDGET",
        expected.tool_budgets.job_output.resolve(global),
        drift,
    );
}

fn check_env_value(
    env: &dyn toml_edit::TableLike,
    key: &str,
    expected: Option<usize>,
    drift: &mut Vec<String>,
) {
    let actual = env.get(key).and_then(Item::as_str);
    let expected_string = expected.map(|value| value.to_string());
    if actual != expected_string.as_deref() {
        drift.push(format!("mcp_servers.fastctx.env.{key}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removing_the_owned_element_retains_comments_at_every_array_position() {
        for entries in [
            "\n # 注释\n 'mcp__fastctx' # suffix\n",
            " 'alpha',\n # 注释\n 'mcp__fastctx' # suffix\n",
            "\n # 注释\n 'mcp__fastctx', # suffix\n 'omega'",
        ] {
            let original = format!(
                "# heading\n[features.code_mode]\ndirect_only_tool_namespaces = [{entries}] # tail\nother = 'retained'\n"
            );
            let edited = remove_owned_direct_only_namespace(original.as_bytes()).unwrap();
            let edited = String::from_utf8(edited).unwrap();
            for comment in [
                "# heading",
                "# 注释",
                "# suffix",
                "# tail",
                "other = 'retained'",
            ] {
                assert!(edited.contains(comment), "{edited}");
            }
            assert!(!has_namespace(edited.as_bytes(), "mcp__fastctx"));
            assert_eq!(
                has_namespace(edited.as_bytes(), "alpha"),
                original.contains("alpha")
            );
            assert_eq!(
                has_namespace(edited.as_bytes(), "omega"),
                original.contains("omega")
            );
            assert_eq!(
                remove_owned_direct_only_namespace(edited.as_bytes()).unwrap(),
                edited.as_bytes()
            );
        }
    }
}
