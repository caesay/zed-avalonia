# Avalonia for Zed

Avalonia UI support for the [Zed editor](https://zed.dev) — syntax highlighting, code folding, outline, snippets, and language server integration for `.axaml` files.

## Features

- Syntax highlighting for `.axaml` files (via tree-sitter XML grammar)
- Code folding for elements and comments
- Document outline showing element structure
- 18 Avalonia-specific snippets (`avwindow`, `avbutton`, `avgrid`, etc.)
- Language server support (csharp-ls, OmniSharp, or Roslyn)

## Requirements

A C# language server must be installed. [csharp-ls](https://github.com/razzmatazz/csharp-language-server) is recommended:

```sh
dotnet tool install --global csharp-ls
```

## Installation

### From Zed Extensions (once published)

Open the command palette (`Cmd+Shift+P`), search "Extensions", and install "Avalonia".

### As a Dev Extension

Clone this repo and add it as a dev extension in Zed:

1. Clone: `git clone https://github.com/cwhittl/zed-avalonia.git`
2. In Zed, open the command palette and run `zed: install dev extension`
3. Select the cloned directory

## Language Server Configuration

The extension supports three C# language servers. By default all three are registered — disable the ones you don't use in your Zed settings (`~/.config/zed/settings.json`):

```json
{
  "languages": {
    "Avalonia": {
      "language_servers": ["csharp-ls", "!omnisharp", "!roslyn"]
    }
  }
}
```

| Server | Binary | Install |
|--------|--------|---------|
| csharp-ls | `csharp-ls` | `dotnet tool install --global csharp-ls` |
| OmniSharp | `OmniSharp` or `omnisharp` | [omnisharp-roslyn](https://github.com/OmniSharp/omnisharp-roslyn) |
| Roslyn | `Microsoft.CodeAnalysis.LanguageServer` | Included with .NET SDK |

## Snippets

All snippets are prefixed with `av`:

| Prefix | Description |
|--------|-------------|
| `avwindow` | Window |
| `avusercontrol` | UserControl |
| `avstack` | StackPanel |
| `avgrid` | Grid with row/column definitions |
| `avbutton` | Button with command binding |
| `avtextblock` | TextBlock |
| `avtextbox` | TextBox with binding |
| `avitemscontrol` | ItemsControl with DataTemplate |
| `avlistbox` | ListBox with selection binding |
| `avdatatemplate` | DataTemplate |
| `avstyle` | Style with Selector |
| `avbinding` | Binding expression |
| `avcompiledbinding` | Compiled binding expression |
| `avdesigndc` | Design-time data context |
| `avborder` | Border |
| `avdockpanel` | DockPanel |
| `avmenu` | Menu with MenuItems |
| `avtabcontrol` | TabControl |

## Toggle Between .axaml and Code-Behind

Zed doesn't support extension-provided commands yet, but you can set up tasks to toggle between `.axaml` and `.axaml.cs` files.

Add to your global tasks file (`~/.config/zed/tasks.json`):

```json
[
  {
    "label": "Avalonia: Open Code-Behind (.axaml → .axaml.cs)",
    "command": "zed \"${ZED_FILE}.cs\"",
    "reveal": "never",
    "hide": "on_success",
    "use_new_terminal": false
  },
  {
    "label": "Avalonia: Open Markup (.axaml.cs → .axaml)",
    "command": "zed \"${ZED_DIRNAME}/${ZED_STEM}\"",
    "reveal": "never",
    "hide": "on_success",
    "use_new_terminal": false
  }
]
```

Then bind them to shortcuts in your keymap (`~/.config/zed/keymap.json`):

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "alt-o": ["task::Spawn", { "task_name": "Avalonia: Open Code-Behind (.axaml → .axaml.cs)" }],
      "alt-shift-o": ["task::Spawn", { "task_name": "Avalonia: Open Markup (.axaml.cs → .axaml)" }]
    }
  }
]
```

## License

MIT
