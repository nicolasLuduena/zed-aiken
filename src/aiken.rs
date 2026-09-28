use zed::lsp::CompletionKind;
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, settings::LspSettings, EnvVars, Result};

struct AikenExtension;

impl zed::Extension for AikenExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Users can point the extension at a specific Aiken binary (for example one
        // installed through `aikup`) via:
        //
        //   "lsp": { "aiken": { "binary": { "path": "/path/to/aiken" } } }
        //
        // Everything is optional; without any settings we fall back to `aiken` on $PATH.
        let settings =
            LspSettings::for_worktree(language_server_id.as_ref(), worktree).unwrap_or_default();
        let binary = settings.binary;

        let command = match binary.as_ref().and_then(|binary| binary.path.clone()) {
            Some(path) => path,
            None => worktree.which("aiken").ok_or_else(aiken_not_found)?,
        };

        let args = binary
            .as_ref()
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_else(|| vec!["lsp".to_string()]);

        let env: EnvVars = binary
            .and_then(|binary| binary.env)
            .unwrap_or_default()
            .into_iter()
            .collect();

        Ok(zed::Command { command, args, env })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        let name = &completion.label;
        let ty = strip_newlines_from_detail(&completion.detail?);
        let let_binding = "let a";
        let colon = ": ";
        let assignment = " = ";
        let call = match completion.kind? {
            CompletionKind::Function | CompletionKind::Constructor => "()",
            _ => "",
        };
        let code = format!("{let_binding}{colon}{ty}{assignment}{name}{call}");

        Some(CodeLabel {
            spans: vec![
                CodeLabelSpan::code_range({
                    let start = let_binding.len() + colon.len() + ty.len() + assignment.len();
                    start..start + name.len()
                }),
                CodeLabelSpan::code_range({
                    let start = let_binding.len();
                    start..start + colon.len()
                }),
                CodeLabelSpan::code_range({
                    let start = let_binding.len() + colon.len();
                    start..start + ty.len()
                }),
            ],
            filter_range: (0..name.len()).into(),
            code,
        })
    }
}

zed::register_extension!(AikenExtension);

fn aiken_not_found() -> String {
    "aiken not found; install it via https://aiken-lang.org/installation-instructions, or set \
     `lsp.aiken.binary.path` to a specific binary"
        .to_string()
}

fn strip_newlines_from_detail(detail: &str) -> String {
    let without_newlines = detail
        .replace("->\n  ", "-> ")
        .replace("\n  ", "")
        .replace(",\n", "");

    let comma_delimited_parts = without_newlines.split(',');

    comma_delimited_parts
        .map(|part| part.trim())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use crate::strip_newlines_from_detail;

    #[test]
    fn test_strip_newlines_from_detail() {
        let detail = "fn(\n  Selector(a),\n  b,\n  fn(Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic) -> a,\n) -> Selector(a)";
        let expected = "fn(Selector(a), b, fn(Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic) -> a) -> Selector(a)";
        assert_eq!(strip_newlines_from_detail(detail), expected);

        let detail = "fn(Selector(a), b, fn(Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic) -> a) ->\n  Selector(a)";
        let expected = "fn(Selector(a), b, fn(Dynamic, Dynamic, Dynamic, Dynamic, Dynamic, Dynamic) -> a) -> Selector(a)";
        assert_eq!(strip_newlines_from_detail(detail), expected);

        let detail = "fn(\n  Method,\n  List(#(String, String)),\n  a,\n  Scheme,\n  String,\n  Option(Int),\n  String,\n  Option(String),\n) -> Request(a)";
        let expected = "fn(Method, List(#(String, String)), a, Scheme, String, Option(Int), String, Option(String)) -> Request(a)";
        assert_eq!(strip_newlines_from_detail(detail), expected);
    }
}
