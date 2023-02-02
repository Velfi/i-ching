use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum DivinationMethod {
    /// The ancient yarrow stalk method of divination.
    ///
    /// See [this Wikipedia article](https://en.wikipedia.org/wiki/I_Ching_divination#Yarrow_stalks) for more info.
    AncientYarrowStalk,
    /// The coin toss method of divination.
    ///
    /// See [this Wikipedia article](https://en.wikipedia.org/wiki/I_Ching_divination#Coins) for more info.
    CoinToss,
}
