use anyhow::Result;
use crate::ipc;

pub fn run() -> Result<()> {
    let mut client = ipc::IpcClient::connect()?;
    let _ = client.send_message(ipc::IpcMessage::Stop)?;
    println!("Service stopped");
    Ok(())
}
