fn main() {
    let user = User::build_user(
        String::from("SEXY LADY"), 
        String::from("sxy@ldy.com"),
    );
    let ok = user.sign_in_count_valid();
    println!("{user:#?}\n\n{ok}"); 
}

#[derive(Debug)]
struct User {
    _active: bool,
    _username: String,
    _email: String,
    _sign_in_count: u64,
}

impl User {
    fn sign_in_count_valid(&self) -> bool {
        return self._sign_in_count < 12;
    }
    fn build_user(email: String, username: String) -> Self {
        Self {
            _active: true,
            _username: username,
            _email: email,
            _sign_in_count: 1,
        }
    }
}

