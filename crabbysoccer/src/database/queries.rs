use crate::common::format_vec;
use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

pub trait TableAttributes: IntoEnumIterator {
    fn as_str(&self) -> &'static str;
}

#[derive(EnumIter)]
pub enum TablePlayerAttributes {
    ID,
    Name,
    JerseyNumber,
    ClubName,
    Nationality,
    Age,
}
type _TPA = TablePlayerAttributes; // alias
impl TableAttributes for TablePlayerAttributes {
    fn as_str(&self) -> &'static str {
        match &self {
            _TPA::ID => "id",
            _TPA::Name => "name",
            _TPA::JerseyNumber => "jersey_number",
            _TPA::ClubName => "club_name",
            _TPA::Nationality => "nationality",
            _TPA::Age => "age",
        }
    }
}

#[derive(EnumIter)]
pub enum TableStatisticsAttributes {
    ID,
    PlayerID,
    Appearances,
    Wins,
    Losses,
    Goals,
    GoalsPerMatch,
    HeadedGoals,
    GoalsRightFoot,
    GoalsLeftFoot,
    GoalsFromPenalties,
    GoalsFromFreekicks,
    Shots,
    ShotsOnTarget,
    ShootingAccuracyPct,
    HitWoodwork,
    CleanSheets,
    GoalsConceded,
    Tackles,
    TackleSuccessPct,
    ShotsBlocked,
    Interceptions,
    Clearances,
    HeadedClearances,
    OwnGoals,
    Assists,
    Passes,
    Crosses,
    CrossAccuracyPct,
    PassesPerMatch,
    Saves,
    PenaltiesSaved,
    Punches,
    HighClaims,
    Catches,
    ThrowOuts,
    GoalKicks,
    CardsYellow,
    CardsRed,
    Fouls,
    Offsides,
}
type _TSA = TableStatisticsAttributes;
impl TableAttributes for TableStatisticsAttributes {
    fn as_str(&self) -> &'static str {
        match &self {
            _TSA::ID => "id",
            _TSA::PlayerID => "player_id",
            _TSA::Appearances => "appearances",
            _TSA::Wins => "wins",
            _TSA::Losses => "losses",
            _TSA::Goals => "goals",
            _TSA::GoalsPerMatch => "goals_per_match",
            _TSA::HeadedGoals => "headed_goals",
            _TSA::GoalsRightFoot => "goals_right_foot",
            _TSA::GoalsLeftFoot => "goals_left_foot",
            _TSA::GoalsFromPenalties => "goals_from_penalties",
            _TSA::GoalsFromFreekicks => "goals_from_freekicks",
            _TSA::Shots => "shots",
            _TSA::ShotsOnTarget => "shots_on_target",
            _TSA::ShootingAccuracyPct => "shooting_accuracy_pct",
            _TSA::HitWoodwork => "hit_woodwork",
            _TSA::CleanSheets => "clean_sheets",
            _TSA::GoalsConceded => "goals_conceded",
            _TSA::Tackles => "tackles",
            _TSA::TackleSuccessPct => "tackle_success_pct",
            _TSA::ShotsBlocked => "shots_blocked",
            _TSA::Interceptions => "interceptions",
            _TSA::Clearances => "clearances",
            _TSA::HeadedClearances => "headed_clearances",
            _TSA::OwnGoals => "own_goals",
            _TSA::Assists => "assists",
            _TSA::Passes => "passes",
            _TSA::Crosses => "crosses",
            _TSA::CrossAccuracyPct => "cross_accuracy_pct",
            _TSA::PassesPerMatch => "passes_per_match",
            _TSA::Saves => "saves",
            _TSA::PenaltiesSaved => "penalties_saved",
            _TSA::Punches => "punches",
            _TSA::HighClaims => "high_claims",
            _TSA::Catches => "catches",
            _TSA::ThrowOuts => "throw_outs",
            _TSA::GoalKicks => "goal_kicks",
            _TSA::CardsYellow => "cards_yellow",
            _TSA::CardsRed => "cards_red",
            _TSA::Fouls => "fouls",
            _TSA::Offsides => "offsides",
        }
    }
}

#[derive(EnumIter)]
pub enum TablePositionAttributes {
    PlayerID,
    Name,
}
type _TPOSA = TablePositionAttributes;
impl TableAttributes for TablePositionAttributes {
    fn as_str(&self) -> &'static str {
        match &self {
            _TPOSA::PlayerID => "player_id",
            _TPOSA::Name => "name",
        }
    }
}

pub enum PredefinedQuery {
    CreateTablePlayer,
    CreateTableStatistics,
    CreateTablePosition,
}

pub fn get_predefined_query(query: PredefinedQuery) -> String {
    match query {
        PredefinedQuery::CreateTablePlayer => format_vec(
            "CREATE TABLE player (
            {} INTEGER PRIMARY KEY AUTO_INCREMENT,
            {} VARCHAR(128) NOT NULL,
            {} INTEGER NOT NULL,
            {} VARCHAR(128),
            {} VARCHAR(64) NOT NULL,
            {} INTEGER NOT NULL);",
            &_TPA::iter().map(|e| e.as_str()).collect(),
        ),
        PredefinedQuery::CreateTableStatistics => format_vec(
            "CREATE TABLE statistics (
                {} INTEGER PRIMARY KEY AUTO_INCREMENT,
                {} INTEGER FOREIGN KEY,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} DECIMAL(5,4) NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} DECIMAL(5,4) NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} DECIMAL(5,4) NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL,
                {} INTEGER NOT NULL
            );",
            &_TSA::iter().map(|e| e.as_str()).collect(),
        ),
        PredefinedQuery::CreateTablePosition => format_vec(
            "CREATE TABLE position (
                {} INTEGER FOREIGN KEY
                {} VARCHAR(10) FOREIGN KEY
            );",
            &_TPOSA::iter().map(|e| e.as_str()).collect(),
        ),
    }
}
