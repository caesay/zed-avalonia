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
            "axsg-lsp" => {
                let path = worktree.which("axsg-lsp").ok_or_else(|| {
                    "axsg-lsp not found on PATH. Install it with:\n  \
                     dotnet tool install --global XamlToCSharpGenerator.LanguageServer.Tool --prerelease\n\
                     (requires the .NET 10 SDK, and ~/.dotnet/tools on your PATH)."
                        .to_string()
                })?;
                Ok(zed::Command {
                    command: path,
                    args: vec![],
                    env: vec![],
                })
            }
            _ => Err(format!("Unknown language server: {}", language_server_id.as_ref())),
        }
    }
}

zed::register_extension!(AvaloniaExtension);
