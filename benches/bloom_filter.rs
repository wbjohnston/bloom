#![feature(test)]

extern crate bloom_filter_wbj;
use bloom_filter_wbj::BloomFilter;

extern crate test;
use test::Bencher;
use test::black_box;

#[bench]
fn bench_inst_with_size_100_fp_01(bencher: &mut Bencher) {
    let elems = black_box(100);
    let fp = black_box(0.01f32);

    bencher.iter(|| { BloomFilter::new_with_fp(elems, fp); });
}

#[bench]
fn bench_inst_with_size_1000_fp_01(bencher: &mut Bencher) {
    let elems = black_box(1_000);
    let fp = black_box(0.01f32);

    bencher.iter(|| { BloomFilter::new_with_fp(elems, fp); });
}

#[bench]
fn bench_inst_with_size_10000_fp_01(bencher: &mut Bencher) {
    let elems = black_box(10_000);
    let fp = black_box(0.01f32);

    bencher.iter(|| { BloomFilter::new_with_fp(elems, fp); });
}

#[bench]
fn bench_ins_size_100_fp_01(bencher: &mut Bencher) {
    let elems = 100;
    let fp = 0.01f32;
    let to_insert = black_box(&[1, 2, 3, 4, 5]);
    let mut filter = BloomFilter::new_with_fp(elems, fp);

    bencher.iter(|| { filter.insert(to_insert); });
}
