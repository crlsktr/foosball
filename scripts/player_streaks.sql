/* get the current winning streak */
/* ignores games played in the same series as a loss */
select
    p.id
    ,p.name
    ,SUM(
        case
            when s.id is not null then 1
            else 0
        end
    ) as streak
from players p
join (
    /* getting the last loss of a player */
    select
        player_id
        ,last_loss
    from (
        select
            p.id as player_id
            ,ROW_NUMBER() OVER (PARTITION BY p.id ORDER BY s.played_on desc) AS rowskii
            ,s.played_on as last_loss
        from players p
        join teams t
            on p.id = t.player_one_id
            or p.id = t.player_two_id
        join games g
            on g.team_one_id = t.id
            or g.team_two_id = t.id
        join series s
            on s.id = g.series_id
        where t.id <> g.winners
        group by p.id, s.played_on
    ) l
    where l.rowskii = 1
) ll on ll.player_id = p.id
join teams t
    on p.id = t.player_one_id
    or p.id = t.player_two_id
join games g
    on g.team_one_id = t.id
    or g.team_two_id = t.id
left join series s
    on s.id = g.series_id
    and ll.last_loss < s.played_on
group by p.id, p.name