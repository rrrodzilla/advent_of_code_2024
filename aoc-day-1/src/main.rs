use acton_reactive::prelude::*;
use std::env;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the path to the ancient scroll from the messenger
    let filename = env::args()
        .nth(1)
        .ok_or("🔍 A scroll path must be provided: Usage: program <scroll-path>")?;

    let mut app = ActonApp::launch();
    let mut chief_historian_finder = app.new_agent::<HistorianSage>().await;
    chief_historian_finder
        .act_on::<HistoricalCoordinates>(|agent, context| {
            let (senior, junior) = context.message().0;
            HistorianSage::record_location_sighting(&mut agent.model.senior_historian_list, senior);
            HistorianSage::record_location_sighting(&mut agent.model.junior_historian_list, junior);
            AgentReply::immediate()
        })
        .act_on::<ScrollFullyDecoded>(|agent, _context| {
            let distance = agent.model.measure_historical_discrepancy();
            let score = agent.model.analyze_location_correlation();
            println!("\n🗿 Ancient Scroll Analysis Complete! 📜");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("✨ Historical Location Correlation: {score}");
            println!("📏 Geographical Discrepancy: {distance} leagues");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!(
                "📌 The Chief Historian's findings have been recorded in the eternal archives!\n"
            );
            AgentReply::immediate()
        });

    let mut scroll_reader = app.new_agent::<ScrollDecoder>().await;
    scroll_reader.act_on::<DecodeAncientScroll>({
        |agent, context| {
            let message = context.message();
            let broker = agent.broker().clone();
            // Carefully unroll the ancient scroll and begin decoding its contents
            let content = std::fs::read_to_string(&message.0)
                .expect("📜 The scroll appears to be damaged or sealed!");
            let mut broadcasts = Vec::new();

            // Extract each pair of coordinates from the scroll's mysterious markings
            for line in content.lines() {
                let numbers: Vec<i64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if numbers.len() == 2 {
                    broadcasts.push(HistoricalCoordinates((numbers[0], numbers[1])));
                }
            }

            AgentReply::from_async(async move {
                let futures: Vec<_> = broadcasts
                    .into_iter()
                    .map(|msg| broker.broadcast(msg))
                    .collect();
                futures::future::join_all(futures).await;
                broker.broadcast(ScrollFullyDecoded).await;
            })
        }
    });
    chief_historian_finder
        .handle()
        .subscribe::<HistoricalCoordinates>()
        .await;
    chief_historian_finder
        .handle()
        .subscribe::<HowFarApartAreLocations>()
        .await;
    chief_historian_finder
        .handle()
        .subscribe::<HowSimilarAreTheScrolls>()
        .await;
    chief_historian_finder
        .handle()
        .subscribe::<ScrollFullyDecoded>()
        .await;
    let sage = chief_historian_finder.start().await;
    let ancient_scroll = scroll_reader.start().await;

    ancient_scroll.send(DecodeAncientScroll(filename)).await;

    let _ = ancient_scroll.stop().await;
    let _ = sage.stop().await;
    Ok(())
}
#[derive(Clone, Debug, Default)]
struct ScrollDecoder;

#[derive(Clone, Debug, Default)]
struct ScrollFullyDecoded;

#[derive(Clone, Debug, Default)]
struct DecodeAncientScroll(String);

/// The HistorianSage maintains two parallel chronicles of ancient locations,
/// comparing the findings of senior and junior historians to uncover the truth
/// about the Chief Historian's disappearance. It analyzes both the geographical
/// distances between recorded locations and the correlation between independent
/// sightings.
#[derive(Clone, Debug, Default)]
struct HistorianSage {
    pub(crate) senior_historian_list: Vec<i64>,
    pub(crate) junior_historian_list: Vec<i64>,
}

impl HistorianSage {
    // Records a new location sighting in the historian's scroll, maintaining chronological order
    fn record_location_sighting(scroll: &mut Vec<i64>, location: i64) {
        match scroll.binary_search(&location) {
            Ok(pos) | Err(pos) => scroll.insert(pos, location),
        }
    }

    fn measure_historical_discrepancy(&mut self) -> i64 {
        // Compare findings from both historians up to the shortest record length
        let n = self
            .senior_historian_list
            .len()
            .min(self.junior_historian_list.len());
        // Calculate the geographical distance between each pair of recorded locations
        self.senior_historian_list[..n]
            .iter()
            .zip(&self.junior_historian_list[..n])
            .map(|(l, r)| (l - r).abs())
            .sum()
    }

    fn analyze_location_correlation(&self) -> i64 {
        self.senior_historian_list
            .iter()
            .map(|&location| {
                // For each location the senior historian noted, count how many times
                // the junior historian independently recorded the same location
                let matching_sightings = self
                    .junior_historian_list
                    .iter()
                    .filter(|&&x| x == location)
                    .count();
                // Weight each location by how many times it was independently verified
                location * (matching_sightings as i64)
            })
            .sum()
    }
}

#[derive(Clone, Debug, Default)]
struct HistoricalCoordinates(pub(crate) (i64, i64));

#[derive(Clone, Debug, Default)]
struct HowFarApartAreLocations;

#[derive(Clone, Debug, Default)]
struct HowSimilarAreTheScrolls;

#[cfg(test)]
mod tests {
    use super::*;
    use acton_test::prelude::*;

    #[acton_test]
    async fn test_similarity_score() -> Result<(), Box<dyn Error>> {
        let mut app = ActonApp::launch();
        let mut historian_sage = app.new_agent::<HistorianSage>().await;
        let example_data = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];

        // Set up the handlers
        historian_sage
            .act_on::<HistoricalCoordinates>(|agent, context| {
                let (senior, junior) = context.message().0;
                HistorianSage::record_location_sighting(
                    &mut agent.model.senior_historian_list,
                    senior,
                );
                HistorianSage::record_location_sighting(
                    &mut agent.model.junior_historian_list,
                    junior,
                );
                AgentReply::immediate()
            })
            .act_on::<HowSimilarAreTheScrolls>(|agent, _context| {
                let score = agent.model.analyze_location_correlation();
                assert_eq!(score, 31);
                AgentReply::immediate()
            });

        let historian = historian_sage.start().await;

        // Send all the example data
        for (left, right) in example_data {
            historian.send(HistoricalCoordinates((left, right))).await;
        }

        // Get the similarity score and verify it
        historian.send(HowSimilarAreTheScrolls).await;

        historian.stop().await.unwrap();
        Ok(())
    }
}
