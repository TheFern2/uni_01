#[cfg(test)]
mod tests {
    use crate::HashTable;

    fn fixture(input: String, expected: String) {
        let mut hashtable = HashTable::new(26);
        let tokens = input.split_whitespace();
        for token in tokens {
            if token.len() > 11 {
                println!("Key {} too long {}, must be 10 chars", token, token.len());
            }
            let entry = &token[1..];       
            if token.chars().nth(0) == Some('A') {
                hashtable.add(&entry.to_string());
                
            } else if token.chars().nth(0) == Some('D') {
                hashtable.delete(&entry.to_string());
            } 
        }
        assert_eq!(expected, hashtable.result());
    }

    #[test]
    fn test01() {
        fixture("Aaaa Accc Abbb".to_string(), "aaa bbb ccc".to_string())
    }

    #[test]
    fn test02() {
        fixture("Abba Aaaa Acca".to_string(), "bba aaa cca".to_string())
    }

    #[test]
    fn test03() {
        fixture("Abba Aaaa Acca Daaa".to_string(), "bba cca".to_string())
    }

    #[test]
    fn test04() {
        fixture("Aac Abc Dac Acc".to_string(), "cc bc".to_string())
    }

    #[test]
    fn test05() {
         fixture("Aapple Aorange Dapple Astrawberry".to_string(), "orange strawberry".to_string())
    }
}