use crate::dtype::DataTypeKind;

const PLAN_DEPTH_LIMIT: usize = 4;

/// A tree of relational algebra operators.
///
/// Each node produces a well-defined output schema. Child nodes
/// are referenced via `Box<LogicalPlan>`. Column references
/// within a node use positional indices into the child's output
/// schema.
///
/// Plan tree depth is bounded by construction: `build_plan`
/// produces trees of at most 3 levels
/// (Projection -> Aggregate -> TableScan).
#[derive(Debug, PartialEq, Eq)]
pub enum LogicalPlan {
    /// Leaf node: reads specific columns from a table.
    TableScan {
        table_id: u32,
        table_name: String,
        columns: Vec<ScanColumn>,
    },

    /// Selects, reorders, and aliases columns from its input.
    Projection {
        expressions: Vec<ProjectionExpr>,
        input: Box<LogicalPlan>,
    },

    /// Groups rows by key columns and computes aggregate
    /// functions. Output schema: group keys (in order) then
    /// aggregate results (in order).
    Aggregate {
        group_keys: Vec<ColumnRef>,
        aggregates: Vec<AggregateExpr>,
        input: Box<LogicalPlan>,
    },
}

impl LogicalPlan {
    /// Iteratively computes the output schema by walking the
    /// plan chain bottom-up. Bounded by PLAN_DEPTH_LIMIT.
    pub fn output_columns(&self) -> Vec<OutputColumn> {
        let chain = collect_plan_chain(self);
        let mut columns = Vec::new();

        for node in chain.into_iter().rev() {
            columns = match node {
                LogicalPlan::TableScan {
                    columns: scan_columns,
                    ..
                } => output_columns_table_scan(scan_columns),
                LogicalPlan::Projection { expressions, .. } => {
                    output_columns_projection(expressions, &columns)
                }
                LogicalPlan::Aggregate {
                    group_keys,
                    aggregates,
                    ..
                } => output_columns_aggregate(group_keys, aggregates, &columns),
            };
        }

        columns
    }
}

/// An aggregate function application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregateExpr {
    pub function: AggregateFunction,
}

impl AggregateExpr {
    pub fn output_name(&self, input_columns: &[OutputColumn]) -> String {
        match self.function {
            AggregateFunction::CountStar => "count".to_owned(),
            AggregateFunction::Count { input } => {
                format!("count({})", input_columns[input.index].name)
            }
            AggregateFunction::Avg { input } => {
                format!("avg({})", input_columns[input.index].name)
            }
        }
    }

    pub fn output_type(&self) -> DataTypeKind {
        match self.function {
            AggregateFunction::CountStar | AggregateFunction::Count { .. } => DataTypeKind::U64,
            AggregateFunction::Avg { .. } => DataTypeKind::F64,
        }
    }
}

/// Concrete aggregate function with input column references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateFunction {
    CountStar,
    Count { input: ColumnRef },
    Avg { input: ColumnRef },
}

/// Reference to a column in a plan node's output, by position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnRef {
    pub index: usize,
}

/// Metadata about a column in a plan node's output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputColumn {
    pub name: String,
    pub data_type: DataTypeKind,
}

/// An expression within a projection node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionExpr {
    pub input: ColumnRef,
    pub alias: Option<String>,
}

/// A column read from a table during a scan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanColumn {
    pub name: String,
    pub table_index: usize,
    pub data_type: DataTypeKind,
}

/// Collects plan nodes from root to leaf. Bounded iteration
/// prevents unbounded traversal if the tree is malformed.
fn collect_plan_chain(root: &LogicalPlan) -> Vec<&LogicalPlan> {
    let mut chain = Vec::with_capacity(PLAN_DEPTH_LIMIT);
    let mut current = root;

    for _ in 0..PLAN_DEPTH_LIMIT {
        chain.push(current);
        match current {
            LogicalPlan::TableScan { .. } => return chain,
            LogicalPlan::Projection { input, .. } | LogicalPlan::Aggregate { input, .. } => {
                current = input;
            }
        }
    }

    panic!("plan tree exceeds depth limit of {PLAN_DEPTH_LIMIT}");
}

fn output_columns_aggregate(
    group_keys: &[ColumnRef],
    aggregates: &[AggregateExpr],
    input_columns: &[OutputColumn],
) -> Vec<OutputColumn> {
    let capacity = group_keys.len() + aggregates.len();
    let mut output = Vec::with_capacity(capacity);

    for key in group_keys {
        output.push(input_columns[key.index].clone());
    }
    for aggregate in aggregates {
        output.push(OutputColumn {
            name: aggregate.output_name(input_columns),
            data_type: aggregate.output_type(),
        });
    }

    output
}

fn output_columns_projection(
    expressions: &[ProjectionExpr],
    input_columns: &[OutputColumn],
) -> Vec<OutputColumn> {
    expressions
        .iter()
        .map(|projection| {
            let source = &input_columns[projection.input.index];
            OutputColumn {
                name: projection
                    .alias
                    .clone()
                    .unwrap_or_else(|| source.name.clone()),
                data_type: source.data_type,
            }
        })
        .collect()
}

fn output_columns_table_scan(columns: &[ScanColumn]) -> Vec<OutputColumn> {
    columns
        .iter()
        .map(|scan_column| OutputColumn {
            name: scan_column.name.clone(),
            data_type: scan_column.data_type,
        })
        .collect()
}
