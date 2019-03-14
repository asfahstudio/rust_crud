table! {
    accounts (id) {
        id -> Int8,
        nama -> Text,
        email -> Text,
        alamat -> Text,
    }
}

table! {
    artikels (id_artikels) {
        id_artikels -> Int8,
        judul -> Text,
        konten -> Text,
        uploaded -> Timestamp,
        writer -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    artikels,
);
