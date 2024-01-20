use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
    //
    // let id_selector: Selector = Selector::Id(SelectorId::new(".111"));
    // dbg!(&id_selector);


    let mut set = Selectors::new();
    set.insert("aa");
    set.insert(".bbbb");

    let a:Rule = ("1", "b") ;
}


// https://www.w3.org/TR/2011/REC-CSS2-20110607/syndata.html#keywords


type Selectors<'a> =  HashSet<&'a str>;
type Rule<'a> = (&'a str, &'a str);

struct  Declaration<'a> {
    sub: Declaration<'a>,
    selectors: Selectors<'a>,
    rules:  Vec<Rule<'a>>,
}