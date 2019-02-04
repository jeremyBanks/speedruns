//! Leaderboard logic.
use crate::data::{base::Database, types::*};

// #[derive(Debug, Clone, Getters, Serialize)]
// #[get = "pub"]
// pub struct RankedRun {
//     rank:      Id64,
//     time_ms:   u64,
//     is_tied:   bool,
//     tied_rank: Id64,
//     run:       &'static Run,
// }
// /// Ranks a set of runs (all for the same game/category/level) using the
// /// timing specified for the game rules, then by run date, then by
// /// submission datetime.
// pub fn rank_runs<'db>(&'db self, runs: &[&'db Run]) -> Vec<RankedRun> {
//     let mut runs: Vec<&Run> = runs.to_vec();

//     if runs.is_empty() {
//         return vec![]
//     }

//     let first = runs[0];
//     let game = self
//         .games()
//         .get(first.game_id())
//         .expect("game should exist");

//     runs.sort_by_key(|run| {
//         let time_ms = run.times_ms().get(game.primary_timing()).unwrap();

//         (time_ms, run.date(), run.created())
//     });

//     let mut ranks: Vec<RankedRun> = vec![];

//     for (i, run) in runs.iter().enumerate() {
//         assert_eq!(run.game_id(), first.game_id());
//         assert_eq!(run.level_id(), first.level_id());
//         assert_eq!(run.category_id(), first.category_id());

//         let time_ms = run.times_ms().get(game.primary_timing()).unwrap();
//         let rank = Id64::new((i + 1) as u64).unwrap();
//         let mut tied_rank = rank;
//         let mut is_tied = false;

//         if let Some(ref mut previous) = ranks.last_mut() {
//             if time_ms == *previous.time_ms() {
//                 is_tied = true;
//                 previous.is_tied = true;
//                 tied_rank = previous.tied_rank;
//             }
//         }

//         let new = RankedRun {
//             rank,
//             time_ms,
//             is_tied,
//             tied_rank,
//             run,
//         };

//         ranks.push(new);
//     }

//     ranks
// }
