fn simple_command_parser(cmd: String) -> Command {
    if cmd.to_lowercase() =="ping" {
        return Command::Ping;
    }
    else if !cmd.contains("::") {
        return Command::Get{key:cmd};
    }
    else {
        let res: Vec<String> = cmd.split("::").map(|s| s.to_string()).collect();
        let key = &res[0];
        let value = &res[1];
        return Command::Set{key: key.clone(), value: value.clone() };
    };
}