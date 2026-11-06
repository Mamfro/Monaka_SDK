use super::*;

#[test]
fn only_this_process_s_3d_engine_counts() {
    let name = "pid_123_luid_0x00000000_0x000100b1_phys_0_eng_0_engtype_3D";
    assert!(is_3d_engine_of(name, 123));
    assert!(is_3d_engine_of(&name.to_lowercase(), 123));
    assert!(!is_3d_engine_of(name, 12));
    assert!(!is_3d_engine_of(name, 1234));
    assert!(!is_3d_engine_of("pid_123_luid_0x00000000_0x000100b1_phys_0_eng_5_engtype_Copy", 123));
    assert!(!is_3d_engine_of("_Total", 123));
}

#[test]
fn the_clock_opens_and_reads() {
    // A machine without the counters has no clock: nothing to read then.
    if let Some(mut clock) = GpuClock::open(std::process::id()) {
        assert!(clock.read().is_some());
    }
}
