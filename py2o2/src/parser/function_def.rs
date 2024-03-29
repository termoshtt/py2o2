use super::{builtin::*, expr::*, ParseResult};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::opt,
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    Parser,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Type<'input> {
    Name(&'input str),
    None,
}

pub fn type_(input: &str) -> ParseResult<Type> {
    // FIXME: More possible types e.g. `Callable`
    let (input, name) = identifier(input)?;
    Ok((input, Type::Name(name)))
}

/// `arg = (identifier arg, expr? annotation, string? type_comment)`
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Arg<'input> {
    pub name: &'input str,
    pub ty: Option<Type<'input>>,
    pub default: Option<Expr<'input>>,
}

pub fn arg(input: &str) -> ParseResult<Arg> {
    let (input, (name, ty, default)) = tuple((
        identifier,
        opt(tuple((multispace0, char(':'), multispace0, type_)).map(|(_sp1, _colon, _sp2, ty)| ty)),
        opt(tuple((multispace0, char('='), multispace0, expr))
            .map(|(_sp1, _colon, _sp2, default)| default)),
    ))
    .parse(input)?;
    Ok((input, Arg { name, ty, default }))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum ArgLike<'input> {
    /// Usual argument
    Arg(Arg<'input>),
    /// `*args`
    VarArg(Arg<'input>),
    /// `**kwds`
    KwArg(Arg<'input>),
    /// `/`
    PositionalSep,
    /// `*`
    KeywordSep,
}

pub fn arg_like(input: &str) -> ParseResult<ArgLike> {
    alt((
        char('/').map(|_| ArgLike::PositionalSep),
        tuple((char('*'), multispace0, arg)).map(|(_star, _sp, arg)| ArgLike::VarArg(arg)),
        tuple((tag("**"), multispace0, arg)).map(|(_star, _sp, arg)| ArgLike::KwArg(arg)),
        char('*').map(|_| ArgLike::KeywordSep),
        arg.map(ArgLike::Arg),
    ))
    .parse(input)
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Arguments<'input> {
    /// Usual arguments
    args: Vec<Arg<'input>>,
    /// Args before `/`
    positional_only: Vec<Arg<'input>>,
    /// Args after `*`
    keyword_only: Vec<Arg<'input>>,
    /// `*arg` will be stored without `*`
    var_args: Option<Arg<'input>>,
    /// `**kwargs` will be stored without `**`
    kw_args: Option<Arg<'input>>,
}

impl<'input> From<Vec<ArgLike<'input>>> for Arguments<'input> {
    fn from(args: Vec<ArgLike<'input>>) -> Self {
        let mut arguments = Self::default();
        let mut cursor = &mut arguments.args;
        for arg in args {
            match arg {
                ArgLike::Arg(arg) => cursor.push(arg),
                ArgLike::PositionalSep => arguments.positional_only.append(cursor),
                ArgLike::KeywordSep => cursor = &mut arguments.keyword_only,
                ArgLike::KwArg(arg) => arguments.kw_args = Some(arg),
                ArgLike::VarArg(arg) => arguments.var_args = Some(arg),
            }
        }
        arguments
    }
}

pub fn arguments(input: &str) -> ParseResult<Arguments> {
    let (input, args) =
        separated_list0(tuple((multispace0, char(','), multispace0)), arg_like).parse(input)?;
    Ok((input, args.into()))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct FunctionDef<'input> {
    name: &'input str,
    args: Arguments<'input>,
    type_: Type<'input>,
    decorators: Vec<Expr<'input>>,
}

pub fn function_def(input: &str) -> ParseResult<FunctionDef> {
    let (input, decorators) = many0(
        tuple((char('@'), multispace0, expr, multispace0)).map(|(_at, _sp1, expr, _sp2)| expr),
    )
    .parse(input)?;
    let (input, _def) = tag("def").parse(input)?;
    let (input, _space) = multispace1(input)?;
    let (input, name) = identifier(input)?;
    let (input, args) = delimited(char('('), arguments, char(')')).parse(input)?;
    let (input, ty) =
        opt(tuple((multispace0, tag("->"), multispace0, type_)).map(|(_sp1, _arrow, _sp2, ty)| ty))
            .parse(input)?;
    let (input, _colon) = char(':').parse(input)?;
    let (input, _sp) = multispace0(input)?;
    let (input, _expr) = expr(input)?;
    Ok((
        input,
        FunctionDef {
            name,
            args,
            type_: ty.unwrap_or(Type::None),
            decorators,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::{super::test::CheckParsed, *};

    #[test]
    fn parse_arg() {
        insta::assert_debug_snapshot!(arg("a").check_parsed(),
            @r###"
            Arg {
                name: "a",
                ty: None,
                default: None,
            }
            "###
        );
        insta::assert_debug_snapshot!(arg("a: T").check_parsed(),
            @r###"
            Arg {
                name: "a",
                ty: Some(
                    Name(
                        "T",
                    ),
                ),
                default: None,
            }
            "###
        );
        insta::assert_debug_snapshot!(arg("a: T = None").check_parsed(),
            @r###"
            Arg {
                name: "a",
                ty: Some(
                    Name(
                        "T",
                    ),
                ),
                default: Some(
                    None,
                ),
            }
            "###
        );
        insta::assert_debug_snapshot!(arg("a = None").check_parsed(),
            @r###"
            Arg {
                name: "a",
                ty: None,
                default: Some(
                    None,
                ),
            }
            "###
        );
    }

    #[test]
    fn parse_function_def() {
        insta::assert_debug_snapshot!(
            function_def(
                r#"
                def f(a, b):
                    ...
                "#.trim()
            ).check_parsed(),
            @r###"
            FunctionDef {
                name: "f",
                args: Arguments {
                    args: [
                        Arg {
                            name: "a",
                            ty: None,
                            default: None,
                        },
                        Arg {
                            name: "b",
                            ty: None,
                            default: None,
                        },
                    ],
                    positional_only: [],
                    keyword_only: [],
                    var_args: None,
                    kw_args: None,
                },
                type_: None,
                decorators: [],
            }
            "###
        );

        // type hint
        insta::assert_debug_snapshot!(
            function_def(
                r#"
                def g(x: int) -> str:
                    ...
                "#
                .trim()
            ).check_parsed(),
            @r###"
            FunctionDef {
                name: "g",
                args: Arguments {
                    args: [
                        Arg {
                            name: "x",
                            ty: Some(
                                Name(
                                    "int",
                                ),
                            ),
                            default: None,
                        },
                    ],
                    positional_only: [],
                    keyword_only: [],
                    var_args: None,
                    kw_args: None,
                },
                type_: Name(
                    "str",
                ),
                decorators: [],
            }
            "###
        );

        // *arg
        insta::assert_debug_snapshot!(
            function_def(
                r#"
                def g(*args) -> str:
                    ...
                "#
                .trim()
            ).check_parsed(),
            @r###"
            FunctionDef {
                name: "g",
                args: Arguments {
                    args: [],
                    positional_only: [],
                    keyword_only: [],
                    var_args: Some(
                        Arg {
                            name: "args",
                            ty: None,
                            default: None,
                        },
                    ),
                    kw_args: None,
                },
                type_: Name(
                    "str",
                ),
                decorators: [],
            }
            "###
        );
        // **keywords
        insta::assert_debug_snapshot!(
        function_def(
            r#"
            def g(**keywords) -> str:
                ...
            "#.trim()
        ).check_parsed(), @r###"
        FunctionDef {
            name: "g",
            args: Arguments {
                args: [],
                positional_only: [],
                keyword_only: [],
                var_args: None,
                kw_args: Some(
                    Arg {
                        name: "keywords",
                        ty: None,
                        default: None,
                    },
                ),
            },
            type_: Name(
                "str",
            ),
            decorators: [],
        }
        "###
        );

        // Positional only
        insta::assert_debug_snapshot!(function_def(
            r#"
            def f(x, /, a=1):
                ...
            "#.trim()
        ).check_parsed(), @r###"
        FunctionDef {
            name: "f",
            args: Arguments {
                args: [
                    Arg {
                        name: "a",
                        ty: None,
                        default: Some(
                            Constant {
                                value: Float(
                                    1.0,
                                ),
                            },
                        ),
                    },
                ],
                positional_only: [
                    Arg {
                        name: "x",
                        ty: None,
                        default: None,
                    },
                ],
                keyword_only: [],
                var_args: None,
                kw_args: None,
            },
            type_: None,
            decorators: [],
        }
        "###
        );

        // Keyword only
        insta::assert_debug_snapshot!(function_def(
            r#"
            def f(x, *, a=1):
                ...
            "#.trim()
        ).check_parsed(), @r###"
        FunctionDef {
            name: "f",
            args: Arguments {
                args: [
                    Arg {
                        name: "x",
                        ty: None,
                        default: None,
                    },
                ],
                positional_only: [],
                keyword_only: [
                    Arg {
                        name: "a",
                        ty: None,
                        default: Some(
                            Constant {
                                value: Float(
                                    1.0,
                                ),
                            },
                        ),
                    },
                ],
                var_args: None,
                kw_args: None,
            },
            type_: None,
            decorators: [],
        }
        "###
        );

        // Combined
        insta::assert_debug_snapshot!(function_def(
            r#"
            def f(x, /, y, *, a=1):
                ...
            "#
            .trim()
        ).check_parsed(), @r###"
        FunctionDef {
            name: "f",
            args: Arguments {
                args: [
                    Arg {
                        name: "y",
                        ty: None,
                        default: None,
                    },
                ],
                positional_only: [
                    Arg {
                        name: "x",
                        ty: None,
                        default: None,
                    },
                ],
                keyword_only: [
                    Arg {
                        name: "a",
                        ty: None,
                        default: Some(
                            Constant {
                                value: Float(
                                    1.0,
                                ),
                            },
                        ),
                    },
                ],
                var_args: None,
                kw_args: None,
            },
            type_: None,
            decorators: [],
        }
        "###
        );

        // decorators
        insta::assert_debug_snapshot!(function_def(
            r#"
            @staticmethod
            def f(a, b):
                ...
            "#.trim()
        ).check_parsed(), @r###"
        FunctionDef {
            name: "f",
            args: Arguments {
                args: [
                    Arg {
                        name: "a",
                        ty: None,
                        default: None,
                    },
                    Arg {
                        name: "b",
                        ty: None,
                        default: None,
                    },
                ],
                positional_only: [],
                keyword_only: [],
                var_args: None,
                kw_args: None,
            },
            type_: None,
            decorators: [
                Name {
                    id: "staticmethod",
                },
            ],
        }
        "###
        );

        // decorators with arguments
        insta::assert_debug_snapshot!(function_def(
            r#"
            @deco(1, "test")
            def f(a, b):
                ...
            "#.trim()
        ).check_parsed(), @r###"
        FunctionDef {
            name: "f",
            args: Arguments {
                args: [
                    Arg {
                        name: "a",
                        ty: None,
                        default: None,
                    },
                    Arg {
                        name: "b",
                        ty: None,
                        default: None,
                    },
                ],
                positional_only: [],
                keyword_only: [],
                var_args: None,
                kw_args: None,
            },
            type_: None,
            decorators: [
                Call {
                    func: Name {
                        id: "deco",
                    },
                    args: [
                        Constant {
                            value: Float(
                                1.0,
                            ),
                        },
                        Constant {
                            value: String(
                                "test",
                            ),
                        },
                    ],
                    keywords: [],
                },
            ],
        }
        "###
        );
    }
}
