use crate::types::{
    AsrFactor, CalculationUnit, Degrees, HighLatsMethod, MidnightMethod, Minutes, Parameters,
};

pub const MUSLIM_WORLD_LEAGUE: Parameters = Parameters {
    imsak: CalculationUnit::Minutes(Minutes { minutes: 10.0 }),
    dhuhr: Minutes { minutes: 0.0 },
    asr: AsrFactor { factor: 1.0 },
    high_latitudes: HighLatsMethod::NightMiddle,
    fajr: (Degrees { degree: 18.0 }),
    isha: CalculationUnit::Degrees(Degrees { degree: 17.0 }),
    midnight: MidnightMethod::Standard,
    maghrib: CalculationUnit::Minutes(Minutes { minutes: 0.0 }),
};

pub const ISLAMIC_SOCIETY_OF_NORTH_AMERICA: Parameters = Parameters {
    imsak: CalculationUnit::Minutes(Minutes { minutes: 15.0 }),
    dhuhr: Minutes { minutes: 0.0 },
    asr: AsrFactor { factor: 1.0 },
    high_latitudes: HighLatsMethod::NightMiddle,
    fajr: (Degrees { degree: 15.0 }),
    isha: CalculationUnit::Degrees(Degrees { degree: 15.0 }),
    midnight: MidnightMethod::Standard,
    maghrib: CalculationUnit::Minutes(Minutes { minutes: 0.0 }),
};

pub const EGYPTIAN_GENERAL_AUTHORITY_OF_SURVEY: Parameters = Parameters {
    imsak: CalculationUnit::Minutes(Minutes { minutes: 10.0 }),
    dhuhr: Minutes { minutes: 0.0 },
    asr: AsrFactor { factor: 1.0 },
    high_latitudes: HighLatsMethod::NightMiddle,
    fajr: (Degrees { degree: 19.5 }),
    isha: CalculationUnit::Degrees(Degrees { degree: 17.5 }),
    midnight: MidnightMethod::Standard,
    maghrib: CalculationUnit::Minutes(Minutes { minutes: 0.0 }),
};

pub const UMM_AL_QURA_UNIVERSITY_MAKKAH: Parameters = Parameters {
    imsak: CalculationUnit::Minutes(Minutes { minutes: 10.0 }),
    dhuhr: Minutes { minutes: 0.0 },
    asr: AsrFactor { factor: 1.0 },
    high_latitudes: HighLatsMethod::NightMiddle,
    fajr: (Degrees { degree: 18.5 }),
    isha: CalculationUnit::Minutes(Minutes { minutes: 90.0 }),
    midnight: MidnightMethod::Standard,
    maghrib: CalculationUnit::Minutes(Minutes { minutes: 0.0 }),
};

pub const UNIVERSITY_OF_ISLAMIC_SCIENCES_KARACHI: Parameters = Parameters {
    imsak: CalculationUnit::Minutes(Minutes { minutes: 10.0 }),
    dhuhr: Minutes { minutes: 0.0 },
    asr: AsrFactor { factor: 1.0 },
    high_latitudes: HighLatsMethod::NightMiddle,
    fajr: (Degrees { degree: 18.0 }),
    isha: CalculationUnit::Degrees(Degrees { degree: 18.0 }),
    midnight: MidnightMethod::Standard,
    maghrib: CalculationUnit::Minutes(Minutes { minutes: 0.0 }),
};

pub const INSTITUTE_OF_GEOPHYSICS_UNIVERSITY_OF_TEHRAN: Parameters = Parameters {
    imsak: CalculationUnit::Minutes(Minutes { minutes: 10.0 }),
    dhuhr: Minutes { minutes: 0.0 },
    asr: AsrFactor { factor: 1.0 },
    high_latitudes: HighLatsMethod::NightMiddle,
    fajr: (Degrees { degree: 17.7 }),
    isha: CalculationUnit::Degrees(Degrees { degree: 14.0 }),
    midnight: MidnightMethod::Jafari,
    maghrib: CalculationUnit::Degrees(Degrees { degree: 4.5 }),
};

pub const SHIA_ITHNA_ASHARI_LEVA_INSTITUTE_QUM: Parameters = Parameters {
    imsak: CalculationUnit::Minutes(Minutes { minutes: 10.0 }),
    dhuhr: Minutes { minutes: 0.0 },
    asr: AsrFactor { factor: 1.0 },
    high_latitudes: HighLatsMethod::NightMiddle,
    fajr: (Degrees { degree: 16.0 }),
    isha: CalculationUnit::Degrees(Degrees { degree: 14.0 }),
    midnight: MidnightMethod::Jafari,
    maghrib: CalculationUnit::Degrees(Degrees { degree: 4.0 }),
};
