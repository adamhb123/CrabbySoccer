# CrabbySoccer
Databases Project

## soccer.csv
This file was downloaded from [this kaggle repository](https://www.kaggle.com/code/desalegngeb/english-premier-league-players-statistics/input) on 03/20/24. The original file name was "dataset - 2020-09-24.csv".

### Translation
The csv is translated into the db schema as follows, by table:

* <b>NOTE: </b> table keys (i.e., id) and other attributes not
    taken from the CSV are not included below. See the
    DB Schema section for full table definitions

<b>club</b>
| csv column name | table attribute |
|-----------------|-----------------|
| Club            | club.name       |

<b>player</b>
| csv column name | table attribute                                                 |
|-----------------|-----------------------------------------------------------------|
| Name            | player.name                                                     |
| Jersey Number   | player.jersey_number                                            |
| Position        | ternary relationship (see player_position in DB Schema section) |
| Nationality     | player.nationality                                              |
| Age             | player.age                                                      |

<b>statistics</b>
| csv column name       | table attribute             |
|-----------------------|-----------------------------|
| Appearances           | statistics.appearances      |
| Wins                  | statistics.wins             |
| Losses                | statistics.wins             |
| Goals                 | statistics.wins             |
| Goals per match       | statistics.goals_per_match  |
| Headed goals          | statistics.headed_goals     |
| Goals with right foot | statistics.goals_right_foot |
| Goals with left foot  | statistics.goals_left_foot  |


statistics.goals_left_foot # Goals with left foot
statistics.goals_from_penalties # Penalties scored
statistics.goals_from_freekicks # Freekicks scored
statistics.shots # Shots
statistics.shots_on_target # Shots on target
statistics.shooting_accuracy_pct # Shooting accuracy %
statistics.hit_woodwork # Hit woodwork
[EXCLUDED] ### Big chances missed
statistics.clean_sheets* # Clean sheets
statistics.goals_conceded*? # Goals conceded
statistics.tackles # Tackles
statistics.tackle_success_pct # Tackle success %
[EXCLUDED] ### Last man tackles
Blocked shots
Interceptions
Clearances
Headed Clearance
Clearances off line
Recoveries
Duels won
Duels lost
Successful 50/50s
Aerial battles won
Aerial battles lost
Own goals
Errors leading to goal
Assists
Passes
Passes per match
Big chances created
Crosses
Cross accuracy %
Through balls
Accurate long balls
Saves
Penalties saved
Punches
High Claims
Catches
Sweeper clearances
Throw outs
Goal Kicks
Yellow cards
Red cards
Fouls
Offsides


