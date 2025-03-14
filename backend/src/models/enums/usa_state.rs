use std::str::FromStr;

#[derive(Debug)]
pub enum USAState {
    AL,
    AK,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    FL,
    GA,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    OH,
    OK,
    OR,
    PA,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VA,
    WA,
    WV,
    WI,
    WY,
    UNKNOWN,
}

impl std::fmt::Display for USAState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for USAState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AL" => Ok(USAState::AL),
            "AK" => Ok(USAState::AK),
            "AZ" => Ok(USAState::AZ),
            "AR" => Ok(USAState::AR),
            "CA" => Ok(USAState::CA),
            "CO" => Ok(USAState::CO),
            "CT" => Ok(USAState::CT),
            "DE" => Ok(USAState::DE),
            "FL" => Ok(USAState::FL),
            "GA" => Ok(USAState::GA),
            "HI" => Ok(USAState::HI),
            "ID" => Ok(USAState::ID),
            "IL" => Ok(USAState::IL),
            "IN" => Ok(USAState::IN),
            "IA" => Ok(USAState::IA),
            "KS" => Ok(USAState::KS),
            "KY" => Ok(USAState::KY),
            "LA" => Ok(USAState::LA),
            "ME" => Ok(USAState::ME),
            "MD" => Ok(USAState::MD),
            "MA" => Ok(USAState::MA),
            "MI" => Ok(USAState::MI),
            "MN" => Ok(USAState::MN),
            "MS" => Ok(USAState::MS),
            "MO" => Ok(USAState::MO),
            "MT" => Ok(USAState::MT),
            "NE" => Ok(USAState::NE),
            "NV" => Ok(USAState::NV),
            "NH" => Ok(USAState::NH),
            "NJ" => Ok(USAState::NJ),
            "NM" => Ok(USAState::NM),
            "NY" => Ok(USAState::NY),
            "NC" => Ok(USAState::NC),
            "ND" => Ok(USAState::ND),
            "OH" => Ok(USAState::OH),
            "OK" => Ok(USAState::OK),
            "OR" => Ok(USAState::OR),
            "PA" => Ok(USAState::PA),
            "RI" => Ok(USAState::RI),
            "SC" => Ok(USAState::SC),
            "SD" => Ok(USAState::SD),
            "TN" => Ok(USAState::TN),
            "TX" => Ok(USAState::TX),
            "UT" => Ok(USAState::UT),
            "VT" => Ok(USAState::VT),
            "VA" => Ok(USAState::VA),
            "WA" => Ok(USAState::WA),
            "WV" => Ok(USAState::WV),
            "WI" => Ok(USAState::WI),
            "WY" => Ok(USAState::WY),
            _ => Err(()),
        }
    }
}

impl From<String> for USAState {
    fn from(value: String) -> Self {
        value.parse().unwrap_or(USAState::UNKNOWN)
    }
}
