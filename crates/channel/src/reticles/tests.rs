use super::*;

#[test]
fn a_reticle_travels_and_goes_stale() {
    let name = ChannelName::new(format!(r"Local\monaka-test-reticles-{}", std::process::id())).unwrap();
    let mut producer = ReticleChannel::open(&name).unwrap();
    let mut viewer = ReticleChannel::open(&name).unwrap();
    assert_eq!(viewer.read(0, 1000), None);
    assert!(producer.write(0, Some([0.0, 0.3, -0.95])));
    assert_eq!(viewer.read(0, 1000), Some([0.0, 0.3, -0.95]));
    assert_eq!(viewer.read(1, 1000), None);
    assert!(producer.write(0, None));
    assert_eq!(viewer.read(0, 1000), None);
}

#[test]
fn what_the_game_says_of_a_hand_travels_apart_from_where_it_points() {
    let name = ChannelName::new(format!(r"Local\monaka-test-reticles-shown-{}", std::process::id())).unwrap();
    let mut producer = ReticleChannel::open(&name).unwrap();
    let mut viewer = ReticleChannel::open(&name).unwrap();
    assert_eq!(viewer.shown(1), None, "a game that says nothing");
    assert!(producer.show(1, Some(false)));
    assert_eq!(viewer.shown(1), Some(false));
    assert_eq!(viewer.shown(0), None, "the other hand");
    assert!(producer.write(1, Some([0.0, 0.3, -0.95])));
    assert_eq!(viewer.shown(1), Some(false), "kept when it points elsewhere");
    assert!(producer.show(1, Some(true)));
    assert_eq!((viewer.shown(1), viewer.read(1, 1000)), (Some(true), Some([0.0, 0.3, -0.95])), "and the other way round");
    assert!(producer.write(1, None));
    assert_eq!(viewer.shown(1), Some(true));
    assert!(producer.show(1, None));
    assert_eq!(viewer.shown(1), None);
    assert!(!producer.show(2, Some(true)));
}
