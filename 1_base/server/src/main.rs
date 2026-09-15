use tokio::process::Command;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for _ in 0..4 {
        let handle = tokio::spawn(async {
            let output = Command::new("./connection_bin").output().await;
            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("Process completed with output: {stdout}");
                    Ok(output.status.code().unwrap_or(-1))
                }
                Err(e) => {
                    eprintln!("Failed to start the process: {e}");
                    Err(e)
                }
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        #[allow(clippy::unwrap_used)]
        results.push(handle.await.unwrap());
    }
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(exit_code) => println!(
                "Process {} exited with code {exit_code}",
                i.saturating_add(1)
            ),
            Err(e) => eprintln!("Process {} failed: {e}", i.saturating_add(1)),
        }
    }
}
