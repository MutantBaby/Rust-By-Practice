use std::ops::Range;

enum Change {
    Delete(Range<usize>),
    Replace(Range<usize>, String),
}

trait SpellChecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

fn spell_check<C>(input: &str, spell_checker: C) -> String
where
    C: SpellChecker,
{
    let mut result: String = input.to_owned();

    for change in spell_checker.check(input) {
        apply_change(&mut result, change);
    }

    result
}

fn apply_change(result: &mut String, change: Change) {}

struct NoopSpellChecker;

impl SpellChecker for NoopSpellChecker {
    fn check(&self, input: &str) -> Vec<Change> {
        Vec::new()
    }
}

struct AntiSpaceChecker;

impl SpellChecker for AntiSpaceChecker {
    fn check(&self, input: &str) -> Vec<Change> {
        input
            .match_indices(" ")
            .map(|(index, space)| Change::Delete(index..index + space.len()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test: &str = "Hello, IM here";
        let result: String = spell_check(test, NoopSpellChecker);
        assert!(result == test);

        // let result: String = spell_check(test, AntiSpaceChecker);
        // assert!(result == test);
    }
}
