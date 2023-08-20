pub struct Table {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<Cell>>,
}

pub enum ExprKind {
    ExprKindNumber,
    ExprKindCell,
}

pub struct Expr {
    pub kind: ExprKind,
}

pub enum CellKind {
    Expr(String),
    Text(String),
    Num(f64),
}

pub struct Cell {
    pub kind: CellKind,
}

impl Cell {
    pub fn new(kind: CellKind) -> Self {
        Cell { kind }
    }

    pub fn get_val(&self) -> &self::CellKind {
        &self.kind
    }
}
