extern crate clap;
extern crate reqwest;
extern crate serde_json;

mod cli;

fn main() {
    let matches =
        cli::get_arguments().get_matches();

//    let args: Vec<String> = std::env::args().collect();
//    let mut options = getopts::Options::new();
//    options.optflag("a", "anonymous-request", "set the petition to be anonymous");
//    options.reqopt("i", "input", "string to encrypt", "string");
//    options.optopt("u", "username", "username to login", "username");
//    options.optopt("p", "password", "password to login", "password");
//    options.reqopt("s", "server", "server to connect", "URL");
//    let matches = match options.parse(&args[1..]) {
//        Ok(m) => { m }
//        Err(_) => {
//            println!("{}", options.short_usage("gocd-encrypt"));
//            std::process::exit(1);
//        }
//    };
    let string_to_encrypt = matches.value_of("INPUT").unwrap();
    let server = matches.value_of("SERVER").unwrap().to_owned() + "/go/api/admin/encrypt";
    let mut request = reqwest::blocking::Client::new()
        .post(&server)
        .header(reqwest::header::ACCEPT, "application/vnd.go.cd.v1+json")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&serde_json::json!({
            "value": string_to_encrypt
        }));
    if !matches.is_present("ANONYMOUS") {
        let username = matches.value_of("username").unwrap();
        let password: String;
        if matches.is_present("password") {
            password = String::from(matches.value_of("password").unwrap());
        } else {
            password = rpassword::prompt_password_stderr("Password: ").unwrap();
        };
        request = request.basic_auth(username.clone(), Some(password.clone()));
    }
    let echo_json: serde_json::Value = request.send().unwrap().json().unwrap();
    if echo_json.get("encrypted_value") == None {
        eprintln!("Received response is invalid");
        std::process::exit(1);
    }
    else {
        println!("{}", echo_json.get("encrypted_value").unwrap().as_str().unwrap());
    }

}