fn parse_entry_templ(entry: &String, id: u64) -> Option<String> {
    Some(entry.replace(
        "{ID}",
        &format!("{}", id)
    ))
}

#[get("/post/<id>")]
pub fn post_templ(id: u64) -> Option<String> {
    let entry = "frontend/templates/entry.html";
    match std::fs::read_to_string(entry) {
        Ok(text) => parse_entry_templ(&text, id),
        _ => None
    }
}
