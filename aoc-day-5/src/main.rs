use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::io::{self, Read};

/// The sacred rules of the Enchanted Printer, dictating the mystical order
/// in which safety manual pages must materialize
#[derive(Debug, Clone, Copy)]
struct PrinterLore {
    prerequisite: u32, // The page that must be printed first
    dependent: u32,    // The page that must follow
}

#[derive(Debug, Default)]
struct PrinterDiagnosis {
    properly_ordered_sum: u32, // Sum of middle pages from correctly ordered updates
    reordered_pages_sum: u32,  // Sum of middle pages after fixing incorrect orders
}

impl fmt::Display for PrinterDiagnosis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let redact = f.sign_minus(); // Using sign_minus as our redaction flag

        writeln!(f, "🖨️  Ancient Printer Diagnosis Complete! 📜")?;
        writeln!(f, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
        write!(f, "✨ Properly Ordered Updates Sum: ")?;
        if redact {
            writeln!(f, "*****")?;
        } else {
            writeln!(f, "{}", self.properly_ordered_sum)?;
        }
        write!(f, "📝 Reordered Updates Sum: ")?;
        if redact {
            writeln!(f, "*****")?;
        } else {
            writeln!(f, "{}", self.reordered_pages_sum)?;
        }
        writeln!(f, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    }
}

impl PrinterLore {
    /// Decipher a line of printer lore from the ancient maintenance manual
    fn decipher(ancient_text: &str) -> Option<Self> {
        let mut runes = ancient_text.split('|');
        let prerequisite = runes.next()?.parse().ok()?;
        let dependent = runes.next()?.parse().ok()?;
        Some(Self {
            prerequisite,
            dependent,
        })
    }
}

/// Interpret the ancient printer's maintenance manual
fn interpret_printer_lore(sacred_text: &str) -> Vec<PrinterLore> {
    sacred_text
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(PrinterLore::decipher)
        .collect()
}

/// Decode the list of pending manual updates from the printer's enchanted queue
fn decode_update_requests(enchanted_queue: &str) -> Vec<Vec<u32>> {
    enchanted_queue
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split(',').map(|num| num.trim().parse().ok()).collect())
        .collect()
}

/// The Printer's Tome of Dependencies - a mystical record of which pages
/// must precede others in the grand tapestry of safety documentation
#[derive(Default)]
struct PrinterTome {
    sacred_order: HashMap<u32, HashSet<u32>>,
}

impl PrinterTome {
    /// Construct the tome from the ancient printer lore
    fn inscribe(lore: &[PrinterLore]) -> Self {
        let mut tome = Self::default();
        for &PrinterLore {
            prerequisite,
            dependent,
        } in lore
        {
            tome.sacred_order
                .entry(prerequisite)
                .or_default()
                .insert(dependent);
        }
        tome
    }

    /// Consult the tome to check if a page ordering violates the sacred rules
    fn violates_sacred_order(&self, first_page: u32, second_page: u32) -> bool {
        self.sacred_order
            .get(&second_page)
            .map(|deps| deps.contains(&first_page))
            .unwrap_or(false)
    }
}

/// Verify if a sequence of pages respects the sacred printing order
fn respects_sacred_order(update: &[u32], lore: &[PrinterLore]) -> bool {
    let tome = PrinterTome::inscribe(lore);
    let pages_in_update: HashSet<_> = update.iter().copied().collect();

    !update.windows(2).any(|window| {
        let (first, second) = (window[0], window[1]);
        tome.violates_sacred_order(first, second)
            && pages_in_update.contains(&first)
            && pages_in_update.contains(&second)
    })
}

/// Find the mystical middle page number of an update sequence
fn divine_middle_page(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

/// The Enchanted Order Keeper - responsible for arranging pages
/// in accordance with the sacred printing laws
struct EnchantedOrderKeeper {
    dependencies: HashMap<u32, HashSet<u32>>,
    prerequisites_count: HashMap<u32, usize>,
}

impl EnchantedOrderKeeper {
    fn summon(pages: &[u32], lore: &[PrinterLore]) -> Self {
        let pages_set: HashSet<_> = pages.iter().copied().collect();
        let mut dependencies: HashMap<u32, HashSet<u32>> =
            pages.iter().map(|&page| (page, HashSet::new())).collect();

        let mut prerequisites_count: HashMap<u32, usize> =
            pages.iter().map(|&page| (page, 0)).collect();

        for &PrinterLore {
            prerequisite,
            dependent,
        } in lore
        {
            if pages_set.contains(&prerequisite) && pages_set.contains(&dependent) {
                dependencies
                    .entry(prerequisite)
                    .or_default()
                    .insert(dependent);
                *prerequisites_count.entry(dependent).or_default() += 1;
            }
        }

        Self {
            dependencies,
            prerequisites_count,
        }
    }

    /// Arrange pages in their proper mystical order
    fn arrange_pages(mut self, pages: &[u32]) -> Vec<u32> {
        let mut sacred_sequence = Vec::new();
        let mut ready_pages: VecDeque<_> = self
            .prerequisites_count
            .iter()
            .filter(|(_, &count)| count == 0)
            .map(|(&page, _)| page)
            .collect();

        while let Some(page) = ready_pages.pop_front() {
            sacred_sequence.push(page);

            if let Some(dependents) = self.dependencies.get(&page) {
                for &next in dependents {
                    *self.prerequisites_count.get_mut(&next).unwrap() -= 1;
                    if self.prerequisites_count[&next] == 0 {
                        ready_pages.push_back(next);
                    }
                }
            }
        }

        // Handle any remaining pages (cycles) in original order
        let arranged_pages: HashSet<_> = sacred_sequence.iter().copied().collect();
        sacred_sequence.extend(
            pages
                .iter()
                .filter(|&&page| !arranged_pages.contains(&page)),
        );

        sacred_sequence
    }
}

/// Diagnose and repair the enchanted printer's update sequence
fn diagnose_enchanted_printer(ancient_scroll: &str) -> PrinterDiagnosis {
    let mut sections = ancient_scroll.split("\n\n");
    let lore = sections
        .next()
        .map(interpret_printer_lore)
        .unwrap_or_default();
    let updates = sections
        .next()
        .map(decode_update_requests)
        .unwrap_or_default();

    let properly_ordered_sum: u32 = updates
        .iter()
        .filter(|update| respects_sacred_order(update, &lore))
        .map(|update| divine_middle_page(update))
        .sum();

    let reordered_pages_sum: u32 = updates
        .iter()
        .filter(|update| !respects_sacred_order(update, &lore))
        .map(|update| {
            let keeper = EnchantedOrderKeeper::summon(update, &lore);
            let properly_arranged = keeper.arrange_pages(update);
            divine_middle_page(&properly_arranged)
        })
        .sum();

    PrinterDiagnosis {
        properly_ordered_sum,
        reordered_pages_sum,
    }
}

fn main() -> io::Result<()> {
    let redact = std::env::args().any(|arg| arg == "--redact");

    let mut ancient_scroll = String::new();
    io::stdin().read_to_string(&mut ancient_scroll)?;

    let diagnosis = diagnose_enchanted_printer(&ancient_scroll);

    // Use the new Display implementation with redaction if specified
    if redact {
        print!("{:-}", diagnosis); // The minus flag triggers redaction
    } else {
        print!("{}", diagnosis);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MYSTICAL_TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_printer_diagnosis() {
        let diagnosis = diagnose_enchanted_printer(MYSTICAL_TEST_INPUT);
        assert_eq!(diagnosis.properly_ordered_sum, 143);
        assert_eq!(diagnosis.reordered_pages_sum, 123);
    }

    #[test]
    fn test_page_arrangement() {
        let lore = interpret_printer_lore(MYSTICAL_TEST_INPUT.split("\n\n").next().unwrap());

        let keeper = EnchantedOrderKeeper::summon(&[75, 97, 47, 61, 53], &lore);
        assert_eq!(
            keeper.arrange_pages(&[75, 97, 47, 61, 53]),
            vec![97, 75, 47, 61, 53]
        );

        let keeper = EnchantedOrderKeeper::summon(&[61, 13, 29], &lore);
        assert_eq!(keeper.arrange_pages(&[61, 13, 29]), vec![61, 29, 13]);

        let keeper = EnchantedOrderKeeper::summon(&[97, 13, 75, 29, 47], &lore);
        assert_eq!(
            keeper.arrange_pages(&[97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn test_display_without_redaction() {
        let diagnosis = PrinterDiagnosis {
            properly_ordered_sum: 143,
            reordered_pages_sum: 123,
        };
        let output = format!("{}", diagnosis);
        assert!(output.contains("143"));
        assert!(output.contains("123"));
    }

    #[test]
    fn test_display_with_redaction() {
        let diagnosis = PrinterDiagnosis {
            properly_ordered_sum: 143,
            reordered_pages_sum: 123,
        };
        let output = format!("{:-}", diagnosis);
        assert!(!output.contains("143"));
        assert!(!output.contains("123"));
        assert!(output.contains("*****"));
    }
}
