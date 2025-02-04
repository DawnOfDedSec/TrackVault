use std::io::{self, Write};
use std::process::Command;

fn create_task(task_name: &str, exe_path: &str) -> io::Result<()> {
    // Check if the task already exists
    let _ = match Command::new("schtasks")
        .args(&["/Query", "/TN", task_name, "/FO", "LIST"])
        .output()
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    };

    // Create the scheduled task
    let create_task = Command::new("schtasks")
        .args(&[
            "/Create", "/TN", task_name, "/TR", exe_path, "/SC", "HOURLY", "/MO", "1", "/RL",
            "HIGHEST", "/F",
        ])
        .output()?;

    if !create_task.status.success() {
        eprintln!("Failed to create hourly task.");
        io::stderr().write_all(&create_task.stderr)?;
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to create hourly task.",
        ));
    }

    println!("Hourly task '{}' created successfully.", task_name);

    // Create the logon task
    let create_logon_task = Command::new("schtasks")
        .args(&[
            "/Create", "/TN", task_name, "/TR", exe_path, "/SC", "ONLOGON", "/RL", "HIGHEST", "/F",
        ])
        .output()?;

    if !create_logon_task.status.success() {
        eprintln!("Failed to create logon task.");
        io::stderr().write_all(&create_logon_task.stderr)?;
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to create logon task.",
        ));
    }

    println!("Logon task '{}' created successfully.", task_name);

    Ok(())
}
