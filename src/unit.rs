use cell::Cell;

enum Unit {
    Column([Cell, ..9]),
    Row([Cell, ..9]),
    Grid([Cell, ..9])
}

impl Unit {
    fn contains(&self, cell: Cell) -> bool {
        let cells = match self {
            &Unit::Column(cells) => cells,
            &Unit::Row(cells) => cells,
            &Unit::Grid(cells) => cells
        };

        false
    }
}
