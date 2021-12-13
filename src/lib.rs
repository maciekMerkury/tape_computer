#![allow(unused)]
#![feature(string_remove_matches)]
#[cfg(test)]
mod tests;

type Word = u16;
const WORD_SIZE: usize = std::mem::size_of::<Word>();

pub mod asm_parser;
pub mod hardware;

