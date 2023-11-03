pub fn print_characters(){
    for c in b'A'..=b'z' {
        println!("{}", c as char);
    }
}