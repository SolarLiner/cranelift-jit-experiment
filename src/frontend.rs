use crate::validator::ExprType;
use std::fmt::Display;

use std::fmt::Write;

#[derive(Debug, Copy, Clone)]
pub enum Unaryop {
    Not,
}

impl Display for Unaryop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unaryop::Not => write!(f, "!"),
        }
    }
}

/// "Mathematical" binary operations variants
#[derive(Debug, Copy, Clone)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    LogicalAnd,
    LogicalOr,
}

impl Display for Binop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binop::Add => write!(f, "+"),
            Binop::Sub => write!(f, "-"),
            Binop::Mul => write!(f, "*"),
            Binop::Div => write!(f, "/"),
            Binop::LogicalAnd => write!(f, "&&"),
            Binop::LogicalOr => write!(f, "||"),
        }
    }
}

/// Comparison operations
#[derive(Debug, Copy, Clone)]
pub enum Cmp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Display for Cmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmp::Eq => write!(f, "=="),
            Cmp::Ne => write!(f, "!="),
            Cmp::Lt => write!(f, "<"),
            Cmp::Le => write!(f, "<="),
            Cmp::Gt => write!(f, ">"),
            Cmp::Ge => write!(f, ">="),
        }
    }
}

type NV<T> = non_empty_vec::NonEmpty<T>;

/// The AST node for expressions.
#[derive(Debug, Clone)]
pub enum Expr {
    LiteralFloat(String),
    LiteralInt(String),
    LiteralBool(bool),
    LiteralString(String),
    Identifier(String),
    Binop(Binop, Box<Expr>, Box<Expr>),
    Unaryop(Unaryop, Box<Expr>),
    Compare(Cmp, Box<Expr>, Box<Expr>),
    IfThen(Box<Expr>, Vec<Expr>),
    IfElse(Box<Expr>, Vec<Expr>, Vec<Expr>),
    Assign(NV<String>, NV<Expr>),
    AssignOp(Binop, Box<String>, Box<Expr>),
    NewStruct(String, Vec<StructAssignField>),
    WhileLoop(Box<Expr>, Vec<Expr>), //Should this take a block instead of Vec<Expr>?
    Block(Vec<Expr>),
    Call(String, Vec<Expr>, bool),
    GlobalDataAddr(String),
    Parentheses(Box<Expr>),
    ArrayGet(String, Box<Expr>),
    ArraySet(String, Box<Expr>, Box<Expr>),
}

//TODO indentation, tests
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::LiteralFloat(s) => write!(f, "{}", s),
            Expr::LiteralInt(s) => write!(f, "{}", s),
            Expr::LiteralString(s) => write!(f, "\"{}\"", s),
            Expr::Identifier(s) => write!(f, "{}", s),
            Expr::Binop(op, e1, e2) => write!(f, "{} {} {}", e1, op, e2),
            Expr::Unaryop(op, e1) => write!(f, "{} {}", op, e1),
            Expr::Compare(cmp, e1, e2) => write!(f, "{} {} {}", e1, cmp, e2),
            Expr::IfThen(e, body) => {
                writeln!(f, "if {} {{", e)?;
                for expr in body.iter() {
                    writeln!(f, "{}", expr)?;
                }
                write!(f, "}}")?;
                Ok(())
            }
            Expr::IfElse(e, body, else_body) => {
                writeln!(f, "if {} {{", e)?;
                for expr in body.iter() {
                    writeln!(f, "{}", expr)?;
                }
                writeln!(f, "}} else {{")?;
                for expr in else_body.iter() {
                    writeln!(f, "{}", expr)?;
                }
                write!(f, "}}")?;
                Ok(())
            }
            Expr::Assign(vars, exprs) => {
                for (i, var) in vars.iter().enumerate() {
                    write!(f, "{}", var)?;
                    let len: usize = vars.len().into();
                    if i < len - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " = ")?;
                for (i, expr) in exprs.iter().enumerate() {
                    write!(f, "{}", expr)?;
                    let len: usize = exprs.len().into();
                    if i < len - 1 {
                        write!(f, ", ")?;
                    }
                }
                Ok(())
            }
            Expr::AssignOp(op, s, e) => write!(f, "{} {}= {}", s, op, e),
            Expr::NewStruct(struct_name, args) => {
                writeln!(f, "{}{{", struct_name)?;
                for arg in args.iter() {
                    writeln!(f, "{},", arg)?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
            Expr::WhileLoop(eval, block) => {
                writeln!(f, "while {} {{", eval)?;
                for expr in block.iter() {
                    writeln!(f, "{}", expr)?;
                }
                write!(f, "}}")?;
                Ok(())
            }
            Expr::Block(block) => {
                for expr in block.iter() {
                    writeln!(f, "{}", expr)?;
                }
                Ok(())
            }
            Expr::Call(func_name, args, _impl_func) => {
                //todo print this correctly
                write!(f, "{}(", func_name)?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")?;
                Ok(())
            }
            Expr::GlobalDataAddr(e) => write!(f, "{}", e),
            Expr::LiteralBool(b) => write!(f, "{}", b),
            Expr::Parentheses(e) => write!(f, "({})", e),
            Expr::ArrayGet(var, e) => write!(f, "{}[{}]", var, e),
            Expr::ArraySet(var, idx_e, e) => write!(f, "{}[{}] = {}", var, idx_e, e),
        }
    }
}

pub fn make_nonempty<T>(v: Vec<T>) -> Option<NV<T>> {
    if v.is_empty() {
        None
    } else {
        Some(unsafe { NV::new_unchecked(v) })
    }
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Function(Function),
    Metadata(Vec<String>, String),
    Struct(Struct),
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::Function(e) => write!(f, "{}", e),
            Declaration::Metadata(head, body) => {
                for word in head.iter() {
                    write!(f, "{}", word)?;
                }
                writeln!(f, "")?;
                write!(f, "{}", body)?;
                Ok(())
            }
            Declaration::Struct(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub expr_type: Option<ExprType>, //Type is F64 if not specified
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.expr_type {
            Some(t) => write!(f, "{}: {}", self.name, t),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructAssignField {
    pub field_name: String,
    pub expr: Expr,
}

impl Display for StructAssignField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field_name, self.expr)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Arg>,
    pub returns: Vec<Arg>,
    pub body: Vec<Expr>,
    pub extern_func: bool,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {} (", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            write!(f, "{}", param)?;
            if i < self.params.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ") -> (")?;
        for ret in self.returns.iter() {
            write!(f, "{}", ret)?;
        }
        writeln!(f, ") {{")?;
        for expr in self.body.iter() {
            writeln!(f, "{}", expr)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Arg>,
    pub extern_struct: bool,
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct {} {{", self.name)?;
        for param in &self.fields {
            writeln!(f, "{},", param)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

// TODO there must be a better way.
pub fn pretty_indent(code: &str) -> String {
    let mut f = String::from("");
    let mut depth = 0;
    for line in code.lines() {
        if let Some(b_pos) = line.find("}") {
            if let Some(comment) = line.find("//") {
                if comment > b_pos {
                    depth -= 1;
                }
            } else {
                depth -= 1;
            }
        }
        writeln!(f, "{1:0$}{2:}", depth * 4, "", line).unwrap();
        if let Some(b_pos) = line.find("{") {
            if let Some(comment) = line.find("//") {
                if comment > b_pos {
                    depth += 1;
                }
            } else {
                depth += 1;
            }
        }
    }
    f
}

peg::parser!(pub grammar parser() for str {
    pub rule program() -> Vec<Declaration>
        = (d:declaration() _ { d })*

    rule declaration() -> Declaration
        = function()
        / metadata()
        / structdef()

    rule structdef() -> Declaration
        = _ ext:("extern")? _ "struct" name:identifier() _ "{" _ fields:(a:arg() comma() {a})* _ "}" _ {Declaration::Struct(Struct{name, fields, extern_struct: if ext.is_some() {true} else {false}})}

    rule metadata() -> Declaration
        = _ "@" _ headings:(i:(metadata_identifier()** ([' ' | '\t'])) {i}) ([' ' | '\t'])* "\n" body:$[^'@']* "@" _ {Declaration::Metadata(headings, body.join(""))}

    rule metadata_identifier() -> String
        = quiet!{ _ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.into() } }
        / expected!("identifier")

    rule function() -> Declaration
        = _ ext:("extern")? _  "fn" name:identifier() _
        "(" params:(i:arg() ** comma()) ")" _
        "->" _
        "(" returns:(i:arg() ** comma()) _ ")"
        body:block()
        {
            let mut name = name;
            if let Some(first_param) = params.first() {
                if first_param.name == "self" {
                    name = format!("{}.{}", first_param.expr_type.as_ref().unwrap(), name)
                    //change func name to struct_name.func_name if first param is self
                }
            }
            Declaration::Function(Function {
            name,
            params,
            returns,
            body,
            extern_func: if ext.is_some() {true} else {false},
        }) }

    rule arg() -> Arg
        = _ i:identifier() _ ":" _ t:type_label() _ { Arg {name: i.into(), expr_type: Some(t.into()) } }
        / _ i:identifier() _ { Arg {name: i.into(), expr_type: None } }

    rule type_label() -> ExprType
        = _ n:$("f64") _ { ExprType::F64 }
        / _ n:$("i64") _ { ExprType::I64 }
        / _ n:$("&[f64]") _ { ExprType::UnboundedArrayF64 }
        / _ n:$("&[i64]") _ { ExprType::UnboundedArrayI64 }
        / _ n:$("&") _ { ExprType::Address }
        / _ n:$("bool") _ { ExprType::Bool }
        / _ n:identifier() _ { ExprType::Struct(Box::new(n)) }

    rule block() -> Vec<Expr>
        = _ "{" b:(statement() ** _) _ "}" { b }

    rule statement() -> Expr
        //TODO allow for multiple expressions like: a, b, c returned from if/then/else, etc...
        = while_loop() / assignment() / expression()

    rule expression() -> Expr
        = if_then()
        / if_else()
        / while_loop()
        / arrayset()
        / assignment()
        / binary_op()

    rule if_then() -> Expr
        = _ "if" _ e:expression() then_body:block() "\n"
        { Expr::IfThen(Box::new(e), then_body) }

    rule if_else() -> Expr
        = _ "if" e:expression() _ when_true:block() _ "else" when_false:block()
        { Expr::IfElse(Box::new(e), when_true, when_false) }

    rule while_loop() -> Expr
        = _ "while" e:expression() body:block()
        { Expr::WhileLoop(Box::new(e), body) }

    rule assignment() -> Expr
        = assignments:((i:var_identifier() {i}) ** comma()) _ "=" args:((_ e:expression() _ {e}) ** comma()) {?
            make_nonempty(assignments)
                .and_then(|assignments| make_nonempty(args)
                .map(|args| Expr::Assign(assignments, args)))
                .ok_or("Cannot assign to/from empty tuple")
        }

    rule arrayset() -> Expr
        = i:var_identifier() _ "[" idx:expression() "]" _ "=" _ e:expression() {Expr::ArraySet(i, Box::new(idx), Box::new(e))}


    rule unary_op() -> Expr = precedence!{
        "!" e:expression() { Expr::Unaryop(Unaryop::Not, Box::new(e)) }
    }

    rule binary_op() -> Expr = precedence!{
        a:@ _ "&&" _ b:(@) { Expr::Binop(Binop::LogicalAnd, Box::new(a), Box::new(b)) }
        a:@ _ "||" _ b:(@) { Expr::Binop(Binop::LogicalOr, Box::new(a), Box::new(b)) }
        --
        a:@ _ "==" b:(@) { Expr::Compare(Cmp::Eq, Box::new(a), Box::new(b)) }
        a:@ _ "!=" b:(@) { Expr::Compare(Cmp::Ne, Box::new(a), Box::new(b)) }
        a:@ _ "<"  b:(@) { Expr::Compare(Cmp::Lt, Box::new(a), Box::new(b)) }
        a:@ _ "<=" b:(@) { Expr::Compare(Cmp::Le, Box::new(a), Box::new(b)) }
        a:@ _ ">"  b:(@) { Expr::Compare(Cmp::Gt, Box::new(a), Box::new(b)) }
        a:@ _ ">=" b:(@) { Expr::Compare(Cmp::Ge, Box::new(a), Box::new(b)) }
        --
        a:@ _ "+" _ b:(@) { Expr::Binop(Binop::Add, Box::new(a), Box::new(b)) }
        i:var_identifier() _ "+=" _ a:(@) { Expr::AssignOp(Binop::Add, Box::new(i), Box::new(a)) }

        a:@ _ "-" _ b:(@) { Expr::Binop(Binop::Sub, Box::new(a), Box::new(b)) }
        i:var_identifier() _ "-=" _ a:(@) { Expr::AssignOp(Binop::Sub, Box::new(i), Box::new(a)) }
        --
        a:@ _ "*" _ b:(@) { Expr::Binop(Binop::Mul, Box::new(a), Box::new(b)) }
        i:var_identifier() _ "*=" _ a:(@) { Expr::AssignOp(Binop::Mul, Box::new(i), Box::new(a)) }

        a:@ _ "/" _ b:(@) { Expr::Binop(Binop::Div, Box::new(a), Box::new(b)) }
        i:var_identifier() _ "/=" _ a:(@) { Expr::AssignOp(Binop::Div, Box::new(i), Box::new(a)) }
        --
        i:var_identifier() _ "(" args:((_ e:expression() _ {e}) ** comma()) ")" {
            if i.contains(".") {
                let mut parts = i.split(".").collect::<Vec<&str>>();
                let mut args = args;
                let func_name = parts.pop().unwrap().to_string();
                args.insert(0, Expr::Identifier(parts.join(".")));
                Expr::Call(func_name, args, true)
            } else {
                Expr::Call(i, args, false)
            }
        }
        i:var_identifier() _ "{" args:((_ e:struct_assign_field() _ {e})*) "}" { Expr::NewStruct(i, args) }
        i:var_identifier() _ "[" idx:expression() "]" { Expr::ArrayGet(i, Box::new(idx)) }
        i:var_identifier() { Expr::Identifier(i) }
        l:literal() { l }
        --
        u:unary_op()  { u }
        --
        "(" e:expression() ")" { Expr::Parentheses(Box::new(e)) }
    }

    rule identifier() -> String
        = quiet!{ _ n:$((!"true"!"false")['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.into() } }
        / expected!("identifier")


    rule var_identifier() -> (String)
        = i:(identifier() ++ ".") {i.join(".")}
        / identifier()

    rule literal() -> Expr
        = _ n:$(['-']*['0'..='9']+"."['0'..='9']+) { Expr::LiteralFloat(n.into()) }
        / _ n:$(['-']*['0'..='9']+) { Expr::LiteralInt(n.into()) }
        / "*" i:identifier() { Expr::GlobalDataAddr(i) }
        / _ "true" _ { Expr::LiteralBool(true) }
        / _ "false" _ { Expr::LiteralBool(false) }
        / _ "\"" body:$[^'"']* "\"" _ { Expr::LiteralString(body.join("")) }
        / _ "[" _ "\"" repstr:$[^'\"']* "\"" _ ";" _ len:$(['0'..='9']+) _ "]" _ {
            //Temp solution for creating empty strings
            Expr::LiteralString(repstr.join("").repeat( len.parse().unwrap()))
        } //[" "; 10]

    rule struct_assign_field() -> StructAssignField
        = _ i:identifier() _ ":" _ e:expression() comma() _ { StructAssignField {field_name: i.into(), expr: e } }

    rule comment() -> ()
        = quiet!{"//" [^'\n']*"\n"}

    rule comma() = _ ","

    rule _() =  quiet!{comment() / [' ' | '\t' | '\n']}*
});
