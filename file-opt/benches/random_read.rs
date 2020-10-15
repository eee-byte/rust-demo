use rand::{RngCore, thread_rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::fs::File;
use std::io::{Read, Seek, Write};
use memmap::{Mmap, MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion, ParameterizedBenchmark, Throughput};

fn random_read(str: &str) {
    let mut file = File::open(str).unwrap();
    let mut mmap_data = unsafe { Mmap::map(&file).unwrap() };
    let len = mmap_data.len();
    let mut mmap = unsafe {
        MmapOptions::new()
            .offset(0)
            .len(len)
            .map_mut(&file)
            .unwrap()
    };

    let mut res = vec![0u8; 32];
    let mut i: usize = 0;
    while i < 32 * 1024 * 1024 * 1024 - 32 {
        //let offset: u64 = rand::thread_rng().gen_range(1, 32 * 1024 * 1024 * 1024 - 32);

        // file.seek(std::io::SeekFrom::Start(i)).unwrap();
        // file.read_exact(&mut res).unwrap();
        (&mmap_data[i..i+32]).read(&mut res).unwrap();
        i = i+1;
        println!("count----------------------------------------{:?}", i);
        println!("res--------------------res.len--------------------{:02x?}     {:?}", res, res.len());

    }
}

fn random_read_benchmark(c: &mut Criterion) {
    c.bench(
        "read_random_benchmark",
        ParameterizedBenchmark::new(
            "read_random_benchmark",
            |b, size| {
                b.iter(|| random_read("./sealed"))
            },
            vec![1024,10240,1024*1024, 32*1024*1024],
        )
            .sample_size(100)
            .throughput(|s| Throughput::Bytes(*s as u64))
            .warm_up_time(Duration::from_secs(1)),
    );
}

criterion_group!(benches, random_read_benchmark);
criterion_main!(benches);