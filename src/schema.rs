table! {
    words (id) {
        id -> Integer,
        word -> Text,
        phonetic -> Nullable<Text>,
        definition -> Nullable<Text>,
        translation -> Nullable<Text>,
        pos -> Nullable<Text>,
        collins -> Nullable<Integer>,
        oxford -> Nullable<Integer>,
        tag -> Nullable<Text>,
        bnc -> Nullable<Integer>,
        frq -> Nullable<Integer>,
        exchange -> Nullable<Text>,
        sentences -> Nullable<Text>,
        audio -> Nullable<Text>,
    }
}
