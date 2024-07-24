// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    find_and_print_user("xxx");
    find_and_print_user("matt");
}

fn find_and_print_user(name: &str) {
    let user = find_user(name).map(|user_id| User {
        user_id,
        name: name.to_owned(),
    });
    match user {
        None => println!("{name} not found"),
        Some(user) => println!("{:?}", user),
    };
}
