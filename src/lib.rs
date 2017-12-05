extern crate byteorder;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::prelude::*;

pub enum Endian {
    Big,
    Little,
    //Native
}


pub struct BinaryReader {
    pub data: Vec<u8>,
    pub pos: usize,
    pub length: usize,
    pub endian: Endian
}

impl BinaryReader {
    fn initialize() -> BinaryReader {
        BinaryReader {
            data: Vec::new(),
            pos: 0,
            length: 0,
            endian: Endian::Big
        }
    }

    pub fn from_u8(get: &[u8]) -> BinaryReader {
        let mut a = BinaryReader::initialize();
        a.data = get.to_vec();
        a.length = get.len();
        a
    }

    pub fn from_vec(vec: &Vec<u8>) -> BinaryReader {
        let mut a = BinaryReader::initialize();
        a.data = vec.to_vec();
        a.length = vec.len();
        a
    }

    pub fn from_file(file: &mut std::fs::File) -> BinaryReader {
        let mut a = BinaryReader::initialize();
        let mut v: Vec<u8> = Vec::new();
        a.length = file.read_to_end(&mut v).unwrap();
        a.data = v;
        a
    }

    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian
    }

    pub fn jmp(&mut self, pos: usize) {
        self.pos = pos
    }

    pub fn adv(&mut self, size: usize) {
        self.pos += size
    }

    pub fn align(&mut self, size: usize) {
        self.pos = (self.pos + size - 1) / size * size
    }

    pub fn read(&mut self, size: usize) -> Option<&[u8]> {
        let data = self.data.get(self.pos..self.pos + size);
        self.pos += size;
        data
    }

    pub fn cstr(&mut self) -> String { // "abc" "null" "def"
        let mut data = self.data.clone().get(self.pos..self.length).unwrap().to_vec();
        data.reverse();
        let mut vec: Vec<u8> = Vec::new();
        loop {
            let a = data.pop().unwrap();
            if a == 0x00 { self.pos += vec.len() + 1; return String::from_utf8(vec).unwrap() }
            else { vec.push(a); }
        }
    }

    pub fn i8(&mut self) -> i8 {
        let mut data = self.data.get(self.pos..self.pos + 1).unwrap();
        self.pos += 1;
        data.read_i8().unwrap()
    }

    pub fn i16(&mut self) -> i16 {
        let mut data = self.data.get(self.pos..self.pos + 2).unwrap();
        self.pos += 2;
        match self.endian {
            Endian::Big =>  data.read_i16::<BigEndian>().unwrap(),
            Endian::Little => data.read_i16::<LittleEndian>().unwrap()
        }
    }

    pub fn i32(&mut self) -> i32 {
        let mut data = self.data.get(self.pos..self.pos + 4).unwrap();
        self.pos += 4;
        match self.endian {
            Endian::Big =>  data.read_i32::<BigEndian>().unwrap(),
            Endian::Little => data.read_i32::<LittleEndian>().unwrap()
        }
    }

    pub fn i64(&mut self) -> i64 {
        let mut data = self.data.get(self.pos..self.pos + 8).unwrap();
        self.pos += 8;
        match self.endian {
            Endian::Big =>  data.read_i64::<BigEndian>().unwrap(),
            Endian::Little => data.read_i64::<LittleEndian>().unwrap()
        }
    }

    pub fn u8(&mut self) -> u8 {
        let mut data = self.data.get(self.pos..self.pos + 1).unwrap();
        self.pos += 1;
        data.read_u8().unwrap()
    }

    pub fn u16(&mut self) -> u16 {
        let mut data = self.data.get(self.pos..self.pos + 2).unwrap();
        self.pos += 2;
        match self.endian {
            Endian::Big =>  data.read_u16::<BigEndian>().unwrap(),
            Endian::Little => data.read_u16::<LittleEndian>().unwrap()
        }
    }

    pub fn u32(&mut self) -> u32 {
        let mut data = self.data.get(self.pos..self.pos + 4).unwrap();
        self.pos += 4;
        match self.endian {
            Endian::Big =>  data.read_u32::<BigEndian>().unwrap(),
            Endian::Little => data.read_u32::<LittleEndian>().unwrap()
        }
    }

    pub fn u64(&mut self) -> u64 {
        let mut data = self.data.get(self.pos..self.pos + 8).unwrap();
        self.pos += 8;
        match self.endian {
            Endian::Big =>  data.read_u64::<BigEndian>().unwrap(),
            Endian::Little => data.read_u64::<LittleEndian>().unwrap()
        }
    }
}


#[cfg(test)]
mod tests;