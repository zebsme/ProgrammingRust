use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

//fn show(table: Table) {
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    // assert_eq!(table["Gesualdo"][0], "many madrigals");
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    assert!(rx == ry);
    assert!(!std::ptr::eq(rx, ry));
    // 5.3.1 borrowing a local variable
    let r;
    {
        let x = 1;
        r = &x;
    }
    // assert_eq!(*r, 1);

    // 5.3.2 receiving reference as function arguments
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
}

// 5.3.2 receiving reference as function arguments
static mut STASH: &i32 = &128;
fn f<'a>(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
// 5.3.7 omit lifetime parameters
// If there’s only a single lifetime that appears among your function’s parameters, then Rust assumes any lifetimes in your return value must be that one
// fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
// (&point[0], &point[2])
// }

// with all lifetimes written out
// fn first_third<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32)
// struct StringTable {
// elements: Vec<String>,
// }

// impl StringTable {
//     fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
//         for i in 0 .. self.elements.len() {
//             if self.elements[i].starts_with(prefix) {
//                 return Some(&self.elements[i]);
//             }
//         }
//         None
//     }
// }
