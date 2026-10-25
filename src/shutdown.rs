use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Default)]
pub struct Shutdown(Arc<AtomicBool>);

impl Shutdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    pub fn is_requested(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    #[cfg(unix)]
    pub fn listen_for_signals(&self) -> io::Result<()> {
        use tokio::signal::unix::{SignalKind, signal};

        let mut interrupt = signal(SignalKind::interrupt())?;
        let mut terminate = signal(SignalKind::terminate())?;
        let mut hangup = signal(SignalKind::hangup())?;
        let shutdown = self.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = interrupt.recv() => {}
                _ = terminate.recv() => {}
                _ = hangup.recv() => {}
            }
            shutdown.request();
        });
        Ok(())
    }

    #[cfg(not(unix))]
    pub fn listen_for_signals(&self) -> io::Result<()> {
        let shutdown = self.clone();
        tokio::spawn(async move {
            if tokio::signal::ctrl_c().await.is_ok() {
                shutdown.request();
            }
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_without_a_request() {
        assert!(!Shutdown::new().is_requested());
    }

    #[test]
    fn request_is_seen_through_every_clone() {
        let shutdown = Shutdown::new();
        let observer = shutdown.clone();
        shutdown.request();
        assert!(observer.is_requested());
    }
}
