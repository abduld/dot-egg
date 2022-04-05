
use egg::{*, rewrite as rw};

#[derive(Debug)]
struct MyCostFn;
impl CostFunction<SymbolLang> for MyCostFn {
    type Cost = i64;
    fn cost<C>(&mut self, enode: &SymbolLang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost
    {
        -1 * enode.fold(1, |sum, id| sum - costs(id))
    }
}

fn main() { 
    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("reduce-to-seq-reduce"; "(reduce ?f ?x)" => "(par-reduce ?f (seq-reduce ?f (split ?x core-count)))"),
        rw!("reduce-to-vec-reduce"; "(seq-reduce ?f ?x)" => "(seq-reduce0 ?f (vec-reduce ?f (split ?x vec-size)))")
    ];
    let start : RecExpr<SymbolLang> = "(reduce f (lst 1 2 3)))".parse().unwrap(); 
    let mut runner = Runner::default().with_expr(&start).run(rules);
    let extractor = Extractor::new(&runner.egraph, MyCostFn);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
    println!("best_cost = {}, best_expr = {}", best_cost, best_expr);
    // println!("{}", runner.explain_equivalence(&start, &end).get_flat_string());
}
