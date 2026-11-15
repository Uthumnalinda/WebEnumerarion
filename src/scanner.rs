use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use tokio::sync::Semaphore;

pub fn start_scan_task(
    target_url: String, 
    custom_path: Option<PathBuf>, 
    tx: Sender<(u16, String)>
) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap();

            let semaphore = Arc::new(Semaphore::new(50));
            let mut handles = vec![];

            let paths = if let Some(path) = custom_path {
                if let Ok(file) = File::open(path) {
                    let reader = BufReader::new(file);
                    reader.lines()
                        .filter_map(|l| l.ok())
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty() && !s.starts_with('#'))
                        .collect()
                } else {
                    vec!["admin".to_string(), "login".to_string(), "api".to_string()]
                }
            } else {
                vec![
                    "admin".to_string(), "login".to_string(), "api".to_string(), 
                    "dashboard".to_string(), "robots.txt".to_string(), ".env".to_string()
                ]
            };

            for path in paths {
                let client = client.clone();
                let target = target_url.clone();
                let tx = tx.clone();
                let permit = Arc::clone(&semaphore);

                let handle = tokio::spawn(async move {
                    let _permit = permit.acquire().await.unwrap();

                    let url = format!("{}/{}", target.trim_end_matches('/'), path);
                    if let Ok(resp) = client.get(&url).send().await {
                        let status = resp.status().as_u16();
                        if status != 404 {
                            let _ = tx.send((status, url));
                        }
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                let _ = handle.await;
            }

            let _ = tx.send((0, "DONE".to_string()));
        });
    });
}