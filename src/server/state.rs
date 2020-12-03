use std::{
    time::Duration,
    thread,
};

struct StateReq<S> {
    state: S
}
struct StateRes;

pub trait State {
    type Data: Data;
    fn get(self) -> Self::Data;
}

pub trait Data {
    fn data(self);
}

impl<S> Data for S where S: State {
    fn data(self) {
        self.get().data();
    }
}

impl<S> State for StateReq<S> {
    type Data = StateRes;

    fn get(self) -> Self::Data {
        unimplemented!()
    }
}

impl Data for StateRes {
    fn data(self) {
        unimplemented!()
    }
}
