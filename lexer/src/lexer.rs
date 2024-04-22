#![allow(unused)]

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/nophp.pest"]
pub struct NoPhpParser;
