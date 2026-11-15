use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn save_html_report(
    path: PathBuf, 
    target: &str, 
    results_200: &[String], 
    results_403: &[String], 
    results_others: &[String]
) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;

    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>WebEnum Scan Report - {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; background: #0f172a; color: #f8fafc; padding: 20px; }}
        h1 {{ color: #38bdf8; }}
        .container {{ display: flex; gap: 20px; }}
        .column {{ flex: 1; background: #1e293b; padding: 15px; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.3); }}
        .green {{ color: #4ade80; }}
        .yellow {{ color: #facc15; }}
        .blue {{ color: #60a5fa; }}
        ul {{ padding-left: 20px; word-break: break-all; }}
    </style>
</head>
<body>
    <h1>🌐 WebEnum Security Scan Report</h1>
    <p><strong>Target URL:</strong> {}</p>
    <hr style="border-color: #334155;">
    <div class="container">
        <div class="column">
            <h3 class="green">🟢 200 OK ({})</h3>
            <ul>{}</ul>
        </div>
        <div class="column">
            <h3 class="yellow">🟡 403 Forbidden ({})</h3>
            <ul>{}</ul>
        </div>
        <div class="column">
            <h3 class="blue">🔵 Others / 3xx, 5xx ({})</h3>
            <ul>{}</ul>
        </div>
    </div>
</body>
</html>"#,
        target,
        target,
        results_200.len(),
        results_200.iter().map(|s| format!("<li>{}</li>", s)).collect::<String>(),
        results_403.len(),
        results_403.iter().map(|s| format!("<li>{}</li>", s)).collect::<String>(),
        results_others.len(),
        results_others.iter().map(|s| format!("<li>{}</li>", s)).collect::<String>()
    );

    file.write_all(html_content.as_bytes())?;
    Ok(())
}