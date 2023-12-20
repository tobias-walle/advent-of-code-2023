use std::{fmt::Debug, str::FromStr};

use anyhow::{bail, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1, space0, space1},
    combinator::{map_res, opt, recognize},
    error::ParseError,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

pub fn parse_with_nom<'a, 'b, P, T>(input: &'a str, parse: P) -> Result<T>
where
    P: FnOnce(&'a str) -> IResult<&'b str, T>,
    T: Debug,
{
    let (_, parsed) = match parse(input) {
        Ok(parsed) => parsed,
        Err(err) => {
            bail!("Failed to parse input: {err}")
        }
    };
    Ok(parsed)
}

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), T::from_str)(input)
}

macro_rules! create_delimited2_parser {
    ($name:ident, $delimiter1:expr, $delimiter2:expr) => {
        pub fn $name<'a, O, E, P>(parser: P) -> impl Parser<&'a str, O, E>
        where
            E: ParseError<&'a str>,
            P: Parser<&'a str, O, E>,
        {
            delimited($delimiter1, parser, $delimiter2)
        }
    };
}

macro_rules! create_delimited1_parser {
    ($name:ident, $delimiter:expr) => {
        create_delimited2_parser! {$name, $delimiter, $delimiter}
    };
}

create_delimited1_parser!(d_space0, space0);
create_delimited1_parser!(d_space1, space1);
create_delimited1_parser!(d_multispace0, multispace0);
create_delimited1_parser!(d_multispace1, multispace1);
create_delimited2_parser!(d_curly, tag("{"), tag("}"));
create_delimited2_parser!(d_round, tag("("), tag(")"));
create_delimited2_parser!(d_square, tag("["), tag("]"));
create_delimited2_parser!(d_double, tag("\""), tag("\""));
create_delimited2_parser!(d_single, tag("'"), tag("'"));

macro_rules! create_seperated_list_parser {
    ($name:ident, $list:expr, $seperator:expr) => {
        pub fn $name<'a, O, E, P>(parser: P) -> impl Parser<&'a str, Vec<O>, E>
        where
            E: ParseError<&'a str>,
            P: Parser<&'a str, O, E>,
        {
            $list($seperator, parser)
        }
    };
}

create_seperated_list_parser!(l0_comma, separated_list0, d_space0(tag(",")));
create_seperated_list_parser!(l1_comma, separated_list1, d_space0(tag(",")));
create_seperated_list_parser!(l0_semi, separated_list0, d_space0(tag(";")));
create_seperated_list_parser!(l1_semi, separated_list1, d_space0(tag(";")));
create_seperated_list_parser!(l0_newline, separated_list0, d_space0(tag("\n")));
create_seperated_list_parser!(l1_newline, separated_list1, d_space0(tag("\n")));

macro_rules! create_preceeded_parser {
    ($name:ident, $token:expr) => {
        pub fn $name<'a, O, E, P>(parser: P) -> impl Parser<&'a str, O, E>
        where
            E: ParseError<&'a str>,
            P: Parser<&'a str, O, E>,
        {
            preceded($token, parser)
        }
    };
}

create_preceeded_parser!(p_comma, tag(","));
create_preceeded_parser!(p_column, tag(":"));
create_preceeded_parser!(p_eq, tag("="));
create_preceeded_parser!(p_space, space0);
create_preceeded_parser!(p_mspace, multispace0);
