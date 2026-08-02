// The shared file-size gate from CODE_STANDARDS §2.2 — the 800-line hard
// limit on non-test lines — enforced under plain `cargo test`.

use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn source_files_stay_under_the_limit() {
    macroquad_toolkit::source_gate::assert_source_files_within_limit(
        env!("CARGO_MANIFEST_DIR"),
        &[],
    );
}

/// The view layer reads the session and returns `UiAction`s; a dispatcher
/// applies them. Handing rendering a `&mut GameSession` would let a camera
/// angle or a visual effect change what the player scored, and the damage
/// would be invisible — no test of the simulation would ever see it.
/// `UiContext` holds a shared reference today; this keeps it that way.
#[test]
fn the_view_layer_cannot_mutate_the_session() {
    let ui_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ui");
    let mut sources = vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ui.rs")];
    collect_rust_sources(&ui_root, &mut sources);
    assert!(
        sources.len() > 5,
        "expected to find the ui modules, found {sources:?}"
    );

    let mut offenders = Vec::new();
    for path in &sources {
        let text = fs::read_to_string(path).expect("read ui source");
        for (line_number, line) in text.lines().enumerate() {
            if line.contains("&mut GameSession") || line.contains("session: &mut") {
                offenders.push(format!(
                    "{}:{}: {}",
                    path.display(),
                    line_number + 1,
                    line.trim()
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "the view layer must not take the session mutably; emit a UiAction and \
         let Game::apply_action mutate it instead:\n{}",
        offenders.join("\n")
    );
}

fn collect_rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}
