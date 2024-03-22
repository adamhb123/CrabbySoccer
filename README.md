# CrabbySoccer
Databases Project

## soccer.csv
This file was downloaded from [this kaggle repository](https://www.kaggle.com/code/desalegngeb/english-premier-league-players-statistics/input) on 03/20/24. The original file name was "dataset - 2020-09-24.csv".

### Translation
The csv is translated into the db schema as follows, by table:

* <b>NOTE: </b> table keys (i.e., id) and other attributes not
    taken from the CSV are not included below. See the
    DB Schema section for full table definitions


<b>player</b>
| csv column name | table attribute                                                                                                        |
|-----------------|------------------------------------------------------------------------------------------------------------------------|
| Name            | player.name                                                                                                            |
| Jersey Number   | player.jersey_number                                                                                                   |
| Club            | player.club_name                                                                                                       |
| Position        | not included in <b>player</b>; ternary relationship (see <b>position</b>, <b>player_position</b> in DB Schema section) |
| Nationality     | player.nationality                                                                                                     |
| Age             | player.age                                                                                                             |


<b>statistics</b>
| csv column name       | table attribute                  |
|-----------------------|----------------------------------|
| Appearances           | statistics.appearances           |
| Wins                  | statistics.wins                  |
| Losses                | statistics.losses                |
| Goals                 | statistics.goals                 |
| Goals per match       | statistics.goals_per_match       |
| Headed goals          | statistics.headed_goals          |
| Goals with right foot | statistics.goals_right_foot      |
| Goals with left foot  | statistics.goals_left_foot       |
| Penalties scored      | statistics.goals_from_penalties  |
| Freekicks scored      | statistics.goals_from_freekicks  |
| Shots                 | statistics.shots                 |
| Shots on target       | statistics.shots_on_target       |
| Shooting accuracy %   | statistics.shooting_accuracy_pct |
| Hit woodwork          | statistics.hit_woodwork          |
| Clean sheets          | statistics.clean_sheets          |
| Goals conceded        | statistics.goals_conceded        |
| Tackles               | statistics.tackles               |
| Tackle success %      | statistics.tackle_success_pct    |
| Blocked shots         | statistics.shots_blocked         |
| Interceptions         | statistics.interceptions         |
| Clearances            | statistics.clearances            |
| Headed Clearance      | statistics.headed_clearances     |
| Own goals             | statistics.own_goals             |
| Assists               | statistics.assists               |
| Passes                | statistics.passes                |
| Crosses               | statistics.crosses               |
| Cross accuracy %      | statistics.cross_accuracy_pct    |
| Passes per Match      | statistics.passes_per_match      |
| Saves                 | statistics.saves                 |
| Penalties saved       | statistics.penalties_saved       |
| Punches               | statistics.punches               |
| High Claims           | statistics.high_claims           |
| Catches               | statistics.catches               |
| Throw outs            | statistics.throw_outs            |
| Goal Kicks            | statistics.goal_kicks            |
| Yellow cards          | statistics.cards_yellow          |
| Red cards             | statistics.cards_red             |
| Fouls                 | statistics.fouls                 |
| Offsides              | statistics.offsides              |

Exclusions:
    * Big chances missed
    * Last man tackles
    * Clearances off line
    * Recoveries
    * Duels won
    * Duels lost
    * Successful 50/50s
    * Aerial battles won
    * Aerial battles lost
    * Errors leading to goal
    * Big chances created
    * Through balls
    * Accurate long balls
    * Sweeper clearances
    
## DB Schema

The DB Schema is as follows, by table:

<b>player</b>
| attribute     | type         | constraints |
|---------------|--------------|-------------|
| id            | INTEGER      | PRIMARY KEY |
| name          | VARCHAR(128) | NOT NULL    |
| jersey_number | INTEGER      | NOT NULL    |
| club_name     | VARCHAR(128) |             |
| nationality   | VARCHAR(64)  | NOT NULL    |
| age           | INTEGER      | NOT NULL    |

* Note that statistics could be merged with player, as it is a one-to-one relationship, but I separate them for organization purposes

<b>statistics</b>
| attribute             | type         | constraints |
|-----------------------|--------------|-------------|
| id                    | INTEGER      | PRIMARY KEY |
| player_id             | INTEGER      | FOREIGN KEY |
| appearances           | INTEGER      | NOT NULL    |
| wins                  | INTEGER      | NOT NULL    |
| losses                | INTEGER      | NOT NULL    |
| goals                 | INTEGER      | NOT NULL    |
| goals_per_match       | INTEGER      | NOT NULL    |
| headed_goals          | INTEGER      | NOT NULL    |
| goals_right_foot      | INTEGER      | NOT NULL    |
| goals_left_foot       | INTEGER      | NOT NULL    |
| goals_from_penalties  | INTEGER      | NOT NULL    |
| goals_from_freekicks  | INTEGER      | NOT NULL    |
| shots                 | INTEGER      | NOT NULL    |
| shots_on_target       | INTEGER      | NOT NULL    |
| shooting_accuracy_pct | DECIMAL(5,4) | NOT NULL    |
| hit_woodwork          | INTEGER      | NOT NULL    |
| clean_sheets          | INTEGER      | NOT NULL    |
| goals_conceded        | INTEGER      | NOT NULL    |
| tackles               | INTEGER      | NOT NULL    |
| tackle_success_pct    | DECIMAL(5,4) | NOT NULL    |
| shots_blocked         | INTEGER      | NOT NULL    |
| interceptions         | INTEGER      | NOT NULL    |
| clearances            | INTEGER      | NOT NULL    |
| headed_clearances     | INTEGER      | NOT NULL    |
| own_goals             | INTEGER      | NOT NULL    |
| assists               | INTEGER      | NOT NULL    |
| passes                | INTEGER      | NOT NULL    |
| crosses               | INTEGER      | NOT NULL    |
| cross_accuracy_pct    | DECIMAL(5,4) | NOT NULL    |
| passes_per_match      | INTEGER      | NOT NULL    |
| saves                 | INTEGER      | NOT NULL    |
| penalties_saved       | INTEGER      | NOT NULL    |
| punches               | INTEGER      | NOT NULL    |
| high_claims           | INTEGER      | NOT NULL    |
| catches               | INTEGER      | NOT NULL    |
| throw_outs            | INTEGER      | NOT NULL    |
| goal_kicks            | INTEGER      | NOT NULL    |
| cards_yellow          | INTEGER      | NOT NULL    |
| cards_red             | INTEGER      | NOT NULL    |
| fouls                 | INTEGER      | NOT NULL    |
| offsides              | INTEGER      | NOT NULL    |

<b>position</b>
| attribute             | type         | constraints |
|-----------------------|--------------|-------------|
| id                    | INTEGER      | PRIMARY KEY |
| name                  | VARCHAR(32)  | NOT NULL    |
* Note: Conventions for position 'name' are followed as per the borrowed database. I.e.:
    * Forward
    * Midfielder
    * Defender
    * Goalkeeper

<b>player_position</b>
| attribute             | type         | constraints |
|-----------------------|--------------|-------------|
| player_id             | INTEGER      | FOREIGN KEY |
| position_id           | INTEGER      | FOREIGN KEY |

* Note: player and position have a many-to-many relationship (i.e., many players can be assigned to many positions, many positions can be assigned to many players).
    As a result, at least two tables are required. Three tables are used for clarity.
      * The dataset used doesn't actually contain any such occurrence, but logically, it makes sense to do this.



