use std::process::Child;
use core::data::global::GlobalData;
use core::data::global::ServerData;

pub(crate) struct RunningServer {
    pub name: String,
    pub process: Child
}

pub fn start_servers(global_data: &GlobalData) -> Result<Vec<RunningServer>, String> {
    let mut running_servers = vec![];

    for server in global_data.servers.iter() {
        let running_server = start_server(server)?;
        running_servers.push(running_server);
    }

    Ok(running_servers)
}

fn start_server(server: &ServerData) -> Result<RunningServer, String> {
    let mut process = std::process::Command::new("java")
        .args(server.java_vm_options.split(" "))
        .arg("-jar")
        .arg(server.startup_file.clone())
        .spawn()
        .expect("Failed to start server");

    Ok(RunningServer {
        name: server.name.clone(),
        process,
    })
}