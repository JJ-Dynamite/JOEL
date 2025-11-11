#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    Member {
        object: Box<Expr>,
        member: String,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    None,
    // Pattern matching
    Match {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    // Destructuring
    Destructure {
        pattern: Box<Pattern>,
        value: Box<Expr>,
    },
    // Async/await
    Async {
        body: Box<Expr>,
    },
    Await {
        expr: Box<Expr>,
    },
    // Generators
    Yield(Option<Box<Expr>>),
    Generator {
        body: Vec<Stmt>,
    },
    // Coroutines
    Coroutine {
        body: Vec<Stmt>,
    },
    Suspend,
    Resume {
        coroutine: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        type_annot: Option<String>,
        value: Expr,
    },
    Const {
        name: String,
        type_annot: Option<String>,
        value: Expr,
    },
    Expr(Expr),
    Print(Expr),
    Return(Option<Expr>),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    Block(Vec<Stmt>),
    Fn {
        name: String,
        params: Vec<(String, Option<String>)>,
        return_type: Option<String>,
        body: Vec<Stmt>,
    },
    Import {
        module: String,
        alias: Option<String>,
    },
    Module {
        name: String,
    },
    Actor {
        name: String,
        fields: Vec<(String, Option<String>, Expr)>,
        methods: Vec<Stmt>,
    },
    Contract {
        name: String,
        fields: Vec<(String, Option<String>, Expr)>,
        methods: Vec<Stmt>,
    },
    Component {
        name: String,
        body: Vec<Stmt>,
    },
    Flow {
        name: String,
        nodes: Vec<Stmt>,
    },
    Deployment {
        name: String,
        config: Vec<Stmt>,
    },
    Cluster {
        name: String,
        config: Vec<Stmt>,
    },
    // Pattern matching statement
    MatchStmt {
        expr: Expr,
        arms: Vec<MatchArm>,
    },
    // Async function
    AsyncFn {
        name: String,
        params: Vec<(String, Option<String>)>,
        return_type: Option<String>,
        body: Vec<Stmt>,
    },
    // Parallel execution
    ParallelFor {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    ParallelMap {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    // Coroutine
    CoroutineFn {
        name: String,
        params: Vec<(String, Option<String>)>,
        return_type: Option<String>,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub mode: ExecutionMode,
    pub target: Option<String>,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionMode {
    Compiled,
    Interpreted,
    Unknown,
}

/// Pattern for pattern matching and destructuring
#[derive(Debug, Clone)]
pub enum Pattern {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Tuple(Vec<Pattern>),
    List(Vec<Pattern>),
    Struct {
        name: String,
        fields: Vec<(String, Pattern)>,
    },
    Wildcard,
    Binding {
        name: String,
        pattern: Box<Pattern>,
    },
    Or(Vec<Pattern>),
    Guard {
        pattern: Box<Pattern>,
        condition: Box<Expr>,
    },
}

/// Match arm for pattern matching
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Stmt>,
}

