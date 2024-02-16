#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MarketError {
    MintOverflow,
    MintPSP22Error,
    BurnPSP22Error,
    BurnOverflow,
}
