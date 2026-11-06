use super::*;

#[test]
fn the_gpu_s_time_is_shared_out_over_the_frames_between_readings() {
    // 90 ms of work over 10 frames: 900 000 ticks of 100 ns.
    assert_eq!(per_frame_ms((1_000_000, 50), (1_900_000, 60)), Some(9.0));
    assert_eq!(per_frame_ms((5, 50), (900, 50)), None, "no frame came");
    assert_eq!(per_frame_ms((5, 50), (900, 40)), None, "a new count");
}

#[test]
fn a_counter_near_its_end_wraps() {
    assert_eq!(per_frame_ms((u64::MAX - 9_999, 0), (10_000, 1)), Some(2.0));
}
