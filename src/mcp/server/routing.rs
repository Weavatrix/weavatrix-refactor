//! How one call names the repository it wants.

use blazingly_json::Value;
use std::path::Path;

pub(super) fn text<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
}

pub(super) fn strip_routing(arguments: &mut Value) {
    let Some(object) = arguments.as_object_mut() else {
        return;
    };
    object.remove("repository");
    object.remove("agent");
    object.remove("expected_repository");
}

pub(super) fn accept_expected(
    arguments: &Value,
    launch: &Path,
    target: &Path,
) -> Result<(), String> {
    let Some(expected) = text(arguments, "expected_repository") else {
        return Ok(());
    };
    if same_place(Path::new(expected), launch) || same_place(Path::new(expected), target) {
        return Ok(());
    }
    Err(format!(
        "active repository is {}, not the expected {expected}",
        target.display()
    ))
}

fn same_place(left: &Path, right: &Path) -> bool {
    if let (Ok(left), Ok(right)) = (left.canonicalize(), right.canonicalize()) {
        return left == right;
    }
    norm(left) == norm(right)
}

fn norm(path: &Path) -> String {
    let raw = path.to_string_lossy();
    #[cfg(windows)]
    {
        raw.trim_end_matches(['/', '\\'])
            .replace('/', "\\")
            .to_ascii_lowercase()
    }
    #[cfg(not(windows))]
    {
        raw.trim_end_matches('/').to_owned()
    }
}
