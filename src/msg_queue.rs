use heapless::{consts::*, Vec};
use portable::ui::Msg;

pub struct MsgQueue {
    q: Vec<Msg, U16>,
}
impl MsgQueue {
    pub fn new() -> Self {
        Self { q: Vec::new() }
    }
    pub fn push(&mut self, msg: Msg) {
        if let Err(_) = self.q.push(msg) {
            panic!("msg queue full");
        }
        rtfm::pend(crate::hal::device::Interrupt::EXTI2);
    }
    pub fn get(&mut self) -> Vec<Msg, U16> {
        ::core::mem::replace(&mut self.q, Vec::new())
    }
}
