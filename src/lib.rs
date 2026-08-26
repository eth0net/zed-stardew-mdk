use std::fs;
use std::path::{Path, PathBuf};

use zed_extension_api::{
    self as zed,
    serde_json::{json, Value},
    settings::LspSettings,
    Command, LanguageServerId, LanguageServerInstallationStatus, Result, Worktree,
};

const SERVER_ID: &str = "stardew-json-language-server";
const PACKAGE_NAME: &str = "vscode-langservers-extracted";
const SERVER_PATH: &str =
    "node_modules/vscode-langservers-extracted/bin/vscode-json-language-server";

const CONTENT_PATCHER_ID: &str = "Pathoschild.ContentPatcher";

/// The canonical SMAPI schemas, vendored so validation works offline and can't
/// change under the author mid-session. Refresh with `scripts/update-schemas.sh`.
const SCHEMAS: &[(&str, &str)] = &[
    ("manifest.json", include_str!("../schemas/manifest.json")),
    (
        "content-patcher.json",
        include_str!("../schemas/content-patcher.json"),
    ),
    ("i18n.json", include_str!("../schemas/i18n.json")),
];

#[derive(Default)]
struct StardewExtension {
    server_installed: bool,
}

impl StardewExtension {
    /// Zed resolves a relative command against the *worktree*, so the script
    /// path has to be absolute — the server lives in the extension work dir.
    fn server_script(&mut self, id: &LanguageServerId) -> Result<String> {
        let absolute = || -> Result<String> {
            Ok(work_dir()?.join(SERVER_PATH).to_string_lossy().into_owned())
        };
        let present = || fs::metadata(SERVER_PATH).is_ok_and(|stat| stat.is_file());

        if self.server_installed && present() {
            return absolute();
        }

        zed::set_language_server_installation_status(
            id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let latest = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed = zed::npm_package_installed_version(PACKAGE_NAME)?;

        if installed.as_deref() != Some(latest.as_str()) || !present() {
            zed::set_language_server_installation_status(
                id,
                &LanguageServerInstallationStatus::Downloading,
            );
            if let Err(err) = zed::npm_install_package(PACKAGE_NAME, &latest) {
                // An older copy still validates fine; only fail if there's nothing.
                if !present() {
                    return Err(err);
                }
            }
        }

        self.server_installed = true;
        absolute()
    }
}

impl zed::Extension for StardewExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        if let Some(binary) = LspSettings::for_worktree(SERVER_ID, worktree)
            .ok()
            .and_then(|settings| settings.binary)
        {
            if let Some(path) = binary.path {
                return Ok(Command {
                    command: path,
                    args: binary.arguments.unwrap_or_else(|| vec!["--stdio".into()]),
                    env: Default::default(),
                });
            }
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![self.server_script(id)?, "--stdio".into()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<Value>> {
        Ok(Some(json!({ "provideFormatter": true })))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let dir = staged_schema_dir()?;

        let mut schemas = vec![
            association(&dir, "manifest.json", &["**/manifest.json"]),
            association(&dir, "i18n.json", &["**/i18n/*.json", "**/i18n/**/*.json"]),
        ];
        if wants_content_patcher(worktree) {
            schemas.push(association(
                &dir,
                "content-patcher.json",
                &["**/content.json"],
            ));
        }

        let mut config = json!({
            "json": {
                "validate": { "enable": true },
                "format": { "enable": true },
                "schemas": schemas,
            }
        });

        // Anything under `lsp.stardew-json-language-server.settings` wins, so an
        // author can point a file at their own schema or drop ours entirely.
        if let Some(user) = LspSettings::for_worktree(SERVER_ID, worktree)
            .ok()
            .and_then(|settings| settings.settings)
        {
            merge(&mut config, user);
        }

        Ok(Some(config))
    }
}

/// Several frameworks name their content pack `content.json`, so only claim it
/// for Content Patcher when the mod says that's what it is. With no manifest at
/// the worktree root — a multi-mod folder, say — assume Content Patcher: it is
/// by far the most common, and `json.schemas` in settings is the opt-out.
fn wants_content_patcher(worktree: &Worktree) -> bool {
    match worktree.read_text_file("manifest.json") {
        Ok(manifest) => manifest.contains(CONTENT_PATCHER_ID),
        Err(_) => true,
    }
}

/// The extension's work directory. Relative filesystem access from the wasm
/// guest lands here, but paths handed to Zed or to the server must be absolute.
fn work_dir() -> Result<PathBuf> {
    std::env::current_dir()
        .map(zed_ext::sanitize_windows_path)
        .map_err(|err| err.to_string())
}

/// Unpacks the vendored schemas into the extension's work directory, where the
/// language server process can read them by absolute path.
fn staged_schema_dir() -> Result<PathBuf> {
    let dir = work_dir()?.join("schemas");
    fs::create_dir_all(&dir).map_err(|err| err.to_string())?;

    for (name, body) in SCHEMAS {
        let path = dir.join(name);
        if fs::read_to_string(&path).ok().as_deref() != Some(*body) {
            fs::write(&path, body).map_err(|err| err.to_string())?;
        }
    }

    Ok(dir)
}

fn association(dir: &Path, schema: &str, file_match: &[&str]) -> Value {
    json!({
        "fileMatch": file_match,
        "url": file_uri(&dir.join(schema)),
    })
}

/// The language server resolves schema locations as URIs, so the staged paths
/// have to be encoded as such. Zed's extension work directory contains a space
/// on macOS ("Application Support") and a drive letter on Windows, and both
/// forms have to come out as something `vscode-uri` will parse.
fn file_uri(path: &Path) -> String {
    // `file:///C:/…` on Windows, `file:///Users/…` elsewhere: separators are
    // always forward slashes and the path is always root-anchored.
    let path = path.to_string_lossy().replace('\\', "/");
    let mut uri = String::from("file://");
    if !path.starts_with('/') {
        uri.push('/');
    }

    for byte in path.bytes() {
        match byte {
            // `:` is left alone so a drive letter survives as `C:` rather than
            // `C%3A`, which not every URI parser accepts in a path.
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'/' | b':' | b'-' | b'_' | b'.' | b'~' => {
                uri.push(byte as char)
            }
            _ => uri.push_str(&format!("%{byte:02X}")),
        }
    }

    uri
}

/// Extensions to the Zed extension API that have not yet stabilized.
mod zed_ext {
    /// Strips the leading `/` wasmtime prepends to Windows paths.
    ///
    /// On macOS and Linux this is a no-op.
    ///
    /// Works around <https://github.com/bytecodealliance/wasmtime/issues/10415>.
    pub fn sanitize_windows_path(path: std::path::PathBuf) -> std::path::PathBuf {
        use zed_extension_api::{current_platform, Os};

        let (os, _arch) = current_platform();
        match os {
            Os::Mac | Os::Linux => path,
            Os::Windows => path
                .to_string_lossy()
                .trim_start_matches('/')
                .to_string()
                .into(),
        }
    }
}

fn merge(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Object(base), Value::Object(overlay)) => {
            for (key, value) in overlay {
                merge(base.entry(key).or_insert(Value::Null), value);
            }
        }
        (base, overlay) => *base = overlay,
    }
}

zed::register_extension!(StardewExtension);

#[cfg(test)]
mod tests {
    use super::file_uri;
    use std::path::Path;

    #[test]
    fn encodes_a_macos_work_directory() {
        assert_eq!(
            file_uri(Path::new(
                "/Users/x/Library/Application Support/Zed/extensions/work/stardew-mdk/schemas/manifest.json"
            )),
            "file:///Users/x/Library/Application%20Support/Zed/extensions/work/stardew-mdk/schemas/manifest.json"
        );
    }

    #[test]
    fn encodes_a_windows_work_directory() {
        // Backslashes become separators, the drive letter keeps its colon, and
        // the authority is followed by a root slash: `file:///C:/…`.
        assert_eq!(
            file_uri(Path::new(
                r"C:\Users\x\AppData\Local\Zed\schemas\manifest.json"
            )),
            "file:///C:/Users/x/AppData/Local/Zed/schemas/manifest.json"
        );
    }
}
