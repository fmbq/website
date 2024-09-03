use once_cell::sync::Lazy;
use poem::web::sse::{Event, SSE};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

static CHANNEL: Lazy<(Sender<Event>, Receiver<Event>)> = Lazy::new(|| broadcast::channel(128));

pub fn publish(event: Event) {
    CHANNEL.0.send(event).unwrap();
}

pub fn subscribe() -> SSE {
    SSE::new(BroadcastStream::new(CHANNEL.0.subscribe()).filter_map(|result| result.ok()))
}
