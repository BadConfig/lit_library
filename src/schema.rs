table! {
    books (id) {
        id -> Int8,
        title -> Varchar,
    }
}

table! {
    booksasinged (id) {
        id -> Int8,
        book_id -> Int8,
        class_id -> Int8,
    }
}

table! {
    child (id) {
        id -> Int8,
        name -> Varchar,
        last_name -> Varchar,
        very_last_name -> Varchar,
        login -> Varchar,
        pass_hash -> Varchar,
        register_data -> Timestamp,
    }
}

table! {
    childandclass (id) {
        id -> Int8,
        chid_id -> Int8,
        class_id -> Int8,
    }
}

table! {
    classes (id) {
        id -> Int8,
        name -> Varchar,
    }
}

table! {
    takenbooks (id) {
        id -> Int8,
        book_id -> Int8,
        child_id -> Int8,
    }
}

table! {
    teacher (id) {
        id -> Int8,
        name -> Varchar,
        last_name -> Varchar,
        very_last_name -> Varchar,
        login -> Varchar,
        pass_hash -> Varchar,
        register_data -> Timestamp,
    }
}

table! {
    yearending (id) {
        id -> Int8,
        is_ending -> Bool,
    }
}

joinable!(booksasinged -> books (book_id));
joinable!(booksasinged -> classes (class_id));
joinable!(childandclass -> child (chid_id));
joinable!(childandclass -> classes (class_id));
joinable!(takenbooks -> books (book_id));
joinable!(takenbooks -> child (child_id));

allow_tables_to_appear_in_same_query!(
    books,
    booksasinged,
    child,
    childandclass,
    classes,
    takenbooks,
    teacher,
    yearending,
);
