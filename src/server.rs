pub mod server {

    use dotenv::dotenv;
    use std::env;

    use ssh2::Channel;
    use ssh2::Session;
    use std::io::Read;
    use std::net::TcpStream;

    use crate::COMPANY;

    pub fn connect_and_clean(company: COMPANY) {
        println!("\nConnecting to server...");
        dotenv().ok();

        let hostname: String;
        let username: String;
        let password: String;
        let dir_path: String;

        match company {
            COMPANY::ELECTRICSHUFFLE => {
                println!("Cleaning ES...");
                hostname = env::var("ES_HOSTNAME").expect("ES hostname not found");
                username = env::var("ES_USERNAME").expect("ES username not found");
                password = env::var("ES_PASSWORD").expect("ES password not found");
                dir_path = env::var("ES_DIR_PATH").expect("ES dirPath not found");
            }
            COMPANY::FLIGHTCLUB => {
                println!("Cleaning FC...");
                hostname = env::var("FC_HOSTNAME").expect("FC hostname not found");
                username = env::var("FC_USERNAME").expect("FC username not found");
                password = env::var("FC_PASSWORD").expect("FC password not found");
                dir_path = env::var("FC_DIR_PATH").expect("FC dirPath not found");
            }
            COMPANY::REDENGINE => {
                println!("Cleaning RE...");
                hostname = env::var("RE_HOSTNAME").expect("RE hostname not found");
                username = env::var("RE_USERNAME").expect("RE username not found");
                password = env::var("RE_PASSWORD").expect("RE password not found");
                dir_path = env::var("RE_DIR_PATH").expect("RE dirPath not found");
            }
        };

        let tcp: TcpStream = TcpStream::connect(format!("{}:22", hostname)).unwrap();
        let mut sess: Session = Session::new().unwrap();

        sess.set_tcp_stream(tcp);
        sess.handshake().expect("Failure during TCP handshake");
        sess.userauth_password(username.as_str(), password.as_str())
            .expect("Error in user authentication");

        let mut channel: Channel = sess.channel_session().unwrap();

        let delete_command: String = format!("rm -rf {}", dir_path);
        channel
            .exec(&delete_command)
            .expect("Failed to execute delete command");

        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        println!("Command output: {}", output);

        let mut stderr = String::new();
        channel.stderr().read_to_string(&mut stderr).unwrap();
        if !stderr.is_empty() {
            eprintln!("Command error: {}", stderr);
        }

        channel.send_eof().expect("Failure in channel send_eof");
        channel.wait_close().expect("Error closing channel");
    }
}
