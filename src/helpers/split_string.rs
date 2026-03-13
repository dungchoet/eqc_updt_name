use crate::constants::DELIMITER;

pub fn split_file_name(file_name: String) -> Vec<String> {
    file_name.split(DELIMITER).map(|s: &str| s.to_string()).collect()
}

pub fn join_part_file_name(part_list: &[String]) -> String {
    part_list.join(DELIMITER)
}