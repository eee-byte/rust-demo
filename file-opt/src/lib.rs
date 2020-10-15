use rand::{thread_rng, Rng};
use rand::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::fs::File;
use std::io::{Read, Seek, Write};
use memmap::{Mmap, MmapMut, MmapOptions};
use std::fs::OpenOptions;

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
    let mut data = vec![0u8; 2*1024*1024*1024];
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
        pos += 1024*1024*1024;
    }
    mmap.flush().unwrap();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
