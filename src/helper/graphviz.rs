use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::env::temp_dir;
use crate::nfa::NFA;

impl NFA {
    fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph NFA {\n");
        dot.push_str("    rankdir=LR;\n");
        dot.push_str("    node [shape = point]; start;\n");
        dot.push_str(&format!("    start -> {} ;\n", self.q0));

        for state in &self.states {
            if self.final_states.contains(state) {
                dot.push_str(&format!("\t{} [shape = doublecircle];\n", state));
            } else {
                dot.push_str(&format!("\t{} [shape = circle];\n", state));
            }
        }

        for (from, map) in &self.transitions {
            for (&symbol, targets) in map {
                let label = if symbol == '\0' {
                    "ε".to_string()
                } else {
                    symbol.to_string()
                };
                for to in targets {
                    dot.push_str(&format!("\t{} -> {} [label=\"{}\"];\n", from, to, label));
                }
            }
        }

        dot.push_str("}\n");
        dot
    }

    fn write_dot_file(&self, path: &std::path::Path) {
        let mut file = File::create(path)
            .expect("Failed to create dot file");

        file.write_all(self.to_dot().as_bytes())
            .expect("Failed to write to dot file");
    }

    pub fn visualize(&self) {
        let mut dot_path = temp_dir();
        let mut png_path = temp_dir();
        let random_number: u32 = rand::random();
        dot_path.push(format!("nfa_{}.dot", random_number));
        png_path.push(format!("nfa_{}.png", random_number));

        self.write_dot_file(&dot_path);
        Command::new("dot")
            .args(&["-Tpng", dot_path.to_str().unwrap(), "-o", png_path.to_str().unwrap()])
            .status()
            .expect("Failed to run dot command");

        Command::new("xdg-open").arg(&png_path)
            .status()
            .expect("Failed to open image");
    }
}
