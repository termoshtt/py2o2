use super::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::opt,
    multi::{separated_list0, separated_list1},
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
pub struct FunctionDef<'input> {
    name: &'input str,
    args: Vec<Arg<'input>>,
    type_: Type<'input>,
}

pub fn function_def(input: &str) -> ParseResult<FunctionDef> {
    let (input, _def) = tag("def").parse(input)?;
    let (input, _space) = multispace1(input)?;
    let (input, name) = identifier(input)?;
    let (input, args) = delimited(
        char('('),
        separated_list0(tuple((multispace0, char(','), multispace0)), arg),
        char(')'),
    )
    .parse(input)?;
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
        },
    ))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Alias<'input> {
    name: &'input str,
    asname: Option<&'input str>,
}

pub fn alias(input: &str) -> ParseResult<Alias> {
    let (input, name) = identifier(input)?;
    let (input, asname) = opt(tuple((multispace1, tag("as"), multispace1, identifier))
        .map(|(_sp1, _as, _sp2, asname)| asname))
    .parse(input)?;
    Ok((input, Alias { name, asname }))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Import<'input> {
    names: Vec<Alias<'input>>,
}

pub fn import(input: &str) -> ParseResult<Import> {
    let (input, _import) = tag("import").parse(input)?;
    let (input, _sp) = multispace1(input)?;
    let (input, names) =
        separated_list1(tuple((multispace0, char(','), multispace0)), alias).parse(input)?;
    Ok((input, Import { names }))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ImportFrom<'input> {
    module: &'input str,
    names: Vec<Alias<'input>>,
}

pub fn import_from(input: &str) -> ParseResult<ImportFrom> {
    let (input, _from) = tag("from").parse(input)?;
    let (input, _sp) = multispace1(input)?;
    let (input, module) = identifier(input)?;
    let (input, _sp) = multispace1(input)?;
    let (input, _import) = tag("import").parse(input)?;
    let (input, _sp) = multispace1(input)?;
    let (input, names) =
        separated_list1(tuple((multispace0, char(','), multispace0)), alias).parse(input)?;
    Ok((input, ImportFrom { module, names }))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ClassDef<'input> {
    name: &'input str,
    bases: Vec<Expr<'input>>,
    keywords: Vec<Keyword<'input>>,
    body: Vec<Stmt<'input>>,
}

pub fn class_def(input: &str) -> ParseResult<ClassDef> {
    let (input, _class) = tag("class").parse(input)?;
    let (input, _sp) = multispace1(input)?;
    let (input, name) = identifier(input)?;
    let (input, _sp) = multispace0(input)?;

    let (input, bases) = opt(tuple((
        char('('),
        multispace0,
        separated_list0(tuple((multispace0, char(','), multispace0)), expr),
        multispace0,
        char(')'),
    ))
    .map(|(_open, _sp1, bases, _sp2, _close)| bases))
    .map(Option::unwrap_or_default)
    .parse(input)?;

    let (input, _sp) = multispace0(input)?;
    let (input, _comma) = char(':').parse(input)?;
    let (input, _sp) = multispace0(input)?;
    let (input, stmt) = stmt(input)?;

    Ok((
        input,
        ClassDef {
            name,
            bases,
            keywords: Vec::new(),
            body: vec![stmt],
        },
    ))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Stmt<'input> {
    ModuleDoc(&'input str),
    /// `Assign(expr* targets, expr value, string? type_comment)`
    Assign(Assign<'input>),
    /// `Import(alias* names)`
    Import(Import<'input>),
    /// `ImportFrom(identifier? module, alias* names, int? level)`
    ImportFrom(ImportFrom<'input>),
    /// `ClassDef(identifier name, expr* bases, keyword* keywords, stmt* body, expr* decorator_list)`
    ClassDef(ClassDef<'input>),
    /// `FunctionDef(identifier name, arguments args, stmt* body, expr* decorator_list, expr? returns, string? type_comment)`
    FunctionDef(FunctionDef<'input>),
    /// `If(expr test, stmt* body, stmt* orelse)`
    If(If<'input>),
    /// `Expr(expr value)`
    Expr(Expr<'input>),
}

pub fn stmt(input: &str) -> ParseResult<Stmt> {
    alt((
        string.map(Stmt::ModuleDoc),
        assign.map(Stmt::Assign),
        import.map(Stmt::Import),
        import_from.map(Stmt::ImportFrom),
        class_def.map(Stmt::ClassDef),
        function_def.map(Stmt::FunctionDef),
        if_.map(Stmt::If),
        expr.map(Stmt::Expr),
    ))
    .parse(input)
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Assign<'input> {
    targets: Vec<Expr<'input>>,
    value: Expr<'input>,
    type_comment: Option<Type<'input>>,
}

pub fn assign(input: &str) -> ParseResult<Assign> {
    let (input, targets) =
        separated_list1(tuple((multispace0, char(','), multispace0)), expr).parse(input)?;
    let (input, type_comment) =
        opt(tuple((multispace0, char(':'), multispace0, type_)).map(|(_sp1, _comma, _sp2, ty)| ty))
            .parse(input)?;
    let (input, (_sp1, _eq, _sp2, value)) =
        tuple((multispace0, char('='), multispace0, expr)).parse(input)?;
    Ok((
        input,
        Assign {
            targets,
            value,
            type_comment,
        },
    ))
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct If<'input> {
    test: Expr<'input>,
    body: Box<Stmt<'input>>,
    orelse: Option<Box<Stmt<'input>>>,
}

pub fn if_(input: &str) -> ParseResult<If> {
    let (input, _if) = tuple((tag("if"), multispace1)).parse(input)?;
    let (input, test) = expr(input)?;
    let (input, _colon) = tuple((multispace0, char(':'), multispace0))(input)?;
    let (input, body) = stmt.map(Box::new).parse(input)?;
    let (input, orelse) = opt(tuple((
        multispace1,
        tag("else"),
        multispace0,
        char(':'),
        multispace0,
        stmt,
    ))
    .map(|(_sp1, _else, _sp2, _colon, _sp3, orelse)| Box::new(orelse)))
    .parse(input)?;
    Ok((input, If { test, body, orelse }))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Finish;

    #[test]
    fn parse_arg() {
        insta::assert_debug_snapshot!(arg("a").finish().unwrap());
        insta::assert_debug_snapshot!(arg("a: T").finish().unwrap());
        insta::assert_debug_snapshot!(arg("a: T = None").finish().unwrap());
        insta::assert_debug_snapshot!(arg("a = None").finish().unwrap());
    }

    #[test]
    fn parse_class_def() {
        insta::assert_debug_snapshot!(class_def(
            r#"
            class A:
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());

        insta::assert_debug_snapshot!(class_def(
            r#"
            class A(B):
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());
    }

    #[test]
    fn parse_function_def() {
        insta::assert_debug_snapshot!(function_def(
            r#"
            def f(a, b):
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());
        insta::assert_debug_snapshot!(function_def(
            r#"
            def g(x: int) -> str:
                ...
            "#
            .trim()
        )
        .finish()
        .unwrap());
    }

    #[test]
    fn parse_import() {
        insta::assert_debug_snapshot!(import("import numpy, pandas").finish().unwrap());
        insta::assert_debug_snapshot!(import_from("from numpy import array").finish().unwrap());
    }

    #[test]
    fn parse_if() {
        insta::assert_debug_snapshot!(if_(r#"
            if sys.version_info >= (3, 9):
                ...
            "#
        .trim())
        .finish()
        .unwrap());
    }

    #[test]
    fn parse_assign() {
        insta::assert_debug_snapshot!(assign("a = 1").finish().unwrap());
        insta::assert_debug_snapshot!(assign("a: int = 1").finish().unwrap());
    }
}
