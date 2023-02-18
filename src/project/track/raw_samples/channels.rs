#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Channels {
    #[default]
    All,
    None,
    Just(usize),
    AllBut(usize)
}