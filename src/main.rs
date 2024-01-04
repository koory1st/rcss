fn main() {
    println!("Hello, world!");

    let id_selector: Selector = Selector::Id(SelectorId::new(".111"));
    dbg!(&id_selector);
}

#[derive(Debug)]
struct SelectorId<'a>(&'a str);

impl <'a> SelectorId<'a> {
    fn new(id: &'a str) -> Self {
        if id.starts_with(&".") {
            panic!("id should not start with '.'");
        }
        Self(id)
    }
}

// https://www.w3.org/TR/2011/REC-CSS2-20110607/syndata.html#keywords

#[derive(Debug)]
enum Selector<'a> {
    Id(SelectorId<'a>),
}

