use egg::{*, rewrite as rw};
let rules: &[Rewrite<SymbolLang, ()>] = &[
    rw!("reduce-as-reduce-reduce"; "(reduce f ?x)" => "(reduce (reduce f (split ?x ?k)))")
];


fn main() {
    println!("Hello, world!");
}
