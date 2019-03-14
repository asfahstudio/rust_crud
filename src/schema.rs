table! {
    accounts (id) {
        id -> Int8,
        nama -> Text,
        email -> Text,
        alamat -> Text,
    }
}

table! {
    article (id) {
        id -> Int8,
        judul -> Text,
        konten -> Text,
        penulis -> Text,
        published -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    article,
);
