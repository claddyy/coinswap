use coinswap::{
    maker::error::MakerError,
    market::rpc::{RpcMsgReq, RpcMsgResp},
    utill::{read_message, send_message},
};
use std::{
    fs, io,
    io::BufRead,
    net::TcpStream,
    path::PathBuf,
    process::{Child, Command},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

fn spawn_directoryd_thread(log_sender: Sender<String>) -> Child {
    let mut directoryd_process = Command::new("cargo")
        .args(["run", "--bin", "directoryd"])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start directoryd process");

    let stdout = directoryd_process.stdout.take().unwrap();
    thread::spawn(move || {
        let reader = io::BufReader::new(stdout);
        for log_message in reader.lines().map_while(Result::ok) {
            log_sender.send(log_message).unwrap();
        }
    });
    directoryd_process
}

fn send_rpc_req(req: &RpcMsgReq) -> Result<Option<RpcMsgResp>, MakerError> {
    let mut stream = TcpStream::connect("127.0.0.1:4321")?;
    stream.set_read_timeout(Some(Duration::from_secs(20)))?;
    stream.set_write_timeout(Some(Duration::from_secs(20)))?;

    send_message(&mut stream, req)?;

    let resp_bytes = read_message(&mut stream)?;
    let resp: RpcMsgResp = serde_cbor::from_slice(&resp_bytes)?;
    Ok(Some(resp))
}

#[test]
fn test_dns() {
    let config_path = PathBuf::from("./.cargo/coinswap-test-data/directory/config.toml");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(
        &config_path,
        "\
        [directory_config]\n\
        port = 8084\n\
        socks_port = 19060\n\
        connection_type = tor\n\
        rpc_port = 4321\n\
        ",
    )
    .unwrap();

    let (log_sender, log_receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    let mut directoryd_process = spawn_directoryd_thread(log_sender);

    let mut server_started = false;
    while let Ok(log_message) = log_receiver.recv_timeout(Duration::from_secs(5)) {
        if log_message.contains("RPC socket binding successful") {
            server_started = true;
            break;
        }
    }
    assert!(
        server_started,
        "Server did not start within the expected time"
    );
    thread::sleep(Duration::from_secs(5));

    let resp = send_rpc_req(&RpcMsgReq::ListAddresses).unwrap();

    if let Some(RpcMsgResp::ListAddressesResp(addresses)) = resp {
        assert!(addresses.is_empty(), "Expected an empty list of addresses");
    } else {
        panic!("Unexpected RPC response");
    }

    directoryd_process
        .kill()
        .expect("Failed to kill directoryd process");
}
