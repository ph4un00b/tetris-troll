use macroquad::prelude::*;

struct CRDT<TValue, TState> {
    // * The entire point of the CRDT is to reliably sync the value between peers.
    value: TValue,
    // * This is the metadata needed for peers to agree on the same value.
    // * To update other peers, the whole state is serialized and sent to them.
    state: TState,
}

impl<TValue, TState> CRDT<TValue, TState> {
    /*
     * This is a function that takes some state
     * (probably received from another peer) and merges it with the local state.
     *
     * tests:
     * - Commutativity: states can be merged in any order; A ∨ B = B ∨ A
     * - Associativity: when merging three (or more) states,
     *      it doesn’t matter which are merged first; (A ∨ B) ∨ C = A ∨ (B ∨ C)
     * - Idempotence: merging a state with itself doesn’t change the state; A ∨ A = A
     */
    fn merge(&self, new_state: TState) {
        todo!()
    }
}
// * Last Write Wins Register
enum State<TValue> {
    LWW {
        peer: String,
        timestamp: i64,
        value: TValue,
    },
}

struct LastWriteWin<TValue> {
    id: String,
    // todo: what's better, enum, struct, tuple❓
    state: State<TValue>,
}

impl<TValue> LastWriteWin<TValue> {
    fn new(id: String, state: State<TValue>) -> Self {
        Self { id, state }
    }

    fn value(&self) -> &TValue {
        match &self.state {
            State::LWW {
                peer: _,
                timestamp: _,
                value,
            } => value,
        }
    }

    fn set(&mut self, new_value: TValue) {
        let old_timestamp = match &self.state {
            State::LWW {
                peer: _,
                timestamp,
                value: _,
            } => timestamp,
        };

        self.state = State::LWW {
            peer: self.id.clone(),
            timestamp: old_timestamp + 1,
            value: new_value,
        }
    }

    fn merge(&mut self, remote_state: State<TValue>) {
        let (remote_peer, remote_time, remote_value) = match remote_state {
            State::LWW {
                peer,
                timestamp,
                value,
            } => (peer, timestamp, value),
        };

        let (local_peer, local_time) = match &self.state {
            State::LWW {
                peer,
                timestamp,
                value: _,
            } => (peer, timestamp),
        };
        if local_time > &remote_time {
            // * If the received timestamp is less than the local timestamp,
            // * the register doesn’t change its state.
            return;
        }
        if local_time == &remote_time && local_peer > &remote_peer {
            // * Ties are broken by comparing the local peer ID to the peer ID in the received state.
            return;
        }
        self.state = State::LWW {
            // * If the received timestamp is greater than the local timestamp
            // * the register overwrites its local value with the received value.
            // * It also stores the received timestamp and some sort of identifier
            // * unique to the peer that last wrote the value (the peer ID).
            peer: remote_peer,
            timestamp: remote_time,
            value: remote_value,
        };
    }
}
#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(DARKBLUE);

        next_frame().await
    }
}
