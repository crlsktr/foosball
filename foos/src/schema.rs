table! {
    games (id) {
        id -> Int4,
        series_id -> Int4,
        team_one_id -> Int4,
        team_two_id -> Int4,
        winners -> Nullable<Int4>,
        spread -> Nullable<Int2>,
        recorded_by -> Nullable<Int4>,
    }
}

table! {
    player_rankings (id) {
        id -> Int4,
        player_id -> Int4,
        current_ranking -> Int4,
        change -> Int4,
        game_id -> Int4,
    }
}

table! {
    players (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        name -> Varchar,
        ranking -> Int4,
        created_by -> Int4,
    }
}

table! {
    series (id) {
        id -> Int4,
        played_on -> Timestamptz,
        created_by -> Int4,
    }
}

table! {
    team_rankings (id) {
        id -> Int4,
        team_id -> Int4,
        current_ranking -> Int4,
        change -> Int4,
        game_id -> Int4,
    }
}

table! {
    teams (id) {
        id -> Int4,
        player_one_id -> Int4,
        player_two_id -> Int4,
        ranking -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        enabled -> Bool,
    }
}

joinable!(games -> series (series_id));
joinable!(games -> users (recorded_by));
joinable!(player_rankings -> games (game_id));
joinable!(player_rankings -> players (player_id));
joinable!(series -> users (created_by));
joinable!(team_rankings -> games (game_id));
joinable!(team_rankings -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    games,
    player_rankings,
    players,
    series,
    team_rankings,
    teams,
    users,
);
