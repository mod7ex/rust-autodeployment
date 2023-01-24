use std::{process::Command, str::from_utf8};
use std::env::args;

fn exe(cmd: &str) -> String {
    let err = format!("Failed to execute command <{}>", cmd);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", cmd])
                .output()
                .expect(&err)
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect(&err)
    };

    String::from(from_utf8(&output.stdout).unwrap())
}

fn main() {
    let _args: Vec<String> = args().collect();

    if _args.len() != 2 {
        println!("[Usage]: {} <$branch name>", _args[0]);
        return;
    }

    let branch = format!("git pull origin {}", _args[1]);

    let cmds = [
        "ssh root@example.com",
        "cd /home/www/example.com",
        &branch,
        "npm run build",
        "cd .output/server",
        "npm i node-fetch-native",
        "cd ..",
        "cd ..",
        "pm2 restart node .output/server/index.mjs",
    ];

    for cmd in cmds {
        let result = exe(cmd);

        println!("{:?}", result);
    }
}
