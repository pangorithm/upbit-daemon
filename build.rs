use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("expected_tables.yaml");

    let all_sql = read_migrations();
    let drop_tables = parse_drop_tables(&all_sql);
    let tables = parse_create_tables(&all_sql, &drop_tables);

    let code = generate_yaml(&tables);
    fs::write(&dest_path, &code).expect("Failed to write expected_tables.yaml");

    println!("cargo::rerun-if-changed=migrations");
}

fn read_migrations() -> String {
    let migration_dir = Path::new("migrations");
    let mut all_sql = String::new();
    let mut entries: Vec<_> = fs::read_dir(migration_dir)
        .expect("Failed to read migrations directory")
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "sql") {
            let sql = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", path, e));
            all_sql.push_str(&sql);
            all_sql.push('\n');
        }
    }
    all_sql
}

fn parse_drop_tables(sql: &str) -> HashSet<&str> {
    sql.lines()
        .filter(|l| l.trim().to_lowercase().starts_with("drop"))
        .filter_map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() >= 3 && parts[1].to_lowercase() == "table" {
                let name = parts[2].trim_end_matches(';').trim();
                if !name.is_empty() { Some(name) } else { None }
            } else { None }
        })
        .collect()
}

fn parse_create_tables(sql: &str, drop_tables: &HashSet<&str>) -> Vec<String> {
    let mut tables: Vec<String> = sql
        .lines()
        .filter(|l| {
            let trimmed = l.trim();
            trimmed.to_lowercase().starts_with("create table ")
                && !trimmed.to_lowercase().contains("partition of")
        })
        .filter_map(|l| {
            let parts: Vec<&str> = l.trim().split_whitespace().collect();
            if parts.len() >= 4 {
                let name = parts[2].trim_end_matches('(').trim_end_matches(';').trim();
                if !name.is_empty() && !drop_tables.contains(name) {
                    Some(name.to_string())
                } else { None }
            } else { None }
        })
        .collect();
    tables.sort();
    tables
}

fn generate_yaml(tables: &[String]) -> String {
    let lines: Vec<String> = tables.iter().map(|t| format!("- {t}")).collect();
    format!("tables:\n{}\n", lines.join("\n"))
}
