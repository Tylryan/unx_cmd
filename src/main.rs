mod unx_cmd;

pub fn main()
{
    let file_contents = String::from("
// Here is a comment
// And another one
Hello there
who are you
Who me
Oh. My name Jeff
Anyway, Later
// Bottom comment
");

    // Remove all comments from a file/string
    let cleaned_contents = remove_comments(file_contents.clone(), "//");
    let append_text      = append_text_to_line(&cleaned_contents);
    let prepend_text     = prepend_text_to_line(&cleaned_contents);

    println!("{cleaned_contents}");
    println!("\n{append_text}");
    println!("\n{prepend_text}");

}

pub fn remove_comments(text: String, comment_type: &str) -> String 
{
    // let f = std::fs::read_to_string("hello.txt").unwrap();
    let regex = String::from(comment_type) + ".*";
    let removed_comments = unx_cmd::sed_g(&text, &regex, "");
    let to_write = removed_comments.trim().to_string();

    return to_write;
}

fn append_text_to_line(text: &String) -> String
{
    let f_lines = text.lines();

    let mut new_lines: Vec<String> = Vec::new();
    for i in f_lines.into_iter()
    {
        if unx_cmd::has_match(i, "^[hHoOaA]")
        {
            let updated = unx_cmd::sed_g(i, i, &format!("{i}."));
            new_lines.push(updated);
        }
        else 
        {
            let updated = unx_cmd::sed_g(i,i, &format!("{i}?"));
            new_lines.push(updated);}
    }
    let updated_string = new_lines.join("\n");
    return updated_string;
}

fn prepend_text_to_line(text: &String) -> String
{
    let f_lines = text.lines();

    let mut new_lines: Vec<String> = Vec::new();
    for i in f_lines.into_iter()
    {
        // Just change this part
        if unx_cmd::has_match(i, "[e]$")
        {
            let updated = unx_cmd::sed_g(i, i, &format!("--> {i}"));
            new_lines.push(updated);
        }
        else 
        {
            let updated = unx_cmd::sed_g(i,i, &format!("{i}?"));
            new_lines.push(updated);}
    }
    let updated_string = new_lines.join("\n");
    return updated_string;
}
