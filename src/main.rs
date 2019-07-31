struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
  let mut user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someone@example.com"),
      active: true,
      sign_in_count: 1,
  };

  user1.email = String::from("anotheremail@example.comi");

  println!("User1's email is: {}", user1.email);
}

