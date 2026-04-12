/// An unresolved query plan. Column and table references are
/// strings, not yet validated against the catalog. Function
/// names are unchecked text from the SQL source.
#[derive(Debug, PartialEq, Eq)]
pub enum UnresolvedPlan {
    Select {
        projection: Vec<UnresolvedSelectItem>,
        from: String,
        group_by: Vec<UnresolvedExpr>,
    },
}

/// An expression before name resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnresolvedExpr {
    Column(String),
    Function {
        name: String,
        args: UnresolvedFunctionArgs,
    },
}

/// Function arguments before resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnresolvedFunctionArgs {
    Star,
    Exprs(Vec<UnresolvedExpr>),
}

/// A projection item before name resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedSelectItem {
    pub expr: UnresolvedExpr,
    pub alias: Option<String>,
}
