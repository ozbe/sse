// Seeded from https://github.com/launchdarkly/rust-eventsource-client/blob/master/examples/tail.rs
use std::time::Duration;

use futures::{future::Future, lazy, stream::Stream};

use eventsource_client as es;

fn main() -> Result<(), es::Error> {
    let url = "http://127.0.0.1:3030/ticks";

    let client = es::Client::for_url(url)?
        .reconnect(
            es::ReconnectOptions::reconnect(true)
                .retry_initial(false)
                .delay(Duration::from_secs(1))
                .backoff_factor(2)
                .delay_max(Duration::from_secs(60))
                .build(),
        )
        .build();
    tokio::run(lazy(|| tail_events(client)));
    Ok(())
}

fn tail_events(mut client: es::Client) -> impl Future<Item = (), Error = ()> {
    client
        .stream()
        .for_each(|event| {
            println!("got an event: {:?}", event);
            Ok(())
        })
        .map_err(|err| eprintln!("error streaming events: {:?}", err))
}
