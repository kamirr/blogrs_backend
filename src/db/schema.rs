table! {
    nonrepeating (id) {
        id -> Varchar,
        value -> Text,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        title -> Text,
        body -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    nonrepeating,
    posts,
);
