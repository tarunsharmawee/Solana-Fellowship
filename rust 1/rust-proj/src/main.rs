fn main() {
    let sentence: String = String::from("Baghi Firse Lotega!");
    let first_word: String = get_first_word(sentence);
    println!("First Word of The Sentence is {}",  first_word);
}
fn get_first_word(sentence: String) -> String{
    let mut ans = String::new();
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans
}