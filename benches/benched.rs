use criterion::{Criterion, criterion_main, criterion_group, BenchmarkId};

fn bench_simple(c: &mut Criterion) {
    use inject_test::simple::{SimpleHandler, DependOnCreateSimpleDataService, CreateSimpleDataService};
    
    c.bench_function("simple handler init", |b| { b.iter(|| { SimpleHandler::init() }) });

    let handler = SimpleHandler::init();

    c.bench_with_input(BenchmarkId::new("simple handler function", "simple"), &handler, |b, i| {
        b.iter(|| { i.create_simple_data_service().create("string".to_owned()) })
    });
}

fn bench_interactor(c: &mut Criterion) {
    use inject_test::interactor::{InteractionHandler, CreateDataService, DependOnCreateDataService};

    c.bench_function("interactor handler init", |b| { b.iter(|| { InteractionHandler::init() }) });

    let handler = InteractionHandler::init();

    c.bench_with_input(BenchmarkId::new("interactor handler function", "interaction"), &handler, |b, i| {
        b.iter(|| { i.create_data_service().create("string".to_owned()) })
    });
}

criterion_group!(benches, bench_simple, bench_interactor);
criterion_main!(benches);