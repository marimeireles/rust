// This test case tests the incremental compilation hash (ICH) implementation
// for function and method call expressions.

// The general pattern followed here is: Change one thing between rev1 and rev2
// and make sure that the hash has changed, then change nothing between rev2 and
// rev3 and make sure that the hash has not changed.

// compile-pass
// revisions: cfail1 cfail2 cfail3
// compile-flags: -Z query-dep-graph -Zincremental-ignore-spans


#![allow(warnings)]
#![feature(rustc_attrs)]
#![crate_type="rlib"]

fn callee1(_x: u32, _y: i64) {}
fn callee2(_x: u32, _y: i64) {}


// Change Callee (Function) ----------------------------------------------------
#[cfg(cfail1)]
pub fn change_callee_function() {
    callee1(1, 2)
}

#[cfg(not(cfail1))]
#[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir,typeck_tables_of")]
#[rustc_clean(cfg="cfail3")]
pub fn change_callee_function() {
    callee2(1, 2)
}



// Change Argument (Function) --------------------------------------------------
#[cfg(cfail1)]
pub fn change_argument_function() {
    callee1(1, 2)
}

#[cfg(not(cfail1))]
#[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir")]
#[rustc_clean(cfg="cfail3")]
pub fn change_argument_function() {
    callee1(1, 3)
}



// Change Callee Indirectly (Function) -----------------------------------------
mod change_callee_indirectly_function {
    #[cfg(cfail1)]
    use super::callee1 as callee;
    #[cfg(not(cfail1))]
    use super::callee2 as callee;

    #[rustc_clean(label="Hir", cfg="cfail2")]
    #[rustc_clean(label="Hir", cfg="cfail3")]
    #[rustc_dirty(label="HirBody", cfg="cfail2")]
    #[rustc_clean(label="HirBody", cfg="cfail3")]


    pub fn change_callee_indirectly_function() {
        callee(1, 2)
    }
}


struct Struct;
impl Struct {
    fn method1(&self, _x: char, _y: bool) {}
    fn method2(&self, _x: char, _y: bool) {}
}

// Change Callee (Method) ------------------------------------------------------
#[cfg(cfail1)]
pub fn change_callee_method() {
    let s = Struct;
    s.method1('x', true);
}

#[cfg(not(cfail1))]
#[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir,typeck_tables_of")]
#[rustc_clean(cfg="cfail3")]
pub fn change_callee_method() {
    let s = Struct;
    s.method2('x', true);
}



// Change Argument (Method) ----------------------------------------------------
#[cfg(cfail1)]
pub fn change_argument_method() {
    let s = Struct;
    s.method1('x', true);
}

#[cfg(not(cfail1))]
#[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir")]
#[rustc_clean(cfg="cfail3")]
pub fn change_argument_method() {
    let s = Struct;
    s.method1('y', true);
}



// Change Callee (Method, UFCS) ------------------------------------------------
#[cfg(cfail1)]
pub fn change_ufcs_callee_method() {
    let s = Struct;
    Struct::method1(&s, 'x', true);
}

#[cfg(not(cfail1))]
#[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir,typeck_tables_of")]
#[rustc_clean(cfg="cfail3")]
pub fn change_ufcs_callee_method() {
    let s = Struct;
    Struct::method2(&s, 'x', true);
}



// Change Argument (Method, UFCS) ----------------------------------------------
#[cfg(cfail1)]
pub fn change_argument_method_ufcs() {
    let s = Struct;
    Struct::method1(&s, 'x', true);
}

#[cfg(not(cfail1))]
#[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir")]
#[rustc_clean(cfg="cfail3")]
pub fn change_argument_method_ufcs() {
    let s = Struct;
    Struct::method1(&s, 'x', false);
}



// Change To UFCS --------------------------------------------------------------
#[cfg(cfail1)]
pub fn change_to_ufcs() {
    let s = Struct;
    s.method1('x', true);
}

#[cfg(not(cfail1))]
#[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir,typeck_tables_of")]
#[rustc_clean(cfg="cfail3")]
// One might think this would be expanded in the {hir,mir}::Body, but it actually
// results in slightly different {hir,mir}::Body.
pub fn change_to_ufcs() {
    let s = Struct;
    Struct::method1(&s, 'x', true);
}


struct Struct2;
impl Struct2 {
    fn method1(&self, _x: char, _y: bool) {}
}

// Change UFCS Callee Indirectly -----------------------------------------------
pub mod change_ufcs_callee_indirectly {
    #[cfg(cfail1)]
    use super::Struct as Struct;
    #[cfg(not(cfail1))]
    use super::Struct2 as Struct;

    #[rustc_clean(cfg="cfail2", except="HirBody,mir_built,optimized_mir,typeck_tables_of")]
    #[rustc_clean(cfg="cfail3")]


    pub fn change_ufcs_callee_indirectly() {
        let s = Struct;
        Struct::method1(&s, 'q', false)
    }
}
