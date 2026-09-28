# Zed Aiken

[Aiken](https://aiken-lang.org) language support for [Zed](https://zed.dev):
syntax highlighting plus the language server built into the `aiken` CLI.

## Installing Aiken

The extension delegates language intelligence to the `aiken` binary on your
`$PATH` (`aiken lsp`), so it always uses whatever version you have installed.
The recommended way to install and manage Aiken versions is
[`aikup`](https://aiken-lang.org/installation-instructions):

```sh
aikup   # install the latest version
```

Run `aikup --help` to install and switch between specific versions. Because
`aikup` puts the selected `aiken` on `$PATH`, the extension follows along
automatically.

## Using a specific binary

To point the extension at a binary that isn't on `$PATH` (a custom build, a
project-local toolchain, a different `aikup` install, etc.), set
`lsp.aiken.binary.path` in your Zed settings:

```json
{
  "lsp": {
    "aiken": {
      "binary": {
        "path": "/path/to/aiken"
      }
    }
  }
}
```

`binary.arguments` and `binary.env` are also supported. `arguments` defaults to
`["lsp"]` and only needs overriding if you know what you're doing.

Because Zed settings can be global or per-project (`.zed/settings.json`), you
can run different Aiken versions in different projects.
