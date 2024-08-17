#[test]
fn search_query() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec!["safe, fast, productive."],
        minigrep_v2::search(query, contents)
    );
}

#[test]
fn search_query_case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["safe, fast, productive."], 
        minigrep_v2::search_case_insensitive(query, contents)
    );
}