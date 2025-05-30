

fn main(){
    let role = "admin";

    match role {
        "admin" => println!("You are wellcome in Admin panel"),
        "user" => println!("you are wellcome in dashboard"),
        _ => println!("Wellcome Mr."),
    }
}