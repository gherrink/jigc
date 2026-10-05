//! **The installed guide artifact exactly as an older build would have left it** — the
//! one fixture three suites share (`adapter_artifact`, `setup_install_pathspec_guard`,
//! `flow48_acceptance`).
//!
//! Each of them used to fake an older build's copy by moving the front matter's
//! `jigc-version:` line alone. That is not what an older build writes — a build puts its
//! version in the stamp **and** in the body's opening sentence, and they agree — it is a
//! hand edit to the front matter, which the ownership oracle now reads as exactly that
//! (the rc.24 fix pass's completion audit, install-teardown F3: an edit anywhere in the
//! file makes the copy the adopter's). So the fixture is built the way the copy is.

/// `installed` — what the running build wrote — with the version moved to `version` in
/// both places a build writes it, and the digest re-recorded over the body that results.
pub fn as_an_older_build_wrote_it(installed: &str, version: &str) -> String {
    let running = env!("CARGO_PKG_VERSION");
    let (front, body) = installed
        .split_once("\n---\n\n")
        .expect("the artifact has a front matter");
    let older_body = body.replacen(
        &format!("from jigc {running} "),
        &format!("from jigc {version} "),
        1,
    );
    assert_ne!(
        older_body, body,
        "the body's opening sentence names the build"
    );
    let front: Vec<String> = front
        .lines()
        .map(|line| {
            if line.starts_with("jigc-version:") {
                format!("jigc-version: {version}")
            } else if line.starts_with("jigc-body-blake3:") {
                format!(
                    "jigc-body-blake3: {}",
                    engine::file_state::hash_bytes(older_body.as_bytes())
                )
            } else {
                line.to_string()
            }
        })
        .collect();
    format!("{}\n---\n\n{older_body}", front.join("\n"))
}
