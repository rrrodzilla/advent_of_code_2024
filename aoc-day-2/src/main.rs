use acton_reactive::prelude::*;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the path to the reactor readings from the engineers
    let filename = env::args()
        .nth(1)
        .ok_or("🦌 Reactor readings file must be provided: Usage: program <readings-path>")?;

    let mut app = ActonApp::launch();
    let mut safety_analyzer = app.new_agent::<ReactorSafetyAnalyzer>().await;
    safety_analyzer
        .act_on::<ReactorLevels>(|agent, context| {
            let message = context.message().clone();
            agent.model.analyze_safety_report(message.0);
            AgentReply::immediate()
        })
        .act_on::<AnalysisComplete>(|agent, _context| {
            println!("\n🦌 Red-Nosed Reactor Safety Analysis Complete! 🎄");
            println!("================================================");
            println!("🔬 Total Reports Analyzed: {}", agent.model.total_reports);
            println!("✅ Safe Reports Detected: {}", agent.model.safe_reports);
            println!("🛠️ Problem Dampener Active");
            println!("================================================");
            AgentReply::immediate()
        });

    let mut readings_scanner = app.new_agent::<ReactorScanner>().await;
    readings_scanner.act_on::<ScanReactorReadings>({
        |agent, context| {
            let message = context.message().clone();
            let broker = agent.broker().clone();
            // Process the reactor readings file
            let content = std::fs::read_to_string(&message.0)
                .expect("🚨 Error: Reactor readings file is corrupted or inaccessible!");
            let mut broadcasts = Vec::new();

            // Parse each line of reactor levels
            for line in content.lines() {
                let levels: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if !levels.is_empty() {
                    broadcasts.push(ReactorLevels(levels));
                }
            }

            AgentReply::from_async(async move {
                let futures: Vec<_> = broadcasts
                    .into_iter()
                    .map(|msg| broker.broadcast(msg))
                    .collect();
                futures::future::join_all(futures).await;
                broker.broadcast(AnalysisComplete).await;
            })
        }
    });

    safety_analyzer.handle().subscribe::<ReactorLevels>().await;
    safety_analyzer
        .handle()
        .subscribe::<AnalysisComplete>()
        .await;
    let analyzer = safety_analyzer.start().await;
    let scanner = readings_scanner.start().await;

    scanner.send(ScanReactorReadings(filename)).await;

    let _ = scanner.stop().await;
    let _ = analyzer.stop().await;
    Ok(())
}

/// The ReactorSafetyAnalyzer maintains a count of safe reports and processes
/// each reading to determine if it meets the strict safety criteria of the
/// Red-Nosed reactor, with support for the Problem Dampener module.
#[derive(Clone, Debug, Default)]
struct ReactorSafetyAnalyzer {
    total_reports: usize,
    safe_reports: usize,
}

impl ReactorSafetyAnalyzer {
    fn check_sequence_safety(&self, levels: &[i32]) -> bool {
        if levels.len() < 2 {
            return true;
        }

        let mut is_increasing = None;

        for window in levels.windows(2) {
            let diff = window[1] - window[0];

            // Check if difference is within safe range (1-3)
            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }

            // Determine and verify consistent direction
            match is_increasing {
                None => is_increasing = Some(diff > 0),
                Some(increasing) => {
                    if (diff > 0) != increasing {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn analyze_safety_report(&mut self, levels: Vec<i32>) {
        self.total_reports += 1;

        // First check if the sequence is already safe
        if self.check_sequence_safety(&levels) {
            self.safe_reports += 1;
            return;
        }

        // If not safe, try removing each level one at a time (Problem Dampener)
        for skip_idx in 0..levels.len() {
            let dampened_levels: Vec<i32> = levels
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip_idx)
                .map(|(_, &v)| v)
                .collect();

            if self.check_sequence_safety(&dampened_levels) {
                self.safe_reports += 1;
                return;
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ReactorScanner;

#[derive(Clone, Debug)]
struct ReactorLevels(Vec<i32>);

#[derive(Clone, Debug, Default)]
struct ScanReactorReadings(String);

#[derive(Clone, Debug, Default)]
struct AnalysisComplete;

#[cfg(test)]
mod tests {
    use super::*;
    use acton_test::prelude::*;

    #[derive(Clone, Debug)]
    struct VerifyResults(usize);

    #[acton_test]
    async fn test_reactor_safety_with_dampener() -> Result<(), Box<dyn Error>> {
        let mut app = ActonApp::launch();
        let mut safety_analyzer = app.new_agent::<ReactorSafetyAnalyzer>().await;

        // Example data from the narrative
        let example_data = vec![
            vec![7, 6, 4, 2, 1], // Safe without dampener
            vec![1, 2, 7, 8, 9], // Unsafe even with dampener
            vec![9, 7, 6, 2, 1], // Unsafe even with dampener
            vec![1, 3, 2, 4, 5], // Safe by removing 3
            vec![8, 6, 4, 4, 1], // Safe by removing middle 4
            vec![1, 3, 6, 7, 9], // Safe without dampener
        ];

        // Set up the handlers
        safety_analyzer
            .act_on::<ReactorLevels>(|agent, context| {
                let message = context.message().clone();
                agent.model.analyze_safety_report(message.0);
                AgentReply::immediate()
            })
            .act_on::<AnalysisComplete>(|agent, _context| {
                assert_eq!(
                    agent.model.safe_reports, 4,
                    "Expected 4 safe reports with Problem Dampener active"
                );
                assert_eq!(agent.model.total_reports, 6, "Expected 6 total reports");
                AgentReply::immediate()
            });

        let analyzer = safety_analyzer.start().await;

        // Process each test case
        for levels in example_data {
            analyzer.send(ReactorLevels(levels)).await;
        }

        // Verify final results
        analyzer.send(AnalysisComplete).await;

        analyzer.stop().await.unwrap();
        Ok(())
    }

    #[acton_test]
    async fn test_specific_dampener_cases() -> Result<(), Box<dyn Error>> {
        let mut app = ActonApp::launch();
        let mut safety_analyzer = app.new_agent::<ReactorSafetyAnalyzer>().await;

        // Test the specific cases mentioned in the narrative
        safety_analyzer
            .act_on::<ReactorLevels>(|agent, context| {
                let message = context.message().clone();
                agent.model.analyze_safety_report(message.0);
                AgentReply::immediate()
            })
            .act_on::<VerifyResults>(|agent, context| {
                let message = context.message().clone();
                let expected = message.0;
                assert_eq!(
                    agent.model.safe_reports, expected,
                    "Expected {} safe reports at this point",
                    expected
                );
                AgentReply::immediate()
            });

        let analyzer = safety_analyzer.start().await;

        // Test case: Safe by removing second level (3)
        analyzer.send(ReactorLevels(vec![1, 3, 2, 4, 5])).await;
        analyzer.send(VerifyResults(1)).await;

        // Test case: Safe by removing third level (4)
        analyzer.send(ReactorLevels(vec![8, 6, 4, 4, 1])).await;
        analyzer.send(VerifyResults(2)).await;

        analyzer.stop().await.unwrap();
        Ok(())
    }
}
