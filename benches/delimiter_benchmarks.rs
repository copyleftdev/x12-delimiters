use criterion::{black_box, criterion_group, criterion_main, Criterion};
use x12_delimiters::Delimiters;

const SAMPLE_ISA_SEGMENT: &[u8] = b"ISA*00*          *00*          *ZZ*SENDERID       *ZZ*RECEIVERID     *250403*0856*U*00501*000000001*0*P*:~";
const SAMPLE_ISA_ALT: &[u8] = b"ISA^00^          ^00^          ^ZZ^SENDERID       ^ZZ^RECEIVERID     ^250403^0856^U^00401^000000002^1^T^>}";

fn bench_default(c: &mut Criterion) {
    c.bench_function("Delimiters::default", |b| b.iter(|| black_box(Delimiters::default())));
}

fn bench_new(c: &mut Criterion) {
    c.bench_function("Delimiters::new", |b| {
        b.iter(|| black_box(Delimiters::new(black_box(b'~'), black_box(b'*'), black_box(b':'))))
    });
}

fn bench_from_isa_standard(c: &mut Criterion) {
    c.bench_function("from_isa_standard", |b| {
        b.iter(|| black_box(Delimiters::from_isa(black_box(SAMPLE_ISA_SEGMENT))))
    });
}

fn bench_from_isa_alternative(c: &mut Criterion) {
    c.bench_function("from_isa_alternative", |b| {
        b.iter(|| black_box(Delimiters::from_isa(black_box(SAMPLE_ISA_ALT))))
    });
}

fn bench_getters(c: &mut Criterion) {
    let delimiters = Delimiters::default();
    c.bench_function("delimiters_getters", |b| {
        b.iter(|| {
            let _seg = black_box(delimiters.segment_terminator());
            let _elem = black_box(delimiters.element_separator());
            let _sub = black_box(delimiters.sub_element_separator());
        })
    });
}

fn bench_are_valid(c: &mut Criterion) {
    let valid = Delimiters::new(b'~', b'*', b':');
    let invalid = Delimiters::new(b'~', b'~', b':');
    
    let mut group = c.benchmark_group("are_valid");
    group.bench_function("valid", |b| b.iter(|| black_box(valid.are_valid())));
    group.bench_function("invalid", |b| b.iter(|| black_box(invalid.are_valid())));
    group.finish();
}

criterion_group!(
    benches,
    bench_default,
    bench_new,
    bench_from_isa_standard,
    bench_from_isa_alternative,
    bench_getters,
    bench_are_valid
);
criterion_main!(benches);
