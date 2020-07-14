use crate::math::*;

use indexmap::IndexMap;
use std::str::FromStr;

pub fn mk_rules(tuples: &[(&str, &str, &str)]) -> Vec<Rewrite> {
    tuples
        .iter()
        .map(|(name, left, right)| {
            let left = Pattern::from_str(left).unwrap();
            let right = Pattern::from_str(right).unwrap();
            Rewrite::new(*name, *name, left, right).unwrap()
        })
        .collect()
}

pub fn math_rules() -> IndexMap<&'static str, Vec<Rewrite>> {
    let mut m = IndexMap::new();
    let mut add = |name, rules| {
        if m.contains_key(name) {
            panic!("{} was already there", name);
        }
        m.insert(name, mk_rules(rules));
    };

    add(
        "erf-rules",
        &[
            ("erf-odd", "(erf (neg ?x))", "(neg (erf ?x))"),
            ("erf-erfc", "(erfc ?x)", "(- 1 (erf ?x))"),
            ("erfc-erf", "(erf ?x)", "(- 1 (erfc ?x))"),
        ],
    );
    add("complex-number-basics",
        &[
            ("real-part","(re (complex ?x ?y))","?x"),
            ("imag-part","(im (complex ?x ?y))","?y"),
            ("complex-add-def","(+.c (complex ?a ?b) (complex ?c ?d))","(complex (+ ?a ?c) (+ ?b ?d))"),
            ("complex-def-add","(complex (+ ?a ?c) (+ ?b ?d))","(+.c (complex ?a ?b) (complex ?c ?d))"),
            ("complex-sub-def","(-.c (complex ?a ?b) (complex ?c ?d))","(complex (- ?a ?c) (- ?b ?d))"),
            ("complex-def-sub","(complex (- ?a ?c) (- ?b ?d))","(-.c (complex ?a ?b) (complex ?c ?d))"),
            ("complex-neg-def","(neg.c (complex ?a ?b))","(complex (neg ?a) (neg ?b))"),
            ("complex-def-neg","(complex (neg ?a) (neg ?b))","(neg.c (complex ?a ?b))"),
            ("complex-mul-def","(*.c (complex ?a ?b) (complex ?c ?d))","(complex (- (* ?a ?c) (* ?b ?d)) (+ (* ?a ?d) (* ?b ?c)))"),
            ("complex-div-def","(/.c (complex ?a ?b) (complex ?c ?d))","(complex (/ (+ (* ?a ?c) (* ?b ?d)) (+ (* ?c ?c) (* ?d ?d))) (/ (- (* ?b ?c) (* ?a ?d)) (+ (* ?c ?c) (* ?d ?d))))"),
            ("complex-conj-def","(conj (complex ?a ?b))","(complex ?a (neg ?b))"),
        ],
    );
    add(
        "branch-reduce",
        &[
            ("if-true", "(if TRUE ?x ?y)", "?x"),
            ("if-false", "(if FALSE ?x ?y)", "?y"),
            ("if-same", "(if ?a ?x ?x)", "?x"),
            ("if-not", "(if (not ?a) ?x ?y)", "(if ?a ?y ?x)"),
            (
                "if-if-or",
                "(if ?a ?x (if ?b ?x ?y))",
                "(if (or ?a ?b) ?x ?y)",
            ),
            (
                "if-if-or-not",
                "(if ?a ?x (if ?b ?y ?x))",
                "(if (or ?a (not ?b)) ?x ?y)",
            ),
            (
                "if-if-and",
                "(if ?a (if ?b ?x ?y) ?y)",
                "(if (and ?a ?b) ?x ?y)",
            ),
            (
                "if-if-and-not",
                "(if ?a (if ?b ?y ?x) ?y)",
                "(if (and ?a (not ?b)) ?x ?y)",
            ),
        ],
    );
    add(
        "compare-reduce",
        &[
            ("lt-same", "(< ?x ?x)", "FALSE"),
            ("gt-same", "(> ?x ?x)", "FALSE"),
            ("lte-same", "(<= ?x ?x)", "TRUE"),
            ("gte-same", "(>= ?x ?x)", "TRUE"),
            ("not-lt", "(not (< ?x ?y))", "(>= ?x ?y)"),
            ("not-gt", "(not (> ?x ?y))", "(<= ?x ?y)"),
            ("not-lte", "(not (<= ?x ?y))", "(> ?x ?y)"),
            ("not-gte", "(not (>= ?x ?y))", "(< ?x ?y)"),
        ],
    );
    add(
        "bool-reduce",
        &[
            ("not-true", "(not TRUE)", "FALSE"),
            ("not-false", "(not FALSE)", "TRUE"),
            ("not-not", "(not (not ?a))", "?a"),
            ("not-and", "(not (and ?a ?b))", "(or (not ?a) (not ?b))"),
            ("not-or", "(not (or ?a ?b))", "(and (not ?a) (not ?b))"),
            ("and-true-l", "(and TRUE ?a)", "?a"),
            ("and-true-r", "(and ?a TRUE)", "?a"),
            ("and-false-l", "(and FALSE ?a)", "FALSE"),
            ("and-false-r", "(and ?a FALSE)", "FALSE"),
            ("and-same", "(and ?a ?a)", "?a"),
            ("or-true-l", "(or TRUE ?a)", "TRUE"),
            ("or-true-r", "(or ?a TRUE)", "TRUE"),
            ("or-false-l", "(or FALSE ?a)", "?a"),
            ("or-false-r", "(or ?a FALSE)", "?a"),
            ("or-same", "(or ?a ?a)", "?a"),
        ],
    );
    add(
        "htrig-reduce",
        &[
            ("sinh-def", "(sinh ?x)", "(/ (- (exp ?x) (exp (neg ?x))) 2)"),
            ("cosh-def", "(cosh ?x)", "(/ (+ (exp ?x) (exp (neg ?x))) 2)"),
            (
                "tanh-def1",
                "(tanh ?x)",
                "(/ (- (exp ?x) (exp (neg ?x))) (+ (exp ?x) (exp (neg ?x))))",
            ),
            (
                "tanh-def2",
                "(tanh ?x)",
                "(/ (- (exp (* 2 ?x)) 1) (+ (exp (* 2 ?x)) 1))",
            ),
            (
                "tanh-def3",
                "(tanh ?x)",
                "(/ (- 1 (exp (* -2 ?x))) (+ 1 (exp (* -2 ?x))))",
            ),
            (
                "sinh-cosh",
                "(- (* (cosh ?x) (cosh ?x)) (* (sinh ?x) (sinh ?x)))",
                "1",
            ),
            ("sinh-+-cosh", "(+ (cosh ?x) (sinh ?x))", "(exp ?x)"),
            ("sinh---cosh", "(- (cosh ?x) (sinh ?x))", "(exp (neg ?x))"),
        ],
    );
    add(
        "trig-reduce-fp-sound-nan",
        &[
            ("sin-neg", "(sin (neg ?x))", "(neg (sin ?x))"),
            ("cos-neg", "(cos (neg ?x))", "(cos ?x)"),
            ("tan-neg", "(tan (neg ?x))", "(neg (tan ?x))"),
        ],
    );
    add(
        "trig-reduce-fp-sound",
        &[
            ("sin-0", "(sin 0)", "0"),
            ("cos-0", "(cos 0)", "1"),
            ("tan-0", "(tan 0)", "0"),
        ],
    );
    add(
        "trig-reduce",
        &[
            (
                "cos-sin-sum",
                "(+ (* (cos ?a) (cos ?a)) (* (sin ?a) (sin ?a)))",
                "1",
            ),
            (
                "1-sub-cos",
                "(- 1 (* (cos ?a) (cos ?a)))",
                "(* (sin ?a) (sin ?a))",
            ),
            (
                "1-sub-sin",
                "(- 1 (* (sin ?a) (sin ?a)))",
                "(* (cos ?a) (cos ?a))",
            ),
            (
                "-1-add-cos",
                "(+ (* (cos ?a) (cos ?a)) -1)",
                "(neg (* (sin ?a) (sin ?a)))",
            ),
            (
                "-1-add-sin",
                "(+ (* (sin ?a) (sin ?a)) -1)",
                "(neg (* (cos ?a) (cos ?a)))",
            ),
            (
                "sub-1-cos",
                "(- (* (cos ?a) (cos ?a)) 1)",
                "(neg (* (sin ?a) (sin ?a)))",
            ),
            (
                "sub-1-sin",
                "(- (* (sin ?a) (sin ?a)) 1)",
                "(neg (* (cos ?a) (cos ?a)))",
            ),
            ("sin-PI/6", "(sin (/ PI 6))", "1/2"),
            ("sin-PI/4", "(sin (/ PI 4))", "(/ (sqrt 2) 2)"),
            ("sin-PI/3", "(sin (/ PI 3))", "(/ (sqrt 3) 2)"),
            ("sin-PI/2", "(sin (/ PI 2))", "1"),
            ("sin-PI", "(sin PI)", "0"),
            ("sin-+PI", "(sin (+ ?x PI))", "(neg (sin ?x))"),
            ("sin-+PI/2", "(sin (+ ?x (/ PI 2)))", "(cos ?x)"),
            ("cos-PI/6", "(cos (/ PI 6))", "(/ (sqrt 3) 2)"),
            ("cos-PI/4", "(cos (/ PI 4))", "(/ (sqrt 2) 2)"),
            ("cos-PI/3", "(cos (/ PI 3))", "1/2"),
            ("cos-PI/2", "(cos (/ PI 2))", "0"),
            ("cos-PI", "(cos PI)", "-1"),
            ("cos-+PI", "(cos (+ ?x PI))", "(neg (cos ?x))"),
            ("cos-+PI/2", "(cos (+ ?x (/ PI 2)))", "(neg (sin ?x))"),
            ("tan-PI/6", "(tan (/ PI 6))", "(/ 1 (sqrt 3))"),
            ("tan-PI/4", "(tan (/ PI 4))", "1"),
            ("tan-PI/3", "(tan (/ PI 3))", "(sqrt 3)"),
            ("tan-PI", "(tan PI)", "0"),
            ("tan-+PI", "(tan (+ ?x PI))", "(tan ?x)"),
            ("tan-+PI/2", "(tan (+ ?x (/ PI 2)))", "(neg (/ 1 (tan ?x)))"),
            (
                "hang-0p-tan",
                "(/ (sin ?a) (+ 1 (cos ?a)))",
                "(tan (/ ?a 2))",
            ),
            (
                "hang-0m-tan",
                "(/ (neg (sin ?a)) (+ 1 (cos ?a)))",
                "(tan (/ (neg ?a) 2))",
            ),
            (
                "hang-p0-tan",
                "(/ (- 1 (cos ?a)) (sin ?a))",
                "(tan (/ ?a 2))",
            ),
            (
                "hang-m0-tan",
                "(/ (- 1 (cos ?a)) (neg (sin ?a)))",
                "(tan (/ (neg ?a) 2))",
            ),
            (
                "hang-p-tan",
                "(/ (+ (sin ?a) (sin ?b)) (+ (cos ?a) (cos ?b)))",
                "(tan (/ (+ ?a ?b) 2))",
            ),
            (
                "hang-m-tan",
                "(/ (- (sin ?a) (sin ?b)) (+ (cos ?a) (cos ?b)))",
                "(tan (/ (- ?a ?b) 2))",
            ),
        ],
    );
    add("log-distribute-fp-safe", &[("log-E", "(log E)", "1")]);
    add(
        "log-distribute",
        &[
            ("log-prod", "(log (* ?a ?b))", "(+ (log ?a) (log ?b))"),
            ("log-div", "(log (/ ?a ?b))", "(- (log ?a) (log ?b))"),
            ("log-rec", "(log (/ 1 ?a))", "(neg (log ?a))"),
            ("log-pow", "(log (pow ?a ?b))", "(* ?b (log ?a))"),
        ],
    );
    add(
        "pow-canonicalize",
        &[
            ("exp-to-pow", "(exp (* (log ?a) ?b))", "(pow ?a ?b)"),
            ("pow-plus", "(* (pow ?a ?b) ?a)", "(pow ?a (+ ?b 1))"),
            ("unpow1/2", "(pow ?a 1/2)", "(sqrt ?a)"),
            ("unpow2", "(pow ?a 2)", "(* ?a ?a)"),
            ("unpow3", "(pow ?a 3)", "(* (* ?a ?a) ?a)"),
            ("unpow1/3", "(pow ?a 1/3)", "(cbrt ?a)"),
        ],
    );
    add(
        "pow-reduce-fp-safe-nan",
        &[
            ("unpow0", "(pow ?a 0)", "1"),
            ("pow-base-1", "(pow 1 ?a)", "1"),
        ],
    );
    add("pow-reduce-fp-safe", &[("unpow1", "(pow ?a 1)", "?a")]);
    add("pow-reduce", &[("unpow-1", "(pow ?a -1)", "(/ 1 ?a)")]);
    add(
        "exp-factor",
        &[
            ("prod-exp", "(* (exp ?a) (exp ?b))", "(exp (+ ?a ?b))"),
            ("rec-exp", "(/ 1 (exp ?a))", "(exp (neg ?a))"),
            ("div-exp", "(/ (exp ?a) (exp ?b))", "(exp (- ?a ?b))"),
            ("exp-prod", "(exp (* ?a ?b))", "(pow (exp ?a) ?b)"),
            ("exp-sqrt", "(exp (/ ?a 2))", "(sqrt (exp ?a))"),
            ("exp-cbrt", "(exp (/ ?a 3))", "(cbrt (exp ?a))"),
            ("exp-lft-sqr", "(exp (* ?a 2))", "(* (exp ?a) (exp ?a))"),
            ("exp-lft-cube", "(exp (* ?a 3))", "(pow (exp ?a) 3)"),
        ],
    );
    add(
        "exp-distribute",
        &[
            ("exp-sum", "(exp (+ ?a ?b))", "(* (exp ?a) (exp ?b))"),
            ("exp-neg", "(exp (neg ?a))", "(/ 1 (exp ?a))"),
            ("exp-diff", "(exp (- ?a ?b))", "(/ (exp ?a) (exp ?b))"),
        ],
    );
    add(
        "exp-constants",
        &[
            ("exp-0", "(exp 0)", "1"),
            ("exp-1-e", "(exp 1)", "E"),
            ("1-exp", "1", "(exp 0)"),
            ("e-exp-1", "E", "(exp 1)"),
        ],
    );
    add(
        "exp-reduce",
        &[
            ("rem-exp-log", "(exp (log ?x))", "?x"),
            ("rem-log-exp", "(log (exp ?x))", "?x"),
        ],
    );
    add(
        "cubes-canonicalize",
        &[("cube-unmult", "(* ?x (* ?x ?x))", "(pow ?x 3)")],
    );
    add(
        "cubes-distribute",
        &[
            (
                "cube-prod",
                "(pow (* ?x ?y) 3)",
                "(* (pow ?x 3) (pow ?y 3))",
            ),
            ("cube-div", "(pow (/ ?x ?y) 3)", "(/ (pow ?x 3) (pow ?y 3))"),
            ("cube-mult", "(pow ?x 3)", "(* ?x (* ?x ?x))"),
        ],
    );
    add(
        "cubes-reduce",
        &[
            ("rem-cube-cbrt", "(pow (cbrt ?x) 3)", "?x"),
            ("rem-cbrt-cube", "(cbrt (pow ?x 3))", "?x"),
            ("cube-neg", "(pow (neg ?x) 3)", "(neg (pow ?x 3))"),
        ],
    );
    add(
        "squares-reduce-fp-sound",
        &[("sqr-neg", "(* (neg ?x) (neg ?x))", "(* ?x ?x)")],
    );
    add(
        "squares-reduce",
        &[
            ("rem-square-sqrt", "(* (sqrt ?x) (sqrt ?x))", "?x"),
            ("rem-sqrt-square", "(sqrt (* ?x ?x))", "(fabs ?x)"),
        ],
    );
    add(
        "fractions-distribute.c",
        &[
            (
                "div-sub.c",
                "(/.c (-.c ?a ?b) ?c)",
                "(-.c (/.c ?a ?c) (/.c ?b ?c))",
            ),
            (
                "times-frac.c",
                "(/.c (*.c ?a ?b) (*.c ?c ?d))",
                "(*.c (/.c ?a ?c) (/.c ?b ?d))",
            ),
        ],
    );
    add(
        "fractions-distribute",
        &[
            ("div-sub", "(/ (- ?a ?b) ?c)", "(- (/ ?a ?c) (/ ?b ?c))"),
            (
                "times-frac",
                "(/ (* ?a ?b) (* ?c ?d))",
                "(* (/ ?a ?c) (/ ?b ?d))",
            ),
        ],
    );
    add(
        "id-reduce-fp-safe",
        &[
            ("+-lft-identity", "(+ 0 ?a)", "?a"),
            ("+-rgt-identity", "(+ ?a 0)", "?a"),
            ("--rgt-identity", "(- ?a 0)", "?a"),
            ("sub0-neg", "(- 0 ?a)", "(neg ?a)"),
            ("remove-double-neg", "(neg (neg ?a))", "?a"),
            ("*-lft-identity", "(* 1 ?a)", "?a"),
            ("*-rgt-identity", "(* ?a 1)", "?a"),
            ("/-rgt-identity", "(/ ?a 1)", "?a"),
            ("mul-1-neg", "(* -1 ?a)", "(neg ?a)"),
        ],
    );
    add(
        "id-reduce-fp-safe-nan",
        &[
            ("+-inverses", "(- ?a ?a)", "0"),
            ("*-inverses", "(/ ?a ?a)", "1"),
            ("div0", "(/ 0 ?a)", "0"),
            ("mul0l", "(* 0 ?a)", "0"),
            ("mul0r", "(* ?a 0)", "0"),
        ],
    );
    add(
        "id-reduce",
        &[
            ("remove-double-div", "(/ 1 (/ 1 ?a))", "?a"),
            ("rgt-mult-inverse", "(* ?a (/ 1 ?a))", "1"),
            ("lft-mult-inverse", "(* (/ 1 ?a) ?a)", "1"),
        ],
    );
    add(
        "difference-of-squares-canonicalize",
        &[
            (
                "swap-sqr",
                "(* (* ?a ?b) (* ?a ?b))",
                "(* (* ?a ?a) (* ?b ?b))",
            ),
            (
                "unswap-sqr",
                "(* (* ?a ?a) (* ?b ?b))",
                "(* (* ?a ?b) (* ?a ?b))",
            ),
            (
                "difference-of-squares",
                "(- (* ?a ?a) (* ?b ?b))",
                "(* (+ ?a ?b) (- ?a ?b))",
            ),
            (
                "difference-of-sqr-1",
                "(- (* ?a ?a) 1)",
                "(* (+ ?a 1) (- ?a 1))",
            ),
            (
                "difference-of-sqr--1",
                "(+ (* ?a ?a) -1)",
                "(* (+ ?a 1) (- ?a 1))",
            ),
            (
                "sqr-pow",
                "(pow ?a ?b)",
                "(* (pow ?a (/ ?b 2)) (pow ?a (/ ?b 2)))",
            ),
            (
                "pow-sqr",
                "(* (pow ?a ?b) (pow ?a ?b))",
                "(pow ?a (* 2 ?b))",
            ),
        ],
    );
    add(
        "distributivity-fp-safe",
        &[
            (
                "distribute-lft-neg-in",
                "(neg (* ?a ?b))",
                "(* (neg ?a) ?b)",
            ),
            (
                "distribute-rgt-neg-in",
                "(neg (* ?a ?b))",
                "(* ?a (neg ?b))",
            ),
            (
                "distribute-lft-neg-out",
                "(* (neg ?a) ?b)",
                "(neg (* ?a ?b))",
            ),
            (
                "distribute-rgt-neg-out",
                "(* ?a (neg ?b))",
                "(neg (* ?a ?b))",
            ),
            (
                "distribute-neg-in",
                "(neg (+ ?a ?b))",
                "(+ (neg ?a) (neg ?b))",
            ),
            (
                "distribute-neg-out",
                "(+ (neg ?a) (neg ?b))",
                "(neg (+ ?a ?b))",
            ),
            ("distribute-frac-neg", "(/ (neg ?a) ?b)", "(neg (/ ?a ?b))"),
            ("distribute-neg-frac", "(neg (/ ?a ?b))", "(/ (neg ?a) ?b)"),
        ],
    );
    add(
        "distributivity.c",
        &[
            (
                "distribute-lft-in.c",
                "(*.c ?a (+.c ?b ?c))",
                "(+.c (*.c ?a ?b) (*.c ?a ?c))",
            ),
            (
                "distribute-rgt-in.c",
                "(*.c ?a (+.c ?b ?c))",
                "(+.c (*.c ?b ?a) (*.c ?c ?a))",
            ),
            (
                "distribute-lft-out.c",
                "(+.c (*.c ?a ?b) (*.c ?a ?c))",
                "(*.c ?a (+.c ?b ?c))",
            ),
            (
                "distribute-lft-out--.c",
                "(-.c (*.c ?a ?b) (*.c ?a ?c))",
                "(*.c ?a (-.c ?b ?c))",
            ),
            (
                "distribute-rgt-out.c",
                "(+.c (*.c ?b ?a) (*.c ?c ?a))",
                "(*.c ?a (+.c ?b ?c))",
            ),
            (
                "distribute-rgt-out--.c",
                "(-.c (*.c ?b ?a) (*.c ?c ?a))",
                "(*.c ?a (-.c ?b ?c))",
            ),
            (
                "distribute-lft1-in.c",
                "(+.c (*.c ?b ?a) ?a)",
                "(*.c (+.c ?b (complex 1 0)) ?a)",
            ),
            (
                "distribute-rgt1-in.c",
                "(+.c ?a (*.c ?c ?a))",
                "(*.c (+.c ?c (complex 1 0)) ?a)",
            ),
        ],
    );
    add(
        "distributivity",
        &[
            (
                "distribute-lft-in",
                "(* ?a (+ ?b ?c))",
                "(+ (* ?a ?b) (* ?a ?c))",
            ),
            (
                "distribute-rgt-in",
                "(* ?a (+ ?b ?c))",
                "(+ (* ?b ?a) (* ?c ?a))",
            ),
            (
                "distribute-lft-out",
                "(+ (* ?a ?b) (* ?a ?c))",
                "(* ?a (+ ?b ?c))",
            ),
            (
                "distribute-lft-out--",
                "(- (* ?a ?b) (* ?a ?c))",
                "(* ?a (- ?b ?c))",
            ),
            (
                "distribute-rgt-out",
                "(+ (* ?b ?a) (* ?c ?a))",
                "(* ?a (+ ?b ?c))",
            ),
            (
                "distribute-rgt-out--",
                "(- (* ?b ?a) (* ?c ?a))",
                "(* ?a (- ?b ?c))",
            ),
            ("distribute-lft1-in", "(+ (* ?b ?a) ?a)", "(* (+ ?b 1) ?a)"),
            ("distribute-rgt1-in", "(+ ?a (* ?c ?a))", "(* (+ ?c 1) ?a)"),
        ],
    );
    add("counting", &[("count-2", "(+ ?x ?x)", "(* 2 ?x)")]);
    add(
        "associativity.c",
        &[
            (
                "associate-+r+.c",
                "(+.c ?a (+.c ?b ?c))",
                "(+.c (+.c ?a ?b) ?c)",
            ),
            (
                "associate-+l+.c",
                "(+.c (+.c ?a ?b) ?c)",
                "(+.c ?a (+.c ?b ?c))",
            ),
            (
                "associate-+r-.c",
                "(+.c ?a (-.c ?b ?c))",
                "(-.c (+.c ?a ?b) ?c)",
            ),
            (
                "associate-+l-.c",
                "(+.c (-.c ?a ?b) ?c)",
                "(-.c ?a (-.c ?b ?c))",
            ),
            (
                "associate--r+.c",
                "(-.c ?a (+.c ?b ?c))",
                "(-.c (-.c ?a ?b) ?c)",
            ),
            (
                "associate--l+.c",
                "(-.c (+.c ?a ?b) ?c)",
                "(+.c ?a (-.c ?b ?c))",
            ),
            (
                "associate--l-.c",
                "(-.c (-.c ?a ?b) ?c)",
                "(-.c ?a (+.c ?b ?c))",
            ),
            (
                "associate--r-.c",
                "(-.c ?a (-.c ?b ?c))",
                "(+.c (-.c ?a ?b) ?c)",
            ),
            (
                "associate-*r*.c",
                "(*.c ?a (*.c ?b ?c))",
                "(*.c (*.c ?a ?b) ?c)",
            ),
            (
                "associate-*l*.c",
                "(*.c (*.c ?a ?b) ?c)",
                "(*.c ?a (*.c ?b ?c))",
            ),
            (
                "associate-*r/.c",
                "(*.c ?a (/.c ?b ?c))",
                "(/.c (*.c ?a ?b) ?c)",
            ),
            (
                "associate-*l/.c",
                "(*.c (/.c ?a ?b) ?c)",
                "(/.c (*.c ?a ?c) ?b)",
            ),
            (
                "associate-/r*.c",
                "(/.c ?a (*.c ?b ?c))",
                "(/.c (/.c ?a ?b) ?c)",
            ),
            (
                "associate-/l*.c",
                "(/.c (*.c ?b ?c) ?a)",
                "(/.c ?b (/.c ?a ?c))",
            ),
            (
                "associate-/r/.c",
                "(/.c ?a (/.c ?b ?c))",
                "(*.c (/.c ?a ?b) ?c)",
            ),
            (
                "associate-/l/.c",
                "(/.c (/.c ?b ?c) ?a)",
                "(/.c ?b (*.c ?a ?c))",
            ),
            ("sub-neg.c", "(-.c ?a ?b)", "(+.c ?a (neg.c ?b))"),
            ("unsub-neg.c", "(+.c ?a (neg.c ?b))", "(-.c ?a ?b)"),
        ],
    );
    add(
        "associativity",
        &[
            ("associate-+r+", "(+ ?a (+ ?b ?c))", "(+ (+ ?a ?b) ?c)"),
            ("associate-+l+", "(+ (+ ?a ?b) ?c)", "(+ ?a (+ ?b ?c))"),
            ("associate-+r-", "(+ ?a (- ?b ?c))", "(- (+ ?a ?b) ?c)"),
            ("associate-+l-", "(+ (- ?a ?b) ?c)", "(- ?a (- ?b ?c))"),
            ("associate--r+", "(- ?a (+ ?b ?c))", "(- (- ?a ?b) ?c)"),
            ("associate--l+", "(- (+ ?a ?b) ?c)", "(+ ?a (- ?b ?c))"),
            ("associate--l-", "(- (- ?a ?b) ?c)", "(- ?a (+ ?b ?c))"),
            ("associate--r-", "(- ?a (- ?b ?c))", "(+ (- ?a ?b) ?c)"),
            ("associate-*r*", "(* ?a (* ?b ?c))", "(* (* ?a ?b) ?c)"),
            ("associate-*l*", "(* (* ?a ?b) ?c)", "(* ?a (* ?b ?c))"),
            ("associate-*r/", "(* ?a (/ ?b ?c))", "(/ (* ?a ?b) ?c)"),
            ("associate-*l/", "(* (/ ?a ?b) ?c)", "(/ (* ?a ?c) ?b)"),
            ("associate-/r*", "(/ ?a (* ?b ?c))", "(/ (/ ?a ?b) ?c)"),
            ("associate-/l*", "(/ (* ?b ?c) ?a)", "(/ ?b (/ ?a ?c))"),
            ("associate-/r/", "(/ ?a (/ ?b ?c))", "(* (/ ?a ?b) ?c)"),
            ("associate-/l/", "(/ (/ ?b ?c) ?a)", "(/ ?b (* ?a ?c))"),
            ("sub-neg", "(- ?a ?b)", "(+ ?a (neg ?b))"),
            ("unsub-neg", "(+ ?a (neg ?b))", "(- ?a ?b)"),
        ],
    );
    add(
        "commutativity.c",
        &[
            ("+.c-commutative", "(+.c ?a ?b)", "(+.c ?b ?a)"),
            ("*.c-commutative", "(*.c ?a ?b)", "(*.c ?b ?a)"),
        ],
    );
    add(
        "commutativity",
        &[
            ("+-commutative", "(+ ?a ?b)", "(+ ?b ?a)"),
            ("*-commutative", "(* ?a ?b)", "(* ?b ?a)"),
        ],
    );

    m
}
