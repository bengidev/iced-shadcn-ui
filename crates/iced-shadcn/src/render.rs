use crate::config::Config;

pub fn render_template(template: &str, config: &Config) -> String {
    template
        .replace("{{ui_path}}", &config.ui_module_path())
        .replace("{{style}}", &config.style)
        .replace("{{base_color}}", &config.base_color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitutes_placeholders() {
        let config = Config::default();
        let template = "use {{ui_path}}::theme;\nstyle={{style}}\nbase={{base_color}}";
        let output = render_template(template, &config);
        assert!(output.contains("use ui::theme;"));
        assert!(output.contains("style=new-york"));
        assert!(output.contains("base=neutral"));
    }
}
