fn main(){
    let mut name = String::from("Hadi");

    // change data in stirng
    name = "Hadiuzzaman".to_string();

    // apend in string text
    name.push_str(" Hadi");

    // apand word
    name.push('!');

    // replace word
    let new_name = name.replace("Hadi!", "Hadi");

    println!("{}", name);
    println!("{}", new_name);
  

    // print every word
    let word = name.split_whitespace();
    println!("{:?}", word);

    for ch in name.split_whitespace(){
        println!("{}", ch);
    }

    // count the sentence length
    println!("Length in bytes: {}", name.len());
    println!("Number of characters : {}", name.chars().count());
}