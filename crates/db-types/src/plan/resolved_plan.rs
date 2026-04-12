use crate::dtype::DataTypeKind;

/// A resolved query plan. All references validated against the
/// catalog: tables carry IDs, columns carry indices and types,
/// aggregate functions are typed enum variants.
#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedPlan {
    Select {
        projection: Vec<ResolvedSelectItem>,
        from: ResolvedTable,
        group_by: Vec<ResolvedExpr>,
    },
}

/// A validated aggregate function with resolved inputs and
/// output type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedAggregate {
    pub function: ResolvedAggregateFunction,
    pub data_type: DataTypeKind,
}

/// Concrete aggregate function with resolved column inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedAggregateFunction {
    CountStar,
    Count { column: ResolvedColumn },
    Avg { column: ResolvedColumn },
}

/// A column resolved to its position and type in a table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedColumn {
    pub name: String,
    pub index: usize,
    pub data_type: DataTypeKind,
}

/// An expression with all references resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedExpr {
    Column(ResolvedColumn),
    Aggregate(ResolvedAggregate),
}

/// A projection item with a resolved expression and optional
/// alias.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSelectItem {
    pub expr: ResolvedExpr,
    pub alias: Option<String>,
}

/// A table resolved against the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedTable {
    pub id: u32,
    pub name: String,
    pub columns: Vec<ResolvedColumn>,
}
