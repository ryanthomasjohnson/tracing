use super::*;
use tracing_subscriber::{filter, prelude::*, Subscribe};

fn filter<S>() -> filter::DynFilterFn<S> {
    // Use dynamic filter fn to disable interest caching and max-level hints,
    // allowing us to put all of these tests in the same file.
    filter::dynamic_filter_fn(|_, _| false)
}

#[test]
fn option_some() {
    let (subscribe, handle) = subscriber::mock().only().run_with_handle();
    let subscribe = Box::new(subscribe.with_filter(Some(filter())));

    let _guard = tracing_subscriber::registry().with(subscribe).set_default();

    for i in 0..2 {
        tracing::info!(i);
    }

    handle.assert_finished();
}

#[test]
fn option_none() {
    let (subscribe, handle) = subscriber::mock()
        .event(expect::event())
        .event(expect::event())
        .only()
        .run_with_handle();
    let subscribe = Box::new(subscribe.with_filter(None::<filter::DynFilterFn<_>>));

    let _guard = tracing_subscriber::registry().with(subscribe).set_default();

    for i in 0..2 {
        tracing::info!(i);
    }

    handle.assert_finished();
}
