use serde::{Deserialize, Serialize};
use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgValueRef},
};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Country {
    China,
    India,
    UnitedStates,
    Indonesia,
    Pakistan,
    Brazil,
    Nigeria,
    Bangladesh,
    Russia,
    Mexico,
    Japan,
    Ethiopia,
    Philippines,
    Egypt,
    Vietnam,
    Turkey,
    Iran,
    Germany,
    Thailand,
    UnitedKingdom,
    France,
    Italy,
    Tanzania,
    SouthAfrica,
    Myanmar,
    SouthKorea,
    Colombia,
    Kenya,
    Spain,
    Argentina,
    Sudan,
    Ukraine,
    Iraq,
    Afghanistan,
    Poland,
    Canada,
    Uganda,
    Morocco,
    SaudiArabia,
    Uzbekistan,
    Peru,
    Malaysia,
    Angola,
    Yemen,
    Ghana,
    Mozambique,
    Nepal,
    Venezuela,
    IvoryCoast,
    Unknown,
}

impl Country {
    /// Returns the ISO 3166-1 alpha-3 code for the country
    pub fn alpha3_code(&self) -> &'static str {
        match self {
            Country::China => "CHN",
            Country::India => "IND",
            Country::UnitedStates => "USA",
            Country::Indonesia => "IDN",
            Country::Pakistan => "PAK",
            Country::Brazil => "BRA",
            Country::Nigeria => "NGA",
            Country::Bangladesh => "BGD",
            Country::Russia => "RUS",
            Country::Mexico => "MEX",
            Country::Japan => "JPN",
            Country::Ethiopia => "ETH",
            Country::Philippines => "PHL",
            Country::Egypt => "EGY",
            Country::Vietnam => "VNM",
            Country::Turkey => "TUR",
            Country::Iran => "IRN",
            Country::Germany => "DEU",
            Country::Thailand => "THA",
            Country::UnitedKingdom => "GBR",
            Country::France => "FRA",
            Country::Italy => "ITA",
            Country::Tanzania => "TZA",
            Country::SouthAfrica => "ZAF",
            Country::Myanmar => "MMR",
            Country::SouthKorea => "KOR",
            Country::Colombia => "COL",
            Country::Kenya => "KEN",
            Country::Spain => "ESP",
            Country::Argentina => "ARG",
            Country::Sudan => "SDN",
            Country::Ukraine => "UKR",
            Country::Iraq => "IRQ",
            Country::Afghanistan => "AFG",
            Country::Poland => "POL",
            Country::Canada => "CAN",
            Country::Uganda => "UGA",
            Country::Morocco => "MAR",
            Country::SaudiArabia => "SAU",
            Country::Uzbekistan => "UZB",
            Country::Peru => "PER",
            Country::Malaysia => "MYS",
            Country::Angola => "AGO",
            Country::Yemen => "YEM",
            Country::Ghana => "GHA",
            Country::Mozambique => "MOZ",
            Country::Nepal => "NPL",
            Country::Venezuela => "VEN",
            Country::IvoryCoast => "CIV",
            Country::Unknown => "UNKNOWN",
        }
    }
}

impl FromStr for Country {
    type Err = Country;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CHN" => Ok(Country::China),
            "IND" => Ok(Country::India),
            "USA" => Ok(Country::UnitedStates),
            "IDN" => Ok(Country::Indonesia),
            "PAK" => Ok(Country::Pakistan),
            "BRA" => Ok(Country::Brazil),
            "NGA" => Ok(Country::Nigeria),
            "BGD" => Ok(Country::Bangladesh),
            "RUS" => Ok(Country::Russia),
            "MEX" => Ok(Country::Mexico),
            "JPN" => Ok(Country::Japan),
            "ETH" => Ok(Country::Ethiopia),
            "PHL" => Ok(Country::Philippines),
            "EGY" => Ok(Country::Egypt),
            "VNM" => Ok(Country::Vietnam),
            "TUR" => Ok(Country::Turkey),
            "IRN" => Ok(Country::Iran),
            "DEU" => Ok(Country::Germany),
            "THA" => Ok(Country::Thailand),
            "GBR" => Ok(Country::UnitedKingdom),
            "FRA" => Ok(Country::France),
            "ITA" => Ok(Country::Italy),
            "TZA" => Ok(Country::Tanzania),
            "ZAF" => Ok(Country::SouthAfrica),
            "MMR" => Ok(Country::Myanmar),
            "KOR" => Ok(Country::SouthKorea),
            "COL" => Ok(Country::Colombia),
            "KEN" => Ok(Country::Kenya),
            "ESP" => Ok(Country::Spain),
            "ARG" => Ok(Country::Argentina),
            "SDN" => Ok(Country::Sudan),
            "UKR" => Ok(Country::Ukraine),
            "IRQ" => Ok(Country::Iraq),
            "AFG" => Ok(Country::Afghanistan),
            "POL" => Ok(Country::Poland),
            "CAN" => Ok(Country::Canada),
            "UGA" => Ok(Country::Uganda),
            "MAR" => Ok(Country::Morocco),
            "SAU" => Ok(Country::SaudiArabia),
            "UZB" => Ok(Country::Uzbekistan),
            "PER" => Ok(Country::Peru),
            "MYS" => Ok(Country::Malaysia),
            "AGO" => Ok(Country::Angola),
            "YEM" => Ok(Country::Yemen),
            "GHA" => Ok(Country::Ghana),
            "MOZ" => Ok(Country::Mozambique),
            "NPL" => Ok(Country::Nepal),
            "VEN" => Ok(Country::Venezuela),
            "CIV" => Ok(Country::IvoryCoast),
            _ => Err(Country::Unknown),
        }
    }
}

impl From<String> for Country {
    fn from(value: String) -> Self {
        value.parse().unwrap_or(Country::Unknown)
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.alpha3_code())
    }
}

impl Type<sqlx::Postgres> for Country {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}

impl Encode<'_, Postgres> for Country {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, sqlx::error::BoxDynError> {
        buf.extend_from_slice(self.alpha3_code().as_bytes());
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Postgres> for Country {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Country::from_str(s).map_err(|_| "Failed to decode country".into())
    }
}
