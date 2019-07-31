struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
  let mut user1 = build_user(String::from("someone@example.com"), String::from("sum"));

  user1.email = String::from("anotheremail@example.com");

  println!("User1's email is: {}", user1.email);
}

fn build_user(email: String, username: String) -> User {
  User {
      email,
      username,
      active: true,
      sign_in_count: 1,
  }
}

