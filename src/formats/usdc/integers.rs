/*
Given a list of integers, say:
let input = [123, 124, 125, 100125, 100125, 100126, 100126]

We encode as follows.  First, we transform the list to be the list of
differences to the previous integer, or the integer itself for the first element
in the list (this can be considered a difference to 0) to get:

let input_diffs = [123, 1, 1, 100000, 0, 1, 0]

Then we find the most commonly occurring value in this sequence, which is '1'.
We write this most commonly occurring value into the output stream.

output = [int32(1)]

Next we write two sections, first a fixed length section, 2-bit codes per
integer, followed by a variable length section of integer data.  The two bit
code indicates what "kind" of integer we have:

00: The most common value
01:  8-bit integer
10: 16-bit integer
11: 32-bit integer

For our example this gives:

input  = [123, 124, 125, 100125, 100125, 100126, 10026]
output = [int32(1) 01 00 00 11 01 00 01 XX int8(123) int32(100000) int8(0) int8(0)]

Where 'XX' represents unused bits in the last byte of the codes section to round
up to a whole number of bytes.
 */


use std::cmp::{min, max};
use nom::{IResult, number::complete::{le_i32, le_i8, le_i16, le_u32}};
use crate::formats::compress::{decompress_from_buffer, get_compressed_buffer_size};

fn get_encoded_buffer_size<T>(number_of_integers: usize) -> usize {
    if number_of_integers == 0 {
        return 0;
    }
    let common_value_size = std::mem::size_of::<T>();

    return  /* commonValue */    common_value_size 
        +   /* numCodesBytes */  ((number_of_integers * 2 + 7) / 8)  
        +   /* maxIntBytes */    (number_of_integers  * common_value_size);
}



fn parse_code(input: (&[u8], usize), count: usize)-> IResult<(&[u8], usize), u8> {
    nom::bits::complete::take(count)(input)
}

pub fn decode_integers(number_of_integers: usize) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<u32>> {
    type T = u32;
    move |input: &[u8]| {
        let (input, common_value) = le_u32(input)?;
        let codes_size = (number_of_integers * 2 + 7) / 8;
        let (codes, input) = input.split_at(codes_size);

        let mut values = vec![common_value; number_of_integers];
        
        let mut next = input;
        let mut current_value:T = 0;
        for i in 0..number_of_integers {
            let (_, code) = parse_code((codes, i * 2), 2).unwrap();

            match code {
                0b01 => {
                    let (remainder, value) = le_i8(next)?;
                    next = remainder;
                    current_value += value as T;
                }
                0b10 => {
                    let (remainder, value) = le_i16(next)?;
                    next = remainder;
                    current_value += value as T;
                }
                0b11 => {
                    let (remainder, value) = le_i32(next)?;
                    next = remainder;
                    current_value += value as T;
                }
                _ => {
                   current_value += common_value;
                }
            }
            values[i] = current_value;
        }
        Ok((input, values))
    }
}

pub fn decompress_integers(number_of_integers: usize, compressed_size:u64) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<u32>> {
    move |input: &[u8]|{
        let uncompressed_size = get_encoded_buffer_size::<u32>(number_of_integers);

        let buffer_size = get_compressed_buffer_size(uncompressed_size);
        
        let compressed_size = min(compressed_size, buffer_size as u64);
        
        let (rest, decompressed_data) = decompress_from_buffer(uncompressed_size as u64, compressed_size)(input).unwrap();
        let (_, integers) = decode_integers(number_of_integers)(&decompressed_data).unwrap();
        Ok((rest, integers))
    }
}
