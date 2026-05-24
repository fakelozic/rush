// pub fn use_quotes(test_str: &str) -> String {
//     // let test_str = "     'hello    world'    ";
//     let trimmed_str = test_str.trim();
//     // let useful_str = test_str.trim().trim_matches('\'');
//     let useful_str: Vec<&str> = trimmed_str.split("'").collect();
//     let mut arg: Vec<&str> = Vec::new();
//     for word in useful_str {
//         if word.is_empty() || word.trim().is_empty() {
//             continue;
//         }
//         arg.push(word);
//     }
//     arg.concat()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn use_quotes_test() {
//         assert_eq!(use_quotes("'hello world'"), String::from("hello world"));
//         assert_eq!(
//             use_quotes("    'hello world'    "),
//             String::from("hello world")
//         );

//         assert_eq!(use_quotes("'hello' 'world'"), String::from("helloworld"));
//         assert_eq!(use_quotes("hello''world"), String::from("helloworld"));
//     }
// }
