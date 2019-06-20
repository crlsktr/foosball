table! {
    games (id) {
        id -> Integer,
        match_id -> Integer,
        team_one_id -> Integer,
        team_two_id -> Integer,
    }
}

table! {
    matches (id) {
        id -> Integer,
        is_gauntlet -> Bool,
        date -> Text,
    }
}

table! {
    players (id) {
        id -> Integer,
        name -> Text,
        ranking -> Integer,
    }
}

table! {
    results (game_id) {
        game_id -> Integer,
        winning_team -> Integer,
        spread -> Integer,
    }
}

table! {
    teams (id) {
        id -> Integer,
        player_one_id -> Integer,
        player_two_id -> Integer,
    }
}

joinable!(games -> matches (match_id));
joinable!(results -> games (game_id));

allow_tables_to_appear_in_same_query!(
    games,
    matches,
    players,
    results,
    teams,
);
