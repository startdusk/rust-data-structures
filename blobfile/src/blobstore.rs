use std::fs::{File, OpenOptions};
use std::io::Seek;
use std::io::SeekFrom;

use serde::Serialize;

use crate::blob::{read_u64, write_u64, Blob};
use crate::error::BlobError;

const COUNT_SIZE: u64 = 32;

/// This blob store will act as one half of the hashmap
/// as with the hashmap wrap this in something to make growing work
pub struct BlobStore {
    file: File,
    hseed: u64,
    block_size: u64,
    nblocks: u64,
    elems: u64,
}

impl BlobStore {
    pub fn new(fname: &str, block_size: u64, nblocks: u64) -> Result<Self, BlobError> {
        let hseed = rand::random::<u64>();
        // create_file
        let mut ff = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true) // the holder of this may want to read
            .open(fname)?;

        let f = &mut ff;
        f.set_len(COUNT_SIZE + block_size * nblocks)?;
        f.seek(SeekFrom::Start(0))?;
        write_u64(f, hseed)?;
        write_u64(f, block_size)?;
        write_u64(f, nblocks)?;
        write_u64(f, 0)?; // 0 elems in new store

        // mark beginnings of each block to show empty
        for x in 0..nblocks {
            f.seek(SeekFrom::Start(COUNT_SIZE + x * block_size))?;
            write_u64(f, 0)?; // Key length 0 means no item
            write_u64(f, block_size - 16)?;
        }
        Ok({
            BlobStore {
                hseed,
                file: ff,
                block_size,
                nblocks,
                elems: 0,
            }
        })
    }

    pub fn open(fname: &str) -> Result<Self, BlobError> {
        let mut ff = OpenOptions::new().write(true).read(true).open(fname)?;
        let f = &mut ff;
        f.seek(SeekFrom::Start(0))?;
        let hseed = read_u64(f)?;
        let block_size = read_u64(f)?;
        let nblocks = read_u64(f)?;
        let elems = read_u64(f)?;
        Ok(BlobStore {
            hseed,
            file: ff,
            block_size,
            nblocks,
            elems,
        })
    }

    pub fn new_or_open(fname: &str, bsize: u64, nblocks: u64) -> Result<Self, BlobError> {
        Self::new(fname, bsize, nblocks).or_else(|_| Self::open(fname))
    }

    pub fn inc_elems(&mut self, n: i32) -> Result<(), BlobError> {
        if n > 0 {
            self.elems += n as u64;
        } else {
            let n2 = (-n) as u64;
            if self.elems > n2 {
                self.elems -= n2;
            }
        }
        self.file.seek(SeekFrom::Start(24))?;
        write_u64(&mut self.file, self.elems)?;
        Ok(())
    }

    // does not remove if already there
    pub fn insert_only<K: Serialize, V: Serialize>(&mut self, k: K, v: V) -> Result<(), BlobError> {
        let blob = Blob::from(&k, &v)?;
        if blob.len() > self.block_size {
            // Let the wrapper make a file with a bigger group
            return Err(BlobError::TooBig(blob.len()));
        }

        let bucket = blob.k_hash(self.hseed) % self.nblocks;
        let f = &mut self.file;
        let mut pos = f.seek(SeekFrom::Start(COUNT_SIZE + self.block_size * bucket))?;
        // start each loop at beginning of an elem
        // remember klen == 0 means empty section
        loop {
            if pos >= COUNT_SIZE + self.block_size * (bucket + 1) {
                // reached end of data block
                // consider other handlings but this will tell the wrapper tp make space
                // another option is to overflow onto the end of the file.
                return Err(BlobError::NoRoom);
            }
            let klen = read_u64(f)?;
            let vlen = read_u64(f)?;
            if klen == 0 && blob.len() < vlen {
                f.seek(SeekFrom::Start(pos))?;
                blob.out(f)?;
                // add pointer immediatly after data ends
                write_u64(f, 0)?;
                write_u64(f, (vlen - blob.len()) - 16)?;
                return Ok(());
            }
            pos = f.seek(SeekFrom::Start(pos + 16 + klen + vlen))?;
        }
    }

    pub fn get<K: Serialize>(&mut self, k: &K) -> Result<Blob, BlobError> {
        let s_blob = Blob::from(k, &0)?;
        let bucket = s_blob.k_hash(self.hseed) % self.nblocks;
        let f = &mut self.file;

        let mut pos = f.seek(SeekFrom::Start(COUNT_SIZE + self.block_size * bucket))?;
        // start each loop in at front of block elem
        loop {
            if pos >= COUNT_SIZE + self.block_size * (bucket + 1) {
                return Err(BlobError::NotFound);
            }
            let b = Blob::read(f)?;
            if b.key_match(&s_blob) {
                return Ok(b);
            }
            pos += b.len();
        }
    }

    pub fn remove<K: Serialize>(&mut self, k: &K) -> Result<(), BlobError> {
        let s_blob = Blob::from(k, &0)?;
        let bucket = s_blob.k_hash(self.hseed) % self.nblocks;
        let f = &mut self.file;

        let mut pos = f.seek(SeekFrom::Start(COUNT_SIZE + self.block_size * bucket))?;
        //start each loop in at front of block elem
        let b_end = COUNT_SIZE + self.block_size * (bucket + 1);
        loop {
            if pos >= b_end {
                return Ok(());
            }
            let b = Blob::read(f)?;
            if b.key_match(&s_blob) {
                let l = b.len();
                //check if next block is empty, then we can join them

                if pos + l < b_end {
                    if read_u64(f)? == 0 {
                        let nlen = read_u64(f)?;
                        f.seek(SeekFrom::Start(pos))?;
                        write_u64(f, 0)?;
                        write_u64(f, l + nlen + 16)?;
                        return Ok(());
                    }
                }
                f.seek(SeekFrom::Start(pos))?;
                write_u64(f, 0)?;
                write_u64(f, l - 16)?;

                return Ok(());
            }
            pos += b.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_create_file() {
        let fs = "test_data/create_file";
        std::fs::remove_file(fs).ok();
        let mut bs = BlobStore::new(fs, 1000, 10).unwrap();
        let blocksize = bs.block_size;
        let mut b2 = BlobStore::open(fs).unwrap();
        assert_eq!(b2.block_size, blocksize);

        b2.insert_only("fish", "so long and thanks for all teh fish")
            .unwrap();

        bs.insert_only(55, "hello people").unwrap();
        bs.insert_only("green", "Another really long data thing")
            .unwrap();
        bs.insert_only("sandwich", vec!["young", "old", "not_sure"])
            .unwrap();

        //let blob = bs.get(&55).unwrap();
        assert_eq!(
            bs.get(&55).unwrap().get_v::<String>().unwrap(),
            "hello people".to_string()
        );
        assert_eq!(
            bs.get(&"green").unwrap().get_v::<String>().unwrap(),
            "Another really long data thing".to_string()
        );
        let vback: Vec<String> = bs.get(&"sandwich").unwrap().get_v().unwrap();
        assert_eq!(
            vback,
            vec![
                "young".to_string(),
                "old".to_string(),
                "not_sure".to_string()
            ]
        );

        bs.remove(&"green").unwrap();

        assert!(bs.get(&"green").is_err());
        assert!(bs.get(&55).is_ok());
    }

    #[test]
    pub fn test_reread() {
        std::fs::remove_file("test_data/bs_reread").ok();
        let bs = BlobStore::new("test_data/bs_reread", 1000, 10).unwrap();
        drop(bs);
        let b2 = BlobStore::open("test_data/bs_reread").unwrap();
        assert_eq!(b2.block_size, 1000);
    }
}
