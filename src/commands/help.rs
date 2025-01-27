use std::vec;

pub async fn help_command() {
    println!("Commands:");

    let commands = read_commands().await;

    for command in commands {
        println!("{}", command);
    }
}

async fn read_commands() -> Vec<String> {
    let dir = std::env::current_dir().unwrap();
    let dir = dir.join("src/commands");

    let mut commands = vec![];

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".rs", "");
        commands.push(name.to_string());
    }

    commands
}
