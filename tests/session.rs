use metaverse_login::login::login_with_defaults;
use std::collections::HashMap;
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::{Duration, Instant};

const PYTHON_PORT: u16 = 9000;
const PYTHON_URL: &'static str = "http://127.0.0.1";
const OSGRID_PORT: u16 = 80;
const OSGRID_URL: &'static str = "http://login.osgrid.org";

struct Reap(Child);
impl Drop for Reap {
    fn drop(&mut self) {
        self.0.kill().expect("process already died");
    }
}

#[test]
fn test_mock_session() {
    let mut reaper = match setup() {
        Ok(reap) => reap,
        Err(_string) => return,
    };
    match reaper.0.try_wait().unwrap() {
        None => {}
        Some(status) => {
            panic!("python process unexpectedly exited: {}", status);
        }
    }

    let _login_response = login_with_defaults(
        "first".to_string(),
        "last".to_string(),
        "password".to_string(),
        "last".to_string(),
        true,
        true,
        build_test_url(PYTHON_URL, PYTHON_PORT),
    );

    match reaper.0.try_wait().unwrap() {
        None => {}
        Some(status) => {
            panic!("python process unexpectedly exited: {}", status);
        }
    }
}

#[test]
fn test_osgrid_session() {
    let creds = match read_creds() {
        Some(x) => x,
        None => {
            println!("test skipped, no creds file");
            return;
        }
    };

    let prod_server_url = build_test_url(OSGRID_URL, OSGRID_PORT);
    let _login_response = login_with_defaults(
        creds.get("first").unwrap().to_string(),
        creds.get("last").unwrap().to_string(),
        creds.get("passwd").unwrap().to_string(),
        creds.get("start").unwrap().to_string(),
        true,
        true,
        prod_server_url,
    );
}

fn read_creds() -> Option<HashMap<String, String>> {
    let mut settings = config::Config::default();
    match settings.merge(config::File::with_name(".creds")) {
        Ok(_file) => _file,
        Err(..) => {
            return None;
        }
    };
    settings
        .merge(config::Environment::with_prefix("APP"))
        .unwrap();

    Some(settings.try_into::<HashMap<String, String>>().unwrap())
}

/// helper function for building URL. May be unnescecary
fn build_test_url(url: &str, port: u16) -> String {
    let mut url_string = "".to_owned();
    url_string.push_str(url);
    url_string.push_str(":");
    url_string.push_str(&port.to_string());
    println!("url string {}", url_string);
    return url_string;
}

/// sets up python xmlrpc server for testing
fn setup() -> Result<Reap, String> {
    // logs when server started
    let start = Instant::now();
    // runs the python command to start the test server
    let mut child = match Command::new("python3")
        .arg("tests/test_server/test_server.py")
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("could not start test server, ignoring test({})", e);
            return Err("Could not start test server".to_string());
        }
    };

    // logs how many tries it took to connect to server
    // attempts to connect to python server
    for iteration in 0.. {
        match child.try_wait().unwrap() {
            None => {}
            Some(status) => panic!("python process died {}", status),
        }
        match TcpStream::connect(("localhost", PYTHON_PORT)) {
            Ok(_) => {
                println!(
                    "connected to server after {:?} (iteration{})",
                    Instant::now() - start,
                    iteration
                );
                return Ok(Reap(child));
            }
            Err(_) => {}
        }
        sleep(Duration::from_millis(50));
    }
    return Ok(Reap(child));
}
