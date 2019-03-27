table! {
    access_tokens (id) {
        id -> Int8,
        user_id -> Int8,
        token -> Varchar,
    }
}

table! {
    accounts (id) {
        id -> Int8,
        nama -> Text,
        email -> Text,
        alamat -> Text,
        password -> Varchar,
    }
}

table! {
    articles (id) {
        id -> Int8,
        judul -> Text,
        konten -> Text,
        waktu -> Timestamp,
        penulis -> Text,
    }
}

joinable!(access_tokens -> accounts (user_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    accounts,
    articles,
);
