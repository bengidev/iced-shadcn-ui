const START: &str = "// iced-shadcn managed start";
const END: &str = "// iced-shadcn managed end";

pub fn patch_mod_rs(existing: &str, modules: &[&str]) -> String {
    let mut module_set: Vec<String> = existing
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("pub mod ")
                .and_then(|rest| rest.strip_suffix(';'))
                .map(str::to_string)
        })
        .collect();

    for module in modules {
        if !module_set.iter().any(|m| m == module) {
            module_set.push((*module).to_string());
        }
    }
    module_set.sort();

    let managed = module_set
        .iter()
        .map(|m| format!("pub mod {m};"))
        .collect::<Vec<_>>()
        .join("\n");

    let managed_block = format!("{START}\n{managed}\n{END}");

    if let (Some(start), Some(end)) = (existing.find(START), existing.find(END)) {
        let mut out = String::new();
        out.push_str(&existing[..start]);
        out.push_str(&managed_block);
        out.push_str(&existing[end + END.len()..]);
        return out;
    }

    format!("{existing}{managed_block}\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_modules_into_managed_section_idempotently() {
        let original = "// user code\n";
        let once = patch_mod_rs(original, &["theme", "button"]);
        let twice = patch_mod_rs(&once, &["theme", "button", "card"]);
        assert!(once.contains("pub mod theme;"));
        assert!(once.contains("pub mod button;"));
        assert!(twice.contains("pub mod card;"));
        assert_eq!(twice.matches("pub mod theme;").count(), 1);
    }
}
