use crate::parse_text::Stacks;

pub(crate) fn stacks_to_answer(stacks: &mut Stacks) -> String {
    stacks.iter()
        .map(|stack| stack.front().unwrap())
        .collect()
}