use crate::rule_set::RuleSet;
use crate::Fruit;
use std::collections::BTreeMap;
impl RuleSet {
    pub fn p96() -> Self {
        Self {
            prob: BTreeMap::<Fruit, u16>::from([
                (Fruit::bananas, 5915),
                (Fruit::bell, 4904),
                (Fruit::watermelon, 4835),
                (Fruit::one_bar, 4623),
                (Fruit::diamond2, 4419),
                (Fruit::grapes, 4297),
                (Fruit::clover, 4000),
                (Fruit::crown, 3801),
                (Fruit::lemon, 3439),
                (Fruit::diamond, 2723),
                (Fruit::cherry, 2687),
                (Fruit::bonus, 2648),
                (Fruit::dollars, 2490),
                (Fruit::orange, 2243),
                (Fruit::heart, 2240),
                (Fruit::kiwi, 2188),
                (Fruit::bar_bar_bar, 2138),
                (Fruit::diamond3, 1616),
                (Fruit::strawberry, 1574),
                (Fruit::big_win, 1523),
                (Fruit::seven, 1232),
            ]),
            rewards: BTreeMap::<(Fruit, u8), u16>::from([
                ((Fruit::bananas, 2), 2),
                ((Fruit::bananas, 3), 25),
                ((Fruit::bell, 2), 2),
                ((Fruit::bell, 3), 44),
                ((Fruit::watermelon, 2), 3),
                ((Fruit::watermelon, 3), 46),
                ((Fruit::one_bar, 2), 3),
                ((Fruit::one_bar, 3), 53),
                ((Fruit::diamond2, 2), 3),
                ((Fruit::diamond2, 3), 62),
                ((Fruit::grapes, 2), 3),
                ((Fruit::grapes, 3), 66),
                ((Fruit::clover, 2), 1),
                ((Fruit::clover, 3), 83),
                ((Fruit::crown, 2), 1),
                ((Fruit::crown, 3), 96),
                ((Fruit::lemon, 2), 2),
                ((Fruit::lemon, 3), 131),
                ((Fruit::diamond, 2), 3),
                ((Fruit::diamond, 3), 264),
                ((Fruit::cherry, 1), 1),
                ((Fruit::cherry, 2), 3),
                ((Fruit::cherry, 3), 276),
                ((Fruit::bonus, 2), 3),
                ((Fruit::bonus, 3), 287),
                ((Fruit::dollars, 2), 3),
                ((Fruit::dollars, 3), 348),
                ((Fruit::orange, 2), 4),
                ((Fruit::orange, 3), 479),
                ((Fruit::heart, 2), 4),
                ((Fruit::heart, 3), 480),
                ((Fruit::kiwi, 2), 4),
                ((Fruit::kiwi, 3), 515),
                ((Fruit::bar_bar_bar, 2), 4),
                ((Fruit::bar_bar_bar, 3), 553),
                ((Fruit::diamond3, 2), 9),
                ((Fruit::diamond3, 3), 1288),
                ((Fruit::strawberry, 2), 10),
                ((Fruit::strawberry, 3), 1395),
                ((Fruit::big_win, 2), 10),
                ((Fruit::big_win, 3), 1540),
                ((Fruit::seven, 2), 17),
                ((Fruit::seven, 3), 2914),
            ]),
            wheel_count: 3,
        }
    }
}
