use std::mem;

fn main() {
    // let s = vec!["undo", "ramon", "soba"];
    // let t = s;
    // let u = s.clone();
    // struct Person {
    //     name: String,
    //     birth: i32,
    // }
    //
    // let mut composers = Vec::new();
    // composers.push(Person {
    //     name: "zeb".to_string(),
    //     birth: 1525,
    // })
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    // let third = v[2];
    // let fifth = v[4];
    let fifth = v.pop().expect("vector empty");
    assert_eq!(fifth, "105");
    struct Person {
        name: Option<String>,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("zeb".to_string()),
        birth: 2003,
    });
    // value cannot move out of index of vec
    // let first_name = composers[0].name;
    let first_name = mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("zeb".to_string()));
    assert_eq!(composers[0].name, None);
    // equals mem::replace
    // let first_name = composers[0].name.take();
    // assert_eq!(first_name, Some("zeb".to_string()));
}
