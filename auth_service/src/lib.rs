#[allow(dead_code, unused_variables)]
struct Credentials {
    username: String,
    password: String,
}

enum Status {
    Connected,
    Interrupted,
}

fn connect_to_database() -> Status {
    Status::Connected
}

fn get_user() {}

fn login(cred: Credentials) {
    get_user()
}

fn authenticate(cred: Credentials) {
    if let Status::Connected = connect_to_database() {
    };
}
