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
