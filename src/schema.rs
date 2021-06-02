table! {
    category (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    data_status (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    label (id) {
        id -> Int4,
        name -> Varchar,
        color_hex -> Nullable<Varchar>,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    list (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    todo (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        start_at -> Timestamptz,
        end_at -> Timestamptz,
        id_category -> Nullable<Int4>,
        id_label -> Nullable<Int4>,
        id_list -> Nullable<Int4>,
        id_data_status -> Nullable<Int4>,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

joinable!(todo -> category (id_category));
joinable!(todo -> data_status (id_data_status));
joinable!(todo -> label (id_label));
joinable!(todo -> list (id_list));

allow_tables_to_appear_in_same_query!(
    category,
    data_status,
    label,
    list,
    todo,
);
