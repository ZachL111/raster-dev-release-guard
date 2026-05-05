use raster_dev_release_guard::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 79, capacity: 89, latency: 13, risk: 12, weight: 13 };
    assert_eq!(score(signal), 163);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 82, capacity: 95, latency: 9, risk: 24, weight: 8 };
    assert_eq!(score(signal), 87);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 85, capacity: 70, latency: 12, risk: 22, weight: 9 };
    assert_eq!(score(signal), 74);
    assert_eq!(classify(signal), "review");
}
