use criterion::{black_box, criterion_group, criterion_main, Criterion};
use teolang::program::parser::Ast;

fn benchmark_parser(c: &mut Criterion) {
    // Basic input
    let input = r#"
        def i(a: Number){
          print(a + 3 * 4);
        };
        n = 3;
        n = 2;
        i(n);
        x = [5, 7, 3];
        a = x[1] + 2;
        x[1] = 3;
        print(a);
        print(x[1]);
        if (x[1] == 3) {
           x[1] = 4;
        };
        lbozo = x[1] > 3;
        print(lbozo);
        def a(n: Bool){
            return(n);
        };
        print("Should get called after here");
        l = a(true);
        print(l);
        print(3 + 4 * 2 - 6 / 3 * 2);
        input();
        x = inputf("%Number %String");
        print(x[0]);
        print(x[1]);
        n = split(input(), "1234");
        x = 0;
        for i in n {
            print(i);
            x = x + 1;
        };
        print(x);
        print("Fuck");
        print(n[0]);
        def custom_pow(base: Number, power: Number) {
            if (power <= 1) {
                return(base);
            };
            if (power > 1) {
                return(base * custom_pow(base, power - 1));
            };
        };
        print(custom_pow(3, 4));
        return(0);
        print("Unreachable??");
    "#;

    c.bench_function("ast_parser", |b| {
        b.iter(|| Ast::parse_code(black_box(input)))
    });
}

criterion_group!(benches, benchmark_parser);
criterion_main!(benches);
