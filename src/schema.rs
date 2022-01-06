table! {
    tasks (id) {
        id -> Varchar,
        title -> Varchar,
        descp -> Text,
        completed -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
);
