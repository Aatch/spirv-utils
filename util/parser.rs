#![allow(unused_imports)]
#![allow(unused_variables)]
use Group;
use Instruction;
use Param;
use ParamTy;
use Ty;
use std::str::FromStr;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Description {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use Group;
    use Instruction;
    use Param;
    use ParamTy;
    use Ty;
    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Description<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Group>, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, &mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Description((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _22_3f_22_3f((usize, ::std::option::Option<&'input str>, usize)),
        Description((usize, Vec<Group>, usize)),
        Group((usize, Group, usize)),
        Group_2a((usize, ::std::vec::Vec<Group>, usize)),
        Group_2b((usize, ::std::vec::Vec<Group>, usize)),
        Id((usize, String, usize)),
        Instruction((usize, Instruction, usize)),
        Instruction_2a((usize, ::std::vec::Vec<Instruction>, usize)),
        Instruction_2b((usize, ::std::vec::Vec<Instruction>, usize)),
        Name((usize, String, usize)),
        OpCode((usize, u16, usize)),
        Param((usize, Param, usize)),
        Param_2a((usize, ::std::vec::Vec<Param>, usize)),
        Param_2b((usize, ::std::vec::Vec<Param>, usize)),
        ParamTy((usize, ParamTy, usize)),
        Ty((usize, Ty, usize)),
        Ty_2b((usize, ::std::vec::Vec<Ty>, usize)),
        ____Description((usize, Vec<Group>, usize)),
    }

    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(input, __tokens, __sym0));
            }
            None => {
                let __start: usize = ::std::default::Default::default();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action34(input, &__start, &__end);
                let __nt = __Nonterminal::Description((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Description(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Group(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Group_2b(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Vec<Group>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __nt = __Nonterminal::____Description((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Group, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(input, __sym0);
                let __nt = __Nonterminal::Group_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Group>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(input, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(input, __sym0);
                let __nt = __Nonterminal::Description((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Group(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Group>, usize)>,
        __sym1: &mut Option<(usize, Group, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25(input, __sym0, __sym1);
                let __nt = __Nonterminal::Group_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state8(input, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __nt = __Nonterminal::Id((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, String, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym3));
            }
            None |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action36(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Group((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Instruction(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state9(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Instruction_2b(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state10(input, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::OpCode(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Instruction, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(input, __sym0);
                let __nt = __Nonterminal::Instruction_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, String, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
        __sym3: &mut Option<(usize, ::std::vec::Vec<Instruction>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (6, __tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state12(input, __tokens, __sym4));
            }
            None |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action37(input, __sym0, __sym1, __sym2, __sym3);
                let __nt = __Nonterminal::Group((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Instruction(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state13(input, __tokens, __lookahead, __sym3, __sym4));
                }
                __Nonterminal::OpCode(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state11(input, __tokens, __lookahead, __sym4));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u16, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(input, __sym0);
                let __nt = __Nonterminal::OpCode((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Instruction>, usize)>,
        __sym1: &mut Option<(usize, Instruction, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27(input, __sym0, __sym1);
                let __nt = __Nonterminal::Instruction_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u16, usize)>,
        __sym1: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state18(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Param(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Param_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state17(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(input, __tokens, __sym0, __sym1));
            }
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state21(input, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41(input, __sym0);
                let __nt = __Nonterminal::Param((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Param, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28(input, __sym0);
                let __nt = __Nonterminal::Param_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, u16, usize)>,
        __sym1: &mut Option<(usize, String, usize)>,
        __sym2: &mut Option<(usize, ::std::vec::Vec<Param>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state23(input, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(input, __tokens, __sym3));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state15(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Param(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state22(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u16, usize)>,
        __sym1: &mut Option<(usize, String, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action44(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Instruction((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state24(input, __tokens, __sym0, __sym1));
            }
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(input, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40(input, __sym0);
                let __nt = __Nonterminal::Param((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, String, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::ParamTy(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Ty(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, String, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39(input, __sym0, __sym1);
                let __nt = __Nonterminal::Param((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Param>, usize)>,
        __sym1: &mut Option<(usize, Param, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29(input, __sym0, __sym1);
                let __nt = __Nonterminal::Param_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, u16, usize)>,
        __sym1: &mut Option<(usize, String, usize)>,
        __sym2: &mut Option<(usize, ::std::vec::Vec<Param>, usize)>,
        __sym3: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action45(input, __sym0, __sym1, __sym2, __sym3);
                let __nt = __Nonterminal::Instruction((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (3, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(input, __tokens, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::ParamTy(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state30(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Ty(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38(input, __sym0, __sym1);
                let __nt = __Nonterminal::Param((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, String, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __nt = __Nonterminal::Ty((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, String, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, ParamTy, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action43(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Param((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Ty, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(input, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33(input, __sym0);
                let __nt = __Nonterminal::ParamTy((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Ty(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state32(input, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: &mut Option<(usize, ParamTy, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action42(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Param((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Ty, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action32(input, __sym0, __sym1);
                let __nt = __Nonterminal::ParamTy((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Ty, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state35(input, __tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ty(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ty_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, Ty, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __nt = __Nonterminal::Ty_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Ty, usize)>,
        __sym2: &mut Option<(usize, ::std::vec::Vec<Ty>, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, (4, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state37(input, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            Some((__loc1, (7, __tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state7(input, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state26(input, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Ty(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state36(input, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Ty, usize)>,
        __sym2: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6(input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::ParamTy((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Ty>, usize)>,
        __sym1: &mut Option<(usize, Ty, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1);
                let __nt = __Nonterminal::Ty_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: &mut Option<(usize, Ty, usize)>,
        __sym2: &mut Option<(usize, ::std::vec::Vec<Ty>, usize)>,
        __sym3: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7(input, __sym0, __sym1, __sym2, __sym3);
                let __nt = __Nonterminal::ParamTy((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Description::parse_Description;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        63 => /* '?' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        93 => /* ']' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 102 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        103 => /* 'g' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        104 ... 122 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 111 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        113 ... 122 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Group>, usize),
) -> Vec<Group>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Group>, usize),
) -> Vec<Group>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, is, _): (usize, ::std::vec::Vec<Instruction>, usize),
) -> Group
{
    {
        Group {
            name: name,
            instructions: is
        }
    }
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, op, _): (usize, u16, usize),
    (_, name, _): (usize, String, usize),
    (_, params, _): (usize, ::std::vec::Vec<Param>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Instruction
{
    {
        Instruction {
            opcode: op,
            name: name,
            params: params,
            group: None
        }
    }
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, name, _): (usize, String, usize),
    (_, opt, _): (usize, ::std::option::Option<&'input str>, usize),
) -> Param
{
    {
        Param {
            ty: ParamTy::Single(Ty::from_str(&name).unwrap(), opt.is_some()),
            name: name,
        }
    }
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, ParamTy, usize),
) -> Param
{
    {
        Param {
            name: name,
            ty:ty
        }
    }
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, Ty, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ParamTy
{
    ParamTy::Repeat(ty)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, Ty, usize),
    (_, tys, _): (usize, ::std::vec::Vec<Ty>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ParamTy
{
    ParamTy::RepeatMany(Some(ty).into_iter().chain(tys.into_iter()).collect())
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, ty, _): (usize, Ty, usize),
    (_, opt, _): (usize, ::std::option::Option<&'input str>, usize),
) -> ParamTy
{
    ParamTy::Single(ty, opt.is_some())
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Ty
{
    Ty::from_str(&__0).unwrap()
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "group".to_owned()
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    (__0).to_owned()
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u16
{
    u16::from_str(__0).unwrap()
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Ty, usize),
) -> ::std::vec::Vec<Ty>
{
    vec![__0]
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Ty>, usize),
    (_, e, _): (usize, Ty, usize),
) -> ::std::vec::Vec<Ty>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::option::Option<&'input str>
{
    Some(__0)
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<&'input str>
{
    None
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Param>
{
    vec![]
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Param>, usize),
) -> ::std::vec::Vec<Param>
{
    v
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Instruction>
{
    vec![]
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Instruction>, usize),
) -> ::std::vec::Vec<Instruction>
{
    v
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Group>
{
    vec![]
}

pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Group>, usize),
) -> ::std::vec::Vec<Group>
{
    v
}

pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Group, usize),
) -> ::std::vec::Vec<Group>
{
    vec![__0]
}

pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Group>, usize),
    (_, e, _): (usize, Group, usize),
) -> ::std::vec::Vec<Group>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> ::std::vec::Vec<Instruction>
{
    vec![__0]
}

pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Instruction>, usize),
    (_, e, _): (usize, Instruction, usize),
) -> ::std::vec::Vec<Instruction>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Param, usize),
) -> ::std::vec::Vec<Param>
{
    vec![__0]
}

pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Param>, usize),
    (_, e, _): (usize, Param, usize),
) -> ::std::vec::Vec<Param>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action30<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
) -> Param
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action16(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __0,
        __temp0,
    )
}

pub fn __action31<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
) -> Param
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __0,
        __temp0,
    )
}

pub fn __action32<
    'input,
>(
    input: &'input str,
    __0: (usize, Ty, usize),
    __1: (usize, &'input str, usize),
) -> ParamTy
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action16(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __temp0,
    )
}

pub fn __action33<
    'input,
>(
    input: &'input str,
    __0: (usize, Ty, usize),
) -> ParamTy
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __temp0,
    )
}

pub fn __action34<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Group>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action22(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

pub fn __action35<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Group>, usize),
) -> Vec<Group>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action23(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

pub fn __action36<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> Group
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action20(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

pub fn __action37<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<Instruction>, usize),
) -> Group
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action21(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

pub fn __action38<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> Param
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action10(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        __temp0,
        __1,
    )
}

pub fn __action39<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
) -> Param
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action11(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        __temp0,
        __1,
    )
}

pub fn __action40<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Param
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action10(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __temp0,
    )
}

pub fn __action41<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
) -> Param
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action11(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __temp0,
    )
}

pub fn __action42<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ParamTy, usize),
) -> Param
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action10(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        __temp0,
        __1,
        __2,
    )
}

pub fn __action43<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ParamTy, usize),
) -> Param
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action11(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        __temp0,
        __1,
        __2,
    )
}

pub fn __action44<
    'input,
>(
    input: &'input str,
    __0: (usize, u16, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> Instruction
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

pub fn __action45<
    'input,
>(
    input: &'input str,
    __0: (usize, u16, usize),
    __1: (usize, String, usize),
    __2: (usize, ::std::vec::Vec<Param>, usize),
    __3: (usize, &'input str, usize),
) -> Instruction
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action19(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
