use bincode::Decode;

#[derive(PartialEq, Debug, Copy, Clone, Decode)]
pub enum AbilityLike {
    P(AbilityName),
    Q(AbilityName),
    W(AbilityName),
    E(AbilityName),
    R(AbilityName),
}

impl AbilityLike {
    pub fn data(&self) -> (char, AbilityName) {
        match self {
            Self::P(name) => ('P', *name),
            Self::Q(name) => ('Q', *name),
            Self::W(name) => ('W', *name),
            Self::E(name) => ('E', *name),
            Self::R(name) => ('R', *name),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Decode)]
pub enum AbilityName {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    Mega,
    Max,
    Min,
    Minion,
    Minion1,
    Minion2,
    Minion3,
    MinionMax,
    Monster,
    Monster1,
    Monster2,
    Monster3,
    Monster4,
    MonsterMax,
    Void,
    _1Max,
    _2Max,
    _3Max,
    _4Max,
    _5Max,
    _6Max,
    _7Max,
    _8Max,
    _1Min,
    _2Min,
    _3Min,
    _4Min,
    _5Min,
    _6Min,
    _7Min,
    _8Min,
}

impl AbilityName {
    pub fn data(&self) -> (&'static str, &'static str) {
        match self {
            Self::_1 => ("1", "damage #1"),
            Self::_2 => ("2", "damage #2"),
            Self::_3 => ("3", "damage #3"),
            Self::_4 => ("4", "damage #4"),
            Self::_5 => ("5", "damage #5"),
            Self::_6 => ("6", "damage #6"),
            Self::_7 => ("7", "damage #7"),
            Self::_8 => ("8", "damage #8"),
            Self::Mega => ("MEGA", "damage as Mega Gnar"),
            Self::Max => ("MAX", "maximum damage"),
            Self::Min => ("MIN", "minimum damage"),
            Self::Minion => ("MN", "minion damage"),
            Self::Minion1 => ("MN", "minion damage #1"),
            Self::Minion2 => ("MN", "minion damage #2"),
            Self::Minion3 => ("MN", "minion damage #3"),
            Self::MinionMax => ("MN+", "minion maximum damage"),
            Self::Monster => ("MT", "monster damage"),
            Self::Monster1 => ("MT1", "monster damage #1"),
            Self::Monster2 => ("MT2", "monster damage #2"),
            Self::Monster3 => ("MT3", "monster damage #3"),
            Self::Monster4 => ("MT4", "monster damage #4"),
            Self::MonsterMax => ("MT+", "monster maximum damage"),
            Self::Void => ("", ""),
            Self::_1Max => ("1+", "maximum damage #1"),
            Self::_2Max => ("2+", "maximum damage #2"),
            Self::_3Max => ("3+", "maximum damage #3"),
            Self::_4Max => ("4+", "maximum damage #4"),
            Self::_5Max => ("5+", "maximum damage #5"),
            Self::_6Max => ("6+", "maximum damage #6"),
            Self::_7Max => ("7+", "maximum damage #7"),
            Self::_8Max => ("8+", "maximum damage #8"),
            Self::_1Min => ("1-", "minimum damage #1"),
            Self::_2Min => ("2-", "minimum damage #2"),
            Self::_3Min => ("3-", "minimum damage #3"),
            Self::_4Min => ("4-", "minimum damage #4"),
            Self::_5Min => ("5-", "minimum damage #5"),
            Self::_6Min => ("6-", "minimum damage #6"),
            Self::_7Min => ("7-", "minimum damage #7"),
            Self::_8Min => ("8-", "minimum damage #8"),
        }
    }
}
