pub mod server {

    use dotenv::dotenv;
    use std::env;

    use ssh2::Channel;
    use ssh2::Session;
    use std::io::Read;
    use std::net::TcpStream;

    use crate::COMPANY;

    pub fn connect_and_clean(company: COMPANY) {
        dotenv().ok();

        let mut us_dir_path: String = String::new();
        let hostname: String;
        let username: String;
        let password: String;
        let dir_path: String;

        match company {
            COMPANY::ELECTRICSHUFFLE => {
                println!("\nConnecting to ES server...");
                println!("Cleaning ES...");
                hostname = env::var("ES_HOSTNAME").expect("ES hostname not found");
                username = env::var("ES_USERNAME").expect("ES username not found");
                password = env::var("ES_PASSWORD").expect("ES password not found");
                dir_path = env::var("ES_UK_DIR_PATH").expect("ES UK dirPath not found");
                us_dir_path = env::var("ES_US_DIR_PATH").expect("ES US dirPath not found");
            }
            COMPANY::FLIGHTCLUB => {
                println!("\nConnecting to FC server...");
                println!("Cleaning FC...");
                hostname = env::var("FC_HOSTNAME").expect("FC hostname not found");
                username = env::var("FC_USERNAME").expect("FC username not found");
                password = env::var("FC_PASSWORD").expect("FC password not found");
                dir_path = env::var("FC_DIR_PATH").expect("FC dirPath not found");
            }
            COMPANY::REDENGINE => {
                println!("\nConnecting to RE server...");
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

        let mut stderr = String::new();
        channel.stderr().read_to_string(&mut stderr).unwrap();
        if !stderr.is_empty() {
            eprintln!("Command error: {}", stderr);
        }

        if !us_dir_path.is_empty() {
            // US

            let mut us_channel: Channel = sess.channel_session().unwrap();

            let us_delete_command: String = format!("rm -rf {}", us_dir_path);
            us_channel
                .exec(&us_delete_command)
                .expect("Failed to execute delete command");

            let mut us_output = String::new();
            us_channel.read_to_string(&mut us_output).unwrap();

            let mut us_stderr = String::new();
            us_channel.stderr().read_to_string(&mut us_stderr).unwrap();
            if !stderr.is_empty() {
                eprintln!("US Command error: {}", us_stderr);
            }

            us_channel
                .send_eof()
                .expect("Failure in US channel send_eof");
            us_channel.wait_close().expect("Error closing US channel");
        }

        channel.send_eof().expect("Failure in channel send_eof");
        channel.wait_close().expect("Error closing channel");
    }
}
