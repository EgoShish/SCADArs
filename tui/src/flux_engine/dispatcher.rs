use tokio::{
    net::TcpStream,
    sync::{
        broadcast,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
};

pub struct Dispatcher {
    store: Store,
    store_tx: UnboundedSender<Store>,
}

impl Dispatcher {
    pub fn new() -> (Self, UnboundedReceiver<Store>) {
        let (store_tx, store_rx) = mpsc::unbounded_channel::<Store>();
        let store = Store::default();

        (Dispatcher { store, store_tx }, store_rx)
    }
    pub fn resend() {
        self.store_tx.send(store.clone())?;
    }
    pub fn dispatch(&mut self, action: Action) {
        
    }
}