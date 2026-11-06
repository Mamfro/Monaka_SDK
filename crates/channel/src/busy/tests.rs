use super::*;

#[test]
fn work_said_on_one_side_reaches_the_other_until_it_ends() {
    let name = ChannelName::new(format!(r"Local\monaka-test-busy-{}", std::process::id())).unwrap();
    let mut producer = BusyChannel::open(&name).unwrap();
    let mut viewer = BusyChannel::open(&name).unwrap();
    assert_eq!(viewer.read(), None);
    producer.report("Preparing shaders", Amount::Known { done: 40, total: 100 });
    assert_eq!(viewer.read(), Some(Busy { text: "Preparing shaders".to_owned(), amount: Amount::Known { done: 40, total: 100 } }));
    producer.report("Loading", Amount::Unknown);
    assert_eq!(viewer.read(), Some(Busy { text: "Loading".to_owned(), amount: Amount::Unknown }));
    producer.clear();
    assert_eq!(viewer.read(), None);
}
