const USER: &str = "admin";
const PASS: &str = "admin";
const HOST: &str = "localhost";
const PORT: u16 = 8086;
const DB: &str = "youtube";

pub fn connect() {
    use influent::create_client;
    use influent::client::{Client, Credentials};
    use influent::measurement::{Measurement, Value};

    let credentials = Credentials {
        username: USER,
        password: PASS,
        database: DB
    };

    let url = format!("http://{}:{}", HOST, PORT);
    let hosts = vec![url];
    create_client(credentials, hosts)
}