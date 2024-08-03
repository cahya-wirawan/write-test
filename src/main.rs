fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    const VEC_STEP: usize = 1024*1024;
    use std::fs::File;
    use std::io::{BufWriter, Seek, Write};
    use bytemuck::cast_slice;
    use std::cmp;

    #[test]
    fn cast_slice_1() {
        let doc_length: u64 = 0x12345678; // 0x12345678
        let binding = [doc_length];
        let ret:&[u8] = cast_slice(&binding);
        println!("ret: {:?} - {:?}", ret, ret.len());
        assert_eq!(ret.len(), 8);
        assert_eq!(ret[0], 0x78);
        assert_eq!(ret[1], 0x56);
        assert_eq!(ret[2], 0x34);
        assert_eq!(ret[3], 0x12);
        assert_eq!(ret[4], 0x00);
    }

    #[test]
    fn cast_slice_2() {
        let mut doc_sizes: Vec<u32> = Vec::new();
        doc_sizes.push(0x78);
        doc_sizes.push(0x56);
        doc_sizes.push(0x34);
        doc_sizes.push(0x12);
        doc_sizes.push(0x00);
        let ret:&[u8] = cast_slice(&doc_sizes);
        println!("ret: {:?} - {:?}", ret, ret.len());
        assert_eq!(ret.len(), doc_sizes.len()*4);
        assert_eq!(ret[0], 0x78);
        assert_eq!(ret[4], 0x56);
        assert_eq!(ret[8], 0x34);
        assert_eq!(ret[12], 0x12);
        assert_eq!(ret[16], 0x00);
    }

    #[test]
    fn cast_slice_3() {
        let max_index:usize = 1000;
        let mut doc_sizes: Vec<u32> = Vec::new();
        for index in 0..max_index {
            let num = index%256;
            doc_sizes.push(num as u32);
        }
        let ret:&[u8] = cast_slice(&doc_sizes);
        println!("ret: {:?}", ret.len());
        assert_eq!(ret.len(), 4000);
    }

    #[test]
    fn cast_slice_4() {
        let max_index:usize = 0x1000;
        let mut doc_sizes: Vec<u32> = Vec::new();
        for index in 0..max_index {
            let num = index%256;
            doc_sizes.push(num as u32);
        }
        let ret:&[u8] = cast_slice(&doc_sizes);
        println!("ret: {:?}", ret.len());
        assert_eq!(ret.len(), 16384);
        let file_idx = File::create("./test_slice_4.idx").expect("couldn't open file");
        let mut file_idx_writer = BufWriter::new(file_idx);
        file_idx_writer.write(ret).expect("Can't write");
        let file_size = file_idx_writer.stream_position().unwrap();
        println!("File size: {file_size}");
        assert_eq!(file_size, 16384);
    }

    #[test]
    fn cast_slice_5() {
        let max_index:usize = 0x366b02d0; // 912.982.736
        let mut doc_sizes: Vec<u32> = Vec::new();
        for index in 0..max_index {
            let num = index%256;
            doc_sizes.push(num as u32);
        }
        let file_idx = File::create("./test_slice_5.idx").expect("couldn't open file");
        let mut file_idx_writer = BufWriter::new(file_idx);
        for i in (0..doc_sizes.len()).step_by(VEC_STEP) {
            let ret:&[u8] = cast_slice(&doc_sizes[i..cmp::min(i+VEC_STEP, doc_sizes.len())]);
            file_idx_writer.write(ret).expect("Can't write");
        }
        let file_size = file_idx_writer.stream_position().unwrap();
        println!("File size: {file_size}");
        assert_eq!(file_size, 3651930944);
    }
}
