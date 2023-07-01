use nom::{
    number::complete::{le_i32, le_u8},
    IResult,
};
use std::{cmp::min, mem};

const LZ4_MAX_INPUT_SIZE: usize = 0x7E000000;

fn get_max_input_size() -> usize {
    127 * LZ4_MAX_INPUT_SIZE
}

fn get_compress_bound(size: usize) -> usize {
    let bound = unsafe { lz4_sys::LZ4_compressBound(size as i32) };
    return bound as usize;
}

pub fn get_compressed_buffer_size(input_size: usize) -> usize {
    if input_size > get_max_input_size() {
        return 0;
    }

    if input_size <= LZ4_MAX_INPUT_SIZE {
        return get_compress_bound(input_size) + 1;
    }

    let n_whole_chunks = input_size / LZ4_MAX_INPUT_SIZE;
    let part_chunk_sz = input_size % LZ4_MAX_INPUT_SIZE;
    let sz = 1 + n_whole_chunks * (get_compress_bound(LZ4_MAX_INPUT_SIZE) + mem::size_of::<i32>());
    if part_chunk_sz != 0 {
        return sz + get_compress_bound(part_chunk_sz) + mem::size_of::<i32>();
    }
    sz
}

fn decompress_chunk(
    input: &[u8],
    compressed_size: usize,
    uncompressed_size: usize,
) -> IResult<&[u8], Vec<u8>> {
    let (input, rest) = input.split_at(compressed_size);
    let decompressed_bytes =
        lz4_flex::decompress(input, min(LZ4_MAX_INPUT_SIZE, uncompressed_size)).unwrap();
    return Ok((rest, decompressed_bytes));
}

fn decompressed_size_prepended(
    uncompressed_size: usize,
) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<u8>> {
    move |input: &[u8]| {
        let (input, compressed_size) = le_i32(input)?;
        decompress_chunk(input, compressed_size as usize, uncompressed_size)
    }
}

pub fn decompress_from_buffer(
    uncompressed_size: u64,
    compressed_size: u64,
) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<u8>> {
    move |input: &[u8]| {
        let (input, number_of_chunks) = le_u8(input)?;

        if number_of_chunks == 0 {
            return decompress_chunk(
                input,
                compressed_size as usize - 1,
                uncompressed_size as usize,
            );
        }
        let mut size_left: usize = uncompressed_size as usize;
        let mut input = input;
        let mut chunks: Vec<u8> = Vec::new();

        for _ in 0..number_of_chunks {
            let result = decompressed_size_prepended(size_left)(input)?;
            input = result.0;
            size_left = size_left - result.1.len();
            chunks.extend(result.1);
        }
        Ok((input, chunks))
    }
}
