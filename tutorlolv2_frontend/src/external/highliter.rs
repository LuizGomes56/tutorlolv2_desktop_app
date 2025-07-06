use synoptic::{Highlighter, TokOpt};

pub static CODE: &str = r##"
#[derive(Debug)]
/// Documentação pública para MyStruct
pub struct MyStruct<'a, T>
where
    T: Clone + 'static,
{
    pub name: &'a str,
    pub values: Vec<T>,
}

// Comentário de linha
/*
   Comentário
   de bloco
*/

use std::collections::HashMap;
use crate::utils::helper_fn;

// Definindo um macro simples
macro_rules! my_macro {
    ($val:expr) => {
        println!("Macro diz: {}", $val);
    };
}

impl<'a, T> MyStruct<'a, T>
where
    T: Clone + 'static,
{
    /// Constrói uma nova instância
    pub fn new(name: &'a str, values: Vec<T>) -> Self {
        Self { name, values }
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }
}

pub trait MyTrait {
    fn do_something(&self) -> bool;
}

impl<'a, T> MyTrait for MyStruct<'a, T>
where
    T: Clone + 'static,
{
    fn do_something(&self) -> bool {
        if self.values.is_empty() {
            return false;
        }
        for value in &self.values {
            match value {
                _ => my_macro!(value),
            }
        }
        true
    }
}

fn main() {
    let s = MyStruct::new("Exemplo", vec![1, 2, 3]);
    println!("Count: {}", s.count());
    println!("Trait: {}", s.do_something());
    let raw = r#"Raw string com "aspas" e {}"#;
    println!("Raw: {}", raw);
    let float = 3.14;
    let int = 42;
    let flag = true;
    my_macro!("Testando macros!");
}
"##;

pub fn highlight() -> String {
    let mut h = Highlighter::new(4);
    h.bounded("comment", r"/\*", r"\*/", false);
    h.keyword("comment", r"//.*$");
    h.bounded_interp("string", "\"", "\"", "\\{", "\\}", true);
    h.keyword("lifetime", r"'\w+");
    h.keyword(
        "keyword",
        r"\b(pub|use|crate|mod|struct|enum|fn|let|impl|trait|where|loop|Self|self)\b",
    );
    h.keyword(
        "control",
        r"\b(break|continue|match|return|yield|for|while|match|if|else|as|in)\b",
    );
    h.keyword("type", r"\b[A-Z][a-zA-Z0-9_]*\b");
    h.keyword("primitive", r"\b(bool|usize|i32|i64|f64|char|str)\b");
    h.keyword("constant", r"\b(const|mut|static|dyn|unsafe|extern|type)\b");
    h.keyword("number", r"\b\d+\.?\d*\b");
    h.keyword("boolean", r"\b(true|false)\b");
    h.keyword("macro", r"[a-zA-Z_][a-zA-Z0-9_]*!");
    h.keyword("function", r"\b[a-z][a-zA-Z0-9_]*\(");
    h.keyword("variable", r"\b[a-z][a-zA-Z0-9_]*\b");
    h.keyword("punctuation", r"[+\-*/=&^|!:;,<>.\[\]{}()] ");

    let code = CODE.lines().map(str::to_string).collect::<Vec<String>>();

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
        out.push_str("<br/>\n");
    }
    format!("<pre class=\"code-highlight\">{}</pre>", out)
}
