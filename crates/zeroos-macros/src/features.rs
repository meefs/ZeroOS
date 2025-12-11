#[macro_export]
macro_rules! require_exactly_one_feature {
    ($($feat:literal),+ $(,)?) => {
        const _: () = {
            let count = 0 $(+ cfg!(feature = $feat) as usize)+;
            assert!(count == 1, concat!(
                "Exactly one feature must be enabled: ",
                $($feat, ", "),+
            ));
        };
    };
}

#[macro_export]
macro_rules! require_at_most_one_feature {
    ($($feat:literal),+ $(,)?) => {
        const _: () = {
            let count = 0 $(+ cfg!(feature = $feat) as usize)+;
            assert!(count <= 1, concat!(
                "At most one feature can be enabled: ",
                $($feat, ", "),+
            ));
        };
    };
}
