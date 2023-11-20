type Id = String;

/// Implements a (C)onflict-free con(v)ergent (R)eplicated (D)ata (T)ype
/// Also known as "state-based" CRDTs
trait CvRDT<T> {
    fn merge(&mut self, state: State<T>);
}

struct State<T> {
    peer: Id,
    timestamp: std::time::Instant,
    value: T,
}

/// Last Write Wins Register is a
/// very simple state-based CRDT
struct LWWRegister<T> {
    id: Id,
    state: State<T>,
}

impl<T> LWWRegister<T> {
    fn new(id: String, state: State<T>) -> Self {
        Self { id, state }
    }

    fn get_value(&self) -> &T {
        &self.state.value
    }

    fn set_value(&mut self, value: T) {
        self.state = State {
            peer: self.id.clone(),
            timestamp: std::time::Instant::now(),
            value,
        }
    }
}

impl<T> CvRDT<T> for LWWRegister<T> {
    fn merge(&mut self, state: State<T>) {
        let remote_peer = &state.peer;
        let remote_timestamp = state.timestamp;
        let local_peer = &self.state.peer;
        let local_timestamp = self.state.timestamp;

        if local_timestamp > remote_timestamp {
            return;
        }

        if local_timestamp == remote_timestamp && local_peer > remote_peer {
            return;
        }

        self.state = state;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
