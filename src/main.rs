fn main() {
    let name = String::from("sujal verma");
    let first = get_first_name(name);
    println!("first name : {}", first);
}

fn get_first_name(name: String) -> String {
    let mut ans = String::from("");
    for ch in name.chars() {
        if ch == ' ' {
            break;
        }
        ans.push(ch);
    }
    return ans;
}
