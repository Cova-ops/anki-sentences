#[cfg(test)]
mod test_review_state {
    use insta::assert_debug_snapshot;

    use crate::helpers::review_state::ReviewState;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn review_state_new_defaults() {
        let s = ReviewState::new();

        // Asserts "clásicos"
        assert_eq!(s.interval, 1);
        assert_eq!(s.repetitions, 0);
        assert!(approx(s.ease_factor, 2.5));

        // Snapshot con valores redondeados
        assert_debug_snapshot!("new_defaults", s.rounded());
    }

    #[test]
    fn review_state_from_clamps_values() {
        let s = ReviewState::from(0, 1.0, 5);

        // interval y ease_factor se clampean
        assert_eq!(s.interval, 1);
        assert!(approx(s.ease_factor, 1.3));
        assert_eq!(s.repetitions, 5);

        assert_debug_snapshot!("from_clamped", s.rounded());
    }

    #[test]
    fn review_fail_resets_interval_and_reps() {
        let s = ReviewState::from(10, 2.0, 3);
        let out = s.review(0); // quality < 2 → fallo

        assert_eq!(out.repetitions, 0);
        assert_eq!(out.interval, 1);
        assert!(out.ease_factor >= 1.3);

        assert_debug_snapshot!("fail_review", out.rounded());
    }

    #[test]
    fn review_fail_never_drops_below_min_ease_factor() {
        let s = ReviewState::from(5, 1.3, 10);
        let out = s.review(0);

        assert_eq!(out.repetitions, 0);
        assert_eq!(out.interval, 1);
        assert!(approx(out.ease_factor, 1.3));

        assert_debug_snapshot!("fail_review_min_ef", out.rounded());
    }

    #[test]
    fn first_three_successes_interval_sequence() {
        let mut s = ReviewState::new();
        let mut log = Vec::new();

        // 1er éxito
        s = s.review(3);
        log.push(s.rounded());
        assert_eq!(s.repetitions, 1);
        assert_eq!(s.interval, 1);

        // 2º éxito
        s = s.review(3);
        log.push(s.rounded());
        assert_eq!(s.repetitions, 2);
        assert_eq!(s.interval, 2);

        // 3er éxito
        s = s.review(3);
        log.push(s.rounded());
        assert_eq!(s.repetitions, 3);
        assert_eq!(s.interval, 4);

        assert_debug_snapshot!("first_three_successes", log);
    }

    #[test]
    fn interval_growth_after_third_success() {
        // simulamos que ya va en interval=4, reps=3, ef=2.5
        let s = ReviewState::from(4, 2.5, 3);
        let out = s.review(3);

        // interval = round(4 * 2.5) = 10
        assert_eq!(out.repetitions, 4);
        assert_eq!(out.interval, 10);

        assert_debug_snapshot!("interval_growth_after_third", out.rounded());
    }

    #[test]
    fn full_review_story_log() {
        let mut s = ReviewState::new();
        let mut log = Vec::new();

        // historia realista: fallo, bien, bien, difícil, bien
        s = s.review(0);
        log.push(s.rounded());

        s = s.review(3);
        log.push(s.rounded());

        s = s.review(3);
        log.push(s.rounded());

        s = s.review(2);
        log.push(s.rounded());

        s = s.review(3);
        log.push(s.rounded());

        assert_eq!(log.len(), 5);

        assert_debug_snapshot!("full_review_story", log);
    }
}
