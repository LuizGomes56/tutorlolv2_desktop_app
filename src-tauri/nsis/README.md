# NSIS packaging

`installer.nsi` is based on tauri-bundler 2.10.4, `src/bundle/windows/nsis/installer.nsi`, under Apache-2.0 OR MIT: https://github.com/tauri-apps/tauri/tree/tauri-bundler-v2.10.4/crates/tauri-bundler/src/bundle/windows/nsis .

The only behavioral change is in PageLeaveReinstall: every installer-driven call to an old NSIS uninstaller passes `/UPDATE`. Upstream only does so when the installer itself was started in update mode. Without this change, interactive replacement may invoke the post-uninstall hook and erase device credentials/history. Standalone uninstall has no `/UPDATE` flag and removes both exact per-user bundle-ID app-data directories, regardless of the optional checkbox.

When upgrading tauri-bundler, compare its template with this vendored file, rebase the small replacement change, and rerun the installer tests. The hook uses `${BUNDLEID}`, so a disposable QA bundle identifier can test the exact cleanup without touching the user's production data. Never test cleanup by uninstalling the maintainer's production app.
