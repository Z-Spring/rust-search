pub mod github;
pub mod google;
pub mod twitter;

pub fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string == "" {
        return "";
    }
    query_string.split_ascii_whitespace().collect::<Vec<_>>()[0]
}
