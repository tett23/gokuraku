use crate::ir::DataTerm;

trait SemiLattice: PartialOrd {
    type Elem;

    fn is_disjoint(&self, other: &Self) -> bool {
        self.partial_cmp(other) == None
    }

    fn union(self, other: &Self) -> Self;
}

trait JoinSemiLattice: SemiLattice {
    fn join(&self, other: &Self) -> Self;
}

#[derive(Debug, PartialEq)]
struct DataJoinSemiLattice {}

impl PartialOrd for DataJoinSemiLattice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unimplemented!()
    }
}

impl SemiLattice for DataJoinSemiLattice {
    type Elem = DataTerm;

    fn union(self, other: &Self) -> Self {
        unimplemented!()
    }
}

impl JoinSemiLattice for DataJoinSemiLattice {
    fn join(&self, other: &Self) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prose_down_script_parse;

    #[test]
    fn test_join() {
        // let ast1 = prose_down_script_parse("nominal True = ()").unwrap();
        // let ast2 = prose_down_script_parse("nominal True = ()").unwrap();
        // let ir1 = super::super::super::ir1::transform1(&ast1);
        // let ir2 = super::super::super::ir2::transform2(ir1, ast1);

        // let a = DataJoinSemiLattice::try_from(ast1).unwrap();
        // let b = DataJoinSemiLattice::new();

        // let c = a.join(&b);

        // assert_eq!(c, DataLattice::new());
    }
}
