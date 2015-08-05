use std::collections::HashMap;
use std::collections::LinkedList;

fn compress(uncompressed: &str) -> LinkedList<u32> {

    let mut dsize = 256;
    let mut map = HashMap::new();

    for i in 0..dsize {
        let ch = std::char::from_u32(i).unwrap();
        map.insert(format!("{}", ch), i);
    }
         
    let mut w = "".to_string();
    let mut result = LinkedList::new();

    for c in uncompressed.chars() {

        let wc = format!("{}{}", w, c);

        if map.contains_key(&wc) { w = wc;}
        else {

            match map.get(&w) {
                Some(val) => result.push_back(*val),
                None => panic!("Something's going wrong now!"),
            }

            dsize += 1; 
            map.insert(wc, dsize);  
            w = format!("{}", c); 
        }
    }

    if w != "" {

        match map.get(&w) {
            Some(val) => result.push_back(*val),
            None => panic!("Something's going wrong now!"),
        }
    }
    
    result
} 

fn decompress(mut compressed: LinkedList<u32>) -> String {

    let mut dsize = 256; 
    let mut map = HashMap::new();

    for i in 0..dsize {
        let ch = std::char::from_u32(i).unwrap();
        map.insert(i, format!("{}", ch));
    }

    let mut result = "".to_string(); 
    let mut w = "".to_string();

    for j in compressed {

        let mut entry: String;

        if map.contains_key(&j) {
            entry = map.get(&j).unwrap().to_string();
        } else if j == dsize {
            entry = format!("{}{}", w, w.chars()
                                        .next()
                                        .unwrap());
        } else {
            panic!("Bad compressed sequence!");
        }

        result = format!("{}{}", result, entry);
        dsize += 1;
        map.insert(dsize,
                   format!("{}{}", 
                   w, 
                   entry.chars()
                        .next()
                        .unwrap())); 
        
        w = entry;  
    }
    
    result
}

fn main() {
    println!("Hello, world!");
    let mut has = HashMap::new();
    has.insert("abc", 1);
    has.insert(stringify!('a'), 2);
    let mut comp = compress("hello world");
    println!("{:?}", comp);
    let dcomp = decompress(comp);
    println!("{}", dcomp);
    let mut comp2 = compress("hello worldhello world");
    println!("{:?}", comp2);
    let dcomp2 = decompress(comp2);
    println!("{}", dcomp2);
}
