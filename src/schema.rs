table! {
    accounts (id) {
        id -> Int8,
        nama -> Text,
        email -> Text,
        alamat -> Text,
    }
}

table! {
    articles (id) {
        id -> Int8,
        judul -> Text,
        konten -> Text,
        waktu -> Text,
        penulis -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    articles,
);
