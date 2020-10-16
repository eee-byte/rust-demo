use memmap::{Mmap, MmapMut, MmapOptions};
use rand::{thread_rng, Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::thread;
use std::time::{Duration, Instant};

pub fn generate_file() {
    let mut rng = thread_rng();
    let mut pos: usize = 0;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("sealed")
        .unwrap();
    file.set_len(124 * 1024 * 1024 * 1024).unwrap();

    let mut mmap = unsafe { MmapMut::map_mut(&file).unwrap() };
    let mut rng_input = vec![0u8; 8 * 32];
    let mut data = vec![0u8; 2 * 1024 * 1024 * 1024];
    let mut bytes_written = 0;
    while pos < 124 * 1024 * 1024 * 1024 {
        let secret_number = rand::thread_rng().gen_range(1, 255);
        let rng = &mut XorShiftRng::from_seed([
            secret_number % 254 as u8,
            0x67,
            0xbe,
            0x5d,
            0x76,
            0x3d,
            0x33,
            0x84,
            0x12,
            0xdf,
            0x37,
            0x32,
            0x54,
            0x06,
            0xbc,
            0xe5,
        ]);
        rng.fill_bytes(&mut data);
        //let bytes_written = file.write(&rng_input).unwrap();
        (&mut mmap[pos..]).write_all(&data).unwrap();

        //if pos%(1024*1024*1024) == 0{
        // bytes_written = bytes_written + file.write(&data).unwrap();
        //}
        pos += 1024 * 1024 * 1024;
    }
    mmap.flush().unwrap();
}

fn random_read_file(str: &str) {
    let mut file = File::open(str).unwrap();
    let num = vec![1024, 10240, 1024 * 1024, 32 * 1024 * 1024, 64 * 1024 * 1024];
    let mut res = vec![0u8; 32];
    let mut i: usize = 0;

    for n in num {
        let now = Instant::now();
        println!("==============> read:{} before:{:?}", n, now);
        i = 0;
        while i < n {
            for x in 0..8 {
                let offset: u64 = rand::thread_rng().gen_range(1, 32 * 1024 * 1024 * 1024 - 32);

                file.seek(std::io::SeekFrom::Start(i as u64)).unwrap();
                file.read_exact(&mut res).unwrap();
            }
            i += 1;
            //println!("i:{:?} offset:{:?} res:{:x?}", i, offset, res);
        }
        println!("Elapsed time: {:.2?}", now.elapsed());
    }
}

fn random_read_mmap_mutil(str: &'static str) {
    for n in 0..15 {
        thread::spawn(move || {
            let mut hello = String::from(str);
            let mut file = File::open(hello + &n.to_string()).unwrap();
            let mut mmap_data = unsafe { Mmap::map(&file).unwrap() };
            let num = vec![
                32 * 1024 * 1024,
                32 * 1024 * 1024,
                32 * 1024 * 1024,
                32 * 1024 * 1024,
                64 * 1024 * 1024,
            ];
            let mut res = vec![0u8; 256];
            let mut i: usize = 0;

            for n in num {
                let now = Instant::now();
                println!("==============> read:{} before:{:?}", n, now);
                i = 0;
                while i < n {
                    for x in 0..8 {
                        let offset: usize =
                            rand::thread_rng().gen_range(1, 32 * 1024 * 1024 * 1024 - 32);
                        res = mmap_data[offset..offset + 32].to_vec();
                    }
                    //println!("i:{:?} res:{:x?}", i, res);
                    i += 1;
                }
                println!("Elapsed time: {:.2?}", now.elapsed());
            }
        });
    }
}

fn random_read_mmap(str: &str) {
    let mut file = File::open(str).unwrap();
    let mut mmap_data = unsafe { Mmap::map(&file).unwrap() };
    let num = vec![
        32 * 1024 * 1024,
        32 * 1024 * 1024,
        32 * 1024 * 1024,
        32 * 1024 * 1024,
        64 * 1024 * 1024,
    ];
    let mut res = vec![0u8; 256];
    let mut i: usize = 0;

    for n in num {
        let now = Instant::now();
        println!("==============> read:{} before:{:?}", n, now);
        i = 0;
        while i < n {
            for x in 0..8 {
                let offset: usize = rand::thread_rng().gen_range(1, 32 * 1024 * 1024 * 1024 - 32);
                res = mmap_data[offset..offset + 32].to_vec();
            }
            //println!("i:{:?} res:{:x?}", i, res);
            i += 1;
        }
        println!("Elapsed time: {:.2?}", now.elapsed());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_read_file() {
        let before = Instant::now();
        random_read_file("/root/test/sealed");
    }
    #[test]
    fn test_random_read_mmap() {
        let before = Instant::now();
        random_read_mmap("/root/test/sealed");
    }
    #[test]
    fn test_random_read_mmap_mutil() {
        let before = Instant::now();
        random_read_mmap_mutil("/root/test/sealed");
    }
}
