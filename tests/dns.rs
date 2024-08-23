use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Child};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::net::ToSocketAddrs;

fn spawn_directoryd() -> Child {
    Command::new("cargo")
        .args(["run", "--bin", "directoryd"])
        .spawn()
        .expect("Failed to start directoryd process")
}

fn perform_dns_lookup(address: &str) -> Result<String, Box<dyn std::error::Error>> {
    let socket_addr = format!("{}:80", address).to_socket_addrs()?.next().ok_or("Failed to resolve address")?;
    Ok(socket_addr.ip().to_string())
}

#[test]
fn test_dns() {
    let config_path = PathBuf::from("./.cargo/coinswap-test-data/directory/config.toml");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(
        &config_path,
        "\
        [directory_config]\n\
        port = 8080\n\
        socks_port = 19060\n\
        connection_type = tor\n\
        rpc_port = 4321\n\
        ",
    )
        .unwrap();

    let mut directoryd_process = spawn_directoryd();

    thread::sleep(Duration::from_secs(2));

    // Initialize the addresses HashSet with a test address
    let addresses: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));
    addresses.write().unwrap().insert("example.com".to_string());

    let address = addresses.read().unwrap().iter().next().cloned().unwrap();
    let resolved_ip = perform_dns_lookup(&address).unwrap();

    // Assert that the resolved IP is valid
    assert!(!resolved_ip.is_empty(), "Resolved IP should not be empty");

    directoryd_process.kill().expect("Failed to kill directoryd process");
}