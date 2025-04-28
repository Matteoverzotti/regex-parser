use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::env::temp_dir;
use crate::nfa::NFA;
use crate::dfa::DFA;
use std::collections::HashMap;

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

// I'm pretty sure I could have used the same function for both NFA and DFA
// But couldn't be bothered to figure out how to do that
// So I'm just going to copy paste the code and make a few changes
impl DFA {
    fn to_dot(&self) -> String {
        let mut id_map: HashMap<Vec<String>, String> = HashMap::new();
        for (i, state) in self.states.iter().enumerate() {
            id_map.insert(state.clone(), format!("S{}", i));
        }

        let mut dot = String::new();
        dot.push_str("digraph DFA {\n");
        dot.push_str("    rankdir=LR;\n");
        dot.push_str("    node [shape=point]; start;\n");
        dot.push_str(&format!("    start -> {} ;\n", &id_map[&self.q0]));

        for state in &self.states {
            let id = &id_map[state];
            let label = state.join(",");
            
            if self.final_states.contains(state) {
                dot.push_str(&format!("    {} [shape=doublecircle, label=\"{}\"];\n", id, label));
            } else {
                dot.push_str(&format!("    {} [shape=circle, label=\"{}\"];\n", id, label));
            }
        }

        for (state, map) in &self.transitions {
            let from_id = &id_map[state];
            for (&symbol, targets) in map {
                let to_id = &id_map[targets];
                let label = symbol.to_string();
                dot.push_str(&format!("    {} -> {} [label=\"{}\"];\n", from_id, to_id, label));
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
        dot_path.push(format!("dfa_{}.dot", random_number));
        png_path.push(format!("dfa_{}.png", random_number));

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