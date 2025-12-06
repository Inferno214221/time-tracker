// @generated automatically by Diesel CLI.

diesel::table! {
    invoice (inv_num) {
        inv_num -> Integer,
        inv_month -> Date,
        recip_id -> Text,
        inv_created -> Nullable<Date>,
    }
}

diesel::table! {
    invoice_activity (act_num) {
        act_num -> Integer,
        inv_num -> Integer,
        act_desc -> Text,
        act_uprice -> Double,
    }
}

diesel::table! {
    project (proj_key) {
        proj_key -> Text,
        proj_name -> Text,
    }
}

diesel::table! {
    recipient (recip_id) {
        recip_id -> Text,
        recip_name -> Text,
        recip_addr -> Text,
    }
}

diesel::table! {
    ticket (proj_key, tick_num) {
        proj_key -> Text,
        tick_num -> Integer,
    }
}

diesel::table! {
    ticket_time (proj_key, tick_num, time_id) {
        proj_key -> Text,
        tick_num -> Integer,
        time_id -> Integer,
    }
}

diesel::table! {
    time (time_id) {
        time_id -> Integer,
        time_start -> Timestamp,
        time_end -> Timestamp,
        time_desc -> Text,
        time_dur -> Nullable<Double>,
        act_num -> Nullable<Integer>,
    }
}

diesel::joinable!(invoice -> recipient (recip_id));
diesel::joinable!(invoice_activity -> invoice (inv_num));
diesel::joinable!(ticket -> project (proj_key));
diesel::joinable!(ticket_time -> project (proj_key));
diesel::joinable!(ticket_time -> time (time_id));
diesel::joinable!(time -> invoice_activity (act_num));

diesel::allow_tables_to_appear_in_same_query!(
    invoice,
    invoice_activity,
    project,
    recipient,
    ticket,
    ticket_time,
    time,
);
