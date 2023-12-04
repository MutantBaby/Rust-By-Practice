use std::ops::Range;

#[allow(dead_code)]
enum Change {
    Delete(Range<usize>),
    Replace(Range<usize>, String),
}

trait SpellChecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

struct NoopSpaceChecker;

impl SpellChecker for NoopSpaceChecker {
    fn check(&self, _: &str) -> Vec<Change> {
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

fn _static_spell_check<C>(input: &str, spell_checker: C)
where
    C: SpellChecker,
{
    spell_checker.check(input);
}

fn _dynamic_spell_check(input: &str, spell_checker: &dyn SpellChecker) {
    spell_checker.check(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test: &str = "Hello, IM here";

        _static_spell_check(test, NoopSpaceChecker);
        _dynamic_spell_check(test, &NoopSpaceChecker);
    }
}
