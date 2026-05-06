use raster_dev_release_guard::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 49, slack: 31, drag: 24, confidence: 49 };
    assert_eq!(review_score(case), 106);
    assert_eq!(review_lane(case), "watch");
}
