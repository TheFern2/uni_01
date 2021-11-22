mod tests;

pub struct Item {
    key: String,
    slot: String
}

impl Default for Item {
    fn default() -> Self {
        Item {
            key: "".to_string(),
            slot: "never used".to_string()
        }
    }
}

pub struct HashTable {
    list: Vec<Item>
}

impl HashTable {
    pub fn new(size: usize) -> HashTable {
        let mut new_list = Vec::new();
        HashTable::init(&mut new_list, size);
        HashTable { list: new_list }
    }

    pub fn result(&self) -> String {
        let mut keys: Vec<String> = Vec::new();
        for (_index, item) in self.list.iter().enumerate() {
            // println!("Item: ({}): {} {}", index, item.key, item.slot);
            if item.slot == "occupied" {
                keys.push(item.key.to_string());
            }
        }
        keys.join(" ")
    }

    fn init(list: &mut Vec<Item>, size: usize){
        for _ in 0..size {
            list.push(Item::default());
        }
    }

    fn add(&mut self, key: &String){
        let mut index:u32 = 0;
        let exist = self.find(&key.to_string(), &mut index);
        if exist {
            return;
        }

        let insert_index = self.get_insert_index(&key);
        self.list[insert_index].key = key.to_string();
        self.list[insert_index].slot = "occupied".to_string();
    }

    fn delete(&mut self, key: &String){
        let mut index:u32 = 0;
        let exist = self.find(&key.to_string(), &mut index);
        if !exist {
            return;
        }
        self.list[index as usize].slot = "tombstone".to_string();
    }

    fn get_index(&self, key: &String) -> u32 {
        // Aapple -> e (101) - a(97) -> 4 
        key.chars().last().unwrap() as u32 - 'a' as u32
    }

    fn get_insert_index(&self, key: &String) -> usize {
        let mut index = self.get_index(key);

        loop {
            if self.list[index as usize].slot == "never used" ||
                self.list[index as usize].slot == "tombstone" {
                    break index as usize;
            }
            index = (index + 1) % 26;
            // TODO implement full function for early break
        }
    }

    fn find(&self, key: &String, out_index: &mut u32) -> bool {
        let mut index = self.get_index(key);

        loop {
            if self.list[index as usize].key == *key {
                *out_index = index;
                break true;
            }

            if self.list[index as usize].slot == "never used" ||
                self.list[index as usize].slot == "tombstone" {
                    break false;
            }

            index = (index + 1) % 26;
        }
    }
}

// Things to improve on
// slots: enums
// full_table function
// find function implement Optional argument for out_index
fn main() {
    // Aapple Aorange Dapple Astrawberry
    // orange strawberry
    let mut input = String::new();
    let mut hashtable = HashTable::new(26);

    std::io::stdin().read_line(&mut input)
    .expect("Error: unable to read user input");

    let tokens = input.split_whitespace();
    // TODO validate all tokens are the correct length
    
    for token in tokens {
        if token.len() > 11 {
            println!("Key {} too long {}, must be 10 chars", token, token.len());
        }
        
        // Aapple -> A apple
        let entry_key = &token[1..];
        if token.chars().nth(0) == Some('A'){
            hashtable.add(&entry_key.to_string());
        } else if token.chars().nth(0) == Some('D'){
            hashtable.delete(&entry_key.to_string());
        }
        
    }

    println!("{}", hashtable.result());
}
