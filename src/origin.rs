use tokio::sync::mpsc;

pub struct TokioManager<T> {
    tx: Option<mpsc::Sender<T>>,
    rx: mpsc::Receiver<T>,
    res: Option<Vec<T>>,
}

impl<T> TokioManager<T> {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = mpsc::channel::<T>(capacity);
        TokioManager {
            tx: Some(tx),
            rx,
            res: None,
        }
    }

    pub fn spawn_transmitter(&self) -> mpsc::Sender<T> {
        if let Some(tx) = self.tx.as_ref() {
            tx.clone()
        } else {
            panic!("TokioManager is closed");
        }
    }

    pub fn close_transmission(&mut self) {
        self.tx = None;
    }

    pub async fn fetch_results(&mut self) -> &Vec<T> {
        if self.res.is_none() {
            let mut v = Vec::new();
            while let Some(msg) = self.rx.recv().await {
                v.push(msg);
            }
            self.res = Some(v);
        }
        self.res.as_ref().unwrap()
    }
}
