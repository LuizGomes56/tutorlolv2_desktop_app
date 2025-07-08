use synoptic::{Highlighter, TokOpt};
use web_sys::console;

pub fn highlight(code_string: &str) -> String {
    console::log_1(&"highlighting".into());

    let mut h = Highlighter::new(4);
    h.bounded("comment", r"/\*", r"\*/", false);
    h.keyword("comment", r"//.*$");
    h.bounded_interp("string", "\"", "\"", "\\{", "\\}", true);
    h.keyword("lifetime", r"'\w+");
    h.keyword(
        "keyword",
        r"\b(pub|use|crate|super|mod|struct|const|enum|fn|let|impl|trait|where|loop|Self|self)\b",
    );
    h.keyword(
        "control",
        r"\b(break|continue|match|return|yield|for|while|match|if|else|as|in)\b",
    );
    h.keyword("constvar", r"\b[A-Z]+\b");
    h.keyword("type", r"\b[A-Z][a-zA-Z0-9_]*\b");
    h.keyword(
        "primitive",
        r"\b(bool|usize|i32|i64|f64|char|str|writer_macros)\b",
    );
    h.keyword("constant", r"\b(mut|static|dyn|unsafe|extern|type)\b");
    h.keyword("float", r"\b\d+\.?\d*f64\b");
    h.keyword("number", r"\b\d+\.?\d*\b");
    h.keyword("boolean", r"\b(true|false)\b");
    h.keyword("macro", r"[a-zA-Z_][a-zA-Z0-9_]*!");
    h.keyword("function", r"\b[a-z][a-zA-Z0-9_]*\(");
    h.keyword("variable", r"\b[a-z][a-zA-Z0-9_]*\b");
    h.keyword("punctuation", r"[+\-*/=&^|!:;,<>.\[\]{}()] ");

    let code = code_string
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>();

    h.run(&code);

    let mut out = String::new();
    for (i, line) in code.iter().enumerate() {
        let mut line_html = String::new();
        for token in h.line(i, line) {
            match token {
                TokOpt::Some(text, kind) => {
                    line_html.push_str(&format!("<span class=\"token {kind}\">{text}</span>"));
                }
                TokOpt::None(text) => {
                    line_html.push_str(&text);
                }
            }
        }
        out.push_str(&line_html);
        out.push_str("\n");
    }
    format!("<pre class=\"code-highlight\">{}</pre>", out)
}
