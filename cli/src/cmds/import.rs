use std::env::current_dir;
use std::process::exit;
use inquire::{Confirm, Text};
use core::data::global::GlobalData;

pub(crate) fn run(initial_data: &GlobalData, arg_path: &str) {
    // get the path of the server
    let cwd = current_dir().unwrap();
    let server_path = cwd.join(arg_path).canonicalize();
    let server_path = match server_path {
        Ok(path) => path,
        Err(error) => panic!("Path does not exist: {:?}", error),
    };

    // get the name of the server from the user
    let dirname = server_path.file_name().unwrap().to_str().unwrap();
    let servername = Text::new("What is the name of the server?")
        .with_default(dirname)
        .prompt();
    let server_name = match servername {
        Ok(name) => name,
        Err(error) => panic!("Failed to get server name: {:?}", error),
    };

    // get the startup file from the user
    let startup_file_prompt = Text::new("What is the name of the startup file?").prompt();
    let startup_file = match startup_file_prompt {
        Ok(file) => file,
        Err(error) => panic!("Failed to get startup file: {:?}", error),
    };

    // add the server to the global data
    let server = core::data::global::ServerData {
        path: server_path,
        name: server_name.clone(),
        startup_file,
        java_vm_options: String::from("-Xms128M -Xmx6144M"),
        status: core::data::global::ServerStatus::Stopped,
        autostart: true,
    };

    let mut gdata = initial_data.clone();
    let mut servers = gdata.servers.clone();

    servers.push(server);
    gdata.servers = servers;

    gdata.write().expect("Failed to write global data");

    let run_now_prompt = Confirm::new(
        "Would you like to run this server now?",
    )
        .with_help_message(format!(
            "If you don't, youll need to run 'mcprocess start {:?}' to start the server",
            &server_name
        ).as_str())
        .prompt();

    let run_now_prompt = match run_now_prompt {
        Ok(true) => true,
        Ok(false) => {
            println!("Okay! You can run it later");
            exit(0);
        },
        Err(error) => panic!("Failed to get create container prompt: {:?}", error),
    };
}
