use super::*;

fn block() -> BusyBlock {
    let mut block = BusyBlock { stamp: Stamp::default(), active: 0, done: 0, total: 0, text_len: 0, at: 0, text: [0; BUSY_TEXT_BYTES] };
    block.start();
    block
}

#[test]
fn nothing_goes_on_until_it_is_said() {
    assert_eq!(block().read(5), None);
}

#[test]
fn known_work_says_how_far_along_and_unknown_work_cannot() {
    let mut record = block();
    record.report("Preparing shaders", Amount::Known { done: 3, total: 12 }, 100);
    let busy = record.read(100).unwrap();
    assert_eq!(busy.text, "Preparing shaders");
    assert_eq!(busy.amount.fraction(), Some(0.25));
    record.report("Loading", Amount::Unknown, 200);
    let busy = record.read(200).unwrap();
    assert_eq!((busy.text.as_str(), busy.amount), ("Loading", Amount::Unknown));
    assert_eq!(busy.amount.fraction(), None);
}

#[test]
fn more_done_than_the_total_is_all_of_it() {
    assert_eq!(Amount::Known { done: 9, total: 4 }.fraction(), Some(1.0));
}

#[test]
fn work_ends_when_said_or_when_its_producer_goes_quiet() {
    let mut record = block();
    record.report("x", Amount::Unknown, 100);
    assert!(record.read(100 + BUSY_FRESH_MS).is_some());
    assert_eq!(record.read(101 + BUSY_FRESH_MS), None);
    record.clear();
    assert_eq!(record.read(100), None);
}

#[test]
fn long_text_is_cut_at_a_character() {
    let mut record = block();
    record.report(&"é".repeat(BUSY_TEXT_BYTES), Amount::Unknown, 0);
    assert_eq!(record.read(0).unwrap().text, "é".repeat(BUSY_TEXT_BYTES / 2));
    record.report(&format!("x{}", "é".repeat(BUSY_TEXT_BYTES)), Amount::Unknown, 0);
    assert_eq!(record.read(0).unwrap().text.len(), BUSY_TEXT_BYTES - 1);
}
