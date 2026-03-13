pub fn check_prj_code(input: String) -> bool {
    if input.len() != 6 {
        return false;
    }

    let parts: Vec<&str> = input.split('-').collect();
    
    if parts.len() != 2 {
        return false;
    }

    parts[0].len() == 2 && parts[0].chars().all(|c| c.is_ascii_digit()) &&
    parts[1].len() == 3 && parts[1].chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_code() {
        assert!(check_prj_code("12-345".to_string()));
        assert!(check_prj_code("00-000".to_string()));
    }

    #[test]
    fn test_invalid_format() {
        assert!(!check_prj_code("123-45".to_string()));
        assert!(!check_prj_code("-12345".to_string()));
        assert!(!check_prj_code("123456".to_string()));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(!check_prj_code("AB-CDE".to_string())); 
        assert!(!check_prj_code("12-34A".to_string())); 
    }

    #[test]
    fn test_wrong_length() {
        assert!(!check_prj_code("1-345".to_string()));  
        assert!(!check_prj_code("12-3456".to_string())); 
    }
}