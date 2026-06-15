use zed_extension_api as zed;

struct AvaloniaExtension;

impl zed::Extension for AvaloniaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        match language_server_id.as_ref() {
            "csharp-ls" => {
                let path = worktree.which("csharp-ls").ok_or_else(|| {
                    "csharp-ls not found. Install with: dotnet tool install --global csharp-ls"
                        .to_string()
                })?;
                Ok(zed::Command {
                    command: path,
                    args: vec![],
                    env: vec![],
                })
            }
            "omnisharp" => {
                let path = worktree
                    .which("OmniSharp")
                    .or_else(|| worktree.which("omnisharp"))
                    .ok_or_else(|| {
                        "OmniSharp not found. See https://github.com/OmniSharp/omnisharp-roslyn for installation."
                            .to_string()
                    })?;
                Ok(zed::Command {
                    command: path,
                    args: vec!["--languageserver".to_string()],
                    env: vec![],
                })
            }
            "roslyn" => {
                let path = worktree
                    .which("Microsoft.CodeAnalysis.LanguageServer")
                    .ok_or_else(|| {
                        "Roslyn language server not found. Install the .NET SDK and ensure Microsoft.CodeAnalysis.LanguageServer is in PATH."
                            .to_string()
                    })?;
                Ok(zed::Command {
                    command: path,
                    args: vec!["--logLevel=Information".to_string()],
                    env: vec![],
                })
            }
            _ => Err(format!("Unknown language server: {}", language_server_id.as_ref())),
        }
    }
}

zed::register_extension!(AvaloniaExtension);
