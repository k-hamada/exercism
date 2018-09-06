pub struct ChessPosition {
    file: i8,
    rank: i8
}

impl ChessPosition {
    pub fn new(file: i8, rank: i8) -> Result<ChessPosition, ()> {
        if file < 0 || 8 <= file || rank < 0 || 8 <= rank {
            return Err(());
        }

        Ok(ChessPosition { file, rank })
    }
}

pub struct Queen {
    position: ChessPosition
}

impl Queen {
    pub fn new(position: ChessPosition) -> Queen {
        Queen { position }
    }

    pub fn can_attack(self, other: &Queen) -> bool {
        if self.position.file == other.position.file { return true; }
        if self.position.rank == other.position.rank { return true; }
        if (self.position.file - self.position.rank) == (other.position.file - other.position.rank) { return true; }
        if (self.position.file + self.position.rank) == (other.position.file + other.position.rank) { return true; }

        return false;
    }
}
