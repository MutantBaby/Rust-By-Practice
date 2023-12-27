use std::ops::Range;

enum Change {
    Delete(Range<usize>),
    Replace(Range<usize>, String),
}

trait SpellChecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

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

fn static_spell_check<C>(input: &str, spell_checker: C) -> String
where
    C: SpellChecker,
{
    let mut result: String = input.to_owned();

    for change in spell_checker.check(input) {
        apply_change(&mut result, change);
    }

    result
}

fn dynamic_spell_check(input: &str, spell_checker: &dyn SpellChecker) -> String {
    let mut result: String = input.to_owned();

    for change in spell_checker.check(input) {
        apply_change(&mut result, change);
    }

    result
}

fn apply_change(result: &mut String, change: Change) {}

fn it_works() {
    let test: &str = "Hello, IM here";

    let result: String = static_spell_check(test, NoopSpellChecker);
    assert!(result == test);

    let result: String = dynamic_spell_check(test, &NoopSpellChecker);
    assert!(result == test);

    let spell_checkers: Vec<Box<dyn SpellChecker>> =
        vec![Box::new(NoopSpellChecker), Box::new(AntiSpaceChecker)];

    for checker in spell_checkers {
        dynamic_spell_check(test, checker.as_ref());
    }
}

fn main() {
    it_works();
}
