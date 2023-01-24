use std::env::args;
use std::{net::TcpStream, io::Read};
use ssh2::Session;

// fn exe(cmd: &str) -> String {
//     let err = format!("Failed to execute command <{}>", cmd);
//     let output = if cfg!(target_os = "windows") {
//         Command::new("cmd")
//                 .args(["/C", cmd])
//                 .output()
//                 .expect(&err)
//     } else {
//         Command::new("sh")
//                 .arg("-c")
//                 .arg(cmd)
//                 .output()
//                 .expect(&err)
//     };
//     String::from(from_utf8(&output.stdout).unwrap())
// }

fn main() {
    let _args: Vec<String> = args().collect();

    if _args.len() != 2 {
        println!("[Usage]: {} <$branch name>", _args[0]);
        return;
    }

    let branch = format!("git pull origin {}", _args[1]);

    let cmds = [
        "cd /home/www/example.com",
        &branch,
        "npm run build",
        "cd .output/server",
        "npm i node-fetch-native",
        "cd ..",
        "cd ..",
        "pm2 restart node .output/server/index.mjs",
    ];

    // Connect to the local SSH server
    let tcp = TcpStream::connect("site.azbuka-novostroek.com:22").unwrap();

    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);

    sess.handshake().unwrap();
    
    // Try to authenticate with the first identity in the agent.
    sess.userauth_agent("root").unwrap();
    
    // Make sure we succeeded
    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();

    for cmd in cmds {
        channel.exec(cmd).unwrap();

        let mut s = String::new();
    
        channel.read_to_string(&mut s).unwrap();
    
        println!("{}", s);
    }

    channel.wait_close().unwrap();

    println!("{}", channel.exit_status().unwrap());
}
