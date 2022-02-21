#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cash {
    AfghanAfghani,                      // AFN  ؋
    AlbanianLek,                        // ALL  L
    AlgerianDinar,                      // DZD  د.ج
    AngolanKwanza,                      // AOA  Kz
    ArgentinePeso,                      // ARS  $
    ArmenianDram,                       // AMD  ֏
    AustralianDollar,                   // AUD  $
    AzerbaijaniManat,                   // AZN  ₼
    BangladeshiTaka,                    // BDT  ৳
    BelarusianRuble,                    // BYN  Br
    BolivianBoliviano,                  // BOB  Bs.
    BotswanaPula,                       // BWP  P
    BrazilianReal,                      // BRL  R$
    BritishPound,                       // GBP  £
    BurmeseKyat,                        // MMK  Ks
    BurundianFranc,                     // BIF  Fr
    CambodianRiel,                      // KHR  ៛
    CanadianDollar,                     // CAD  $
    ChileanPeso,                        // CLP  $
    ChineseYuan,                        // CNY  元
    ColombianPeso,                      // COP  $
    CongoleseFranc,                     // CDF  Fr
    CostaRicanColon,                    // CRC  ₡
    CubanPeso,                          // CUP  $
    CzechKoruna,                        // CZK  Kč
    DominicanPeso,                      // DOP  RD$
    EgyptianPound,                      // EGP  ج.م
    EthiopianBirr,                      // ETB  Br
    Euro,                               // EUR  €
    FijianDollar,                       // FJD  $
    GambianDalasi,                      // GMD  D
    GeorgianLari,                       // GEL  ₾
    GhanaianCedi,                       // GHS  ₵
    GuatemalanQuetzal,                  // GTQ  Q
    GuineanFranc,                       // GNF  Fr
    GuyaneseDollar,                     // GYD  $
    HaitianGourde,                      // HTG  G
    HonduranLempira,                    // HNL  L
    HungarianForint,                    // HUF  Ft
    IcelandicKrona,                     // ISK  kr
    IndianRupee,                        // INR  ₹
    IndonesianRupiah,                   // IDR  Rp
    IranianRial,                        // IRR  ﷼
    IraqiDinar,                         // IQD  ع.د
    IsraeliNewShekel,                   // ILS  ₪
    JamaicanDollar,                     // JMD  $
    JapaneseYen,                        // JPY  円
    KazakhstaniTenge,                   // KZT  ₸
    KenyanShilling,                     // KES  Sh
    KyrgyzstaniSom,                     // KGS  c
    LaoKip,                             // LAK  ₭
    LiberianDollar,                     // LRD  $
    LibyanDinar,                        // LYD  ل.د
    MacedonianDenar,                    // MKD  ден
    MalagasyAriary,                     // MGA  Ar
    MalawianKwacha,                     // MWK  MK
    MalaysianRinggit,                   // MYR  RM
    MauritanianOuguiya,                 // MRU  UM
    MauritianRupee,                     // MUR  Rs
    MexicanPeso,                        // MXN  $
    MoldovanLeu,                        // MDL  L
    MongolianTogrog,                    // MNT  ₮
    MozambicanMetical,                  // MZN  MT
    NewTaiwanDollar,                    // TWD  $
    NewZealandDollar,                   // NZD  $
    NicaraguanCordoba,                  // NIO  C$
    NigerianNaira,                      // NGN  ₦
    NorthKoreanWon,                     // KPW  ₩
    NorwegianKrone,                     // NOK  kr
    PakistaniRupee,                     // PKR  Rs
    PapuaNewGuineanKina,                // PGK  K
    ParaguayanGuarani,                  // PYG  ₲
    PeruvianSol,                        // PEN  S/.
    PhilippinePeso,                     // PHP  ₱
    PolishZloty,                        // PLN  zł
    RomanianLeu,                        // RON  lei
    RwandanFranc,                       // RWF  Fr
    RussianRuble,                       // RUB  ₽
    SamoanTala,                         // WST  T
    SerbianDinar,                       // RSD  дин.
    SeychelloisRupee,                   // SCR  Rs
    SierraLeoneanLeone,                 // SLL  Le
    SingaporeDollar,                    // SGD  $
    SolomonIslandsDollar,               // SBD  $
    SomaliShilling,                     // SOS  Sh
    SomalilandShilling,                 // SLS  Sl
    SouthAfricanRand,                   // ZAR  R
    SouthKoreanWon,                     // KRW  ₩
    SouthSudanesePound,                 // SSP  £
    SriLankanRupee,                     // LKR  ரூ
    SudanesePound,                      // SDG  ج.س.
    SurinameseDollar,                   // SRD  $
    SwedishKrona,                       // SEK  kr
    SwissFranc,                         // CHF  Fr.
    SyrianPound,                        // SYP  ل.س
    TajikistaniSomoni,                  // TJS  с.
    TanzanianShilling,                  // TZS  Sh
    ThaiBaht,                           // THB  ฿
    TonganPaAnga,                       // TOP  T$
    TransnistrianRuble,                 // PRB  p.
    TrinidadandTobagoDollar,            // TTD  $
    TunisianDinar,                      // TND  د.ت
    TurkishLira,                        // TRY  ₺
    TurkmenistanManat,                  // TMT  m.
    UgandanShilling,                    // UGX  USh
    UkrainianHryvnia,                   // UAH  ₴
    UnitedArabEmiratesDirham,           // AED د.إ
    UnitedStatesDollar,                 // USD  $
    UruguayanPeso,                      // UYU  $
    UzbekistaniSom,                     // UZS  Sʻ
    VanuatuVatu,                        // VUV  Vt
    VenezuelanBolivarSoberano,          // VES Bs.
    VietnameseDong,                     // VND  ₫
    YemeniRial,                         // YER  ﷼
    ZambianKwacha,                      // ZMW  ZK
    RTGSdollar,                         // ZWB  RTGS$
}

impl Cash {
    pub const ALL: [Cash; 116] = [
        Cash::AfghanAfghani,                    // AFN  ؋
        Cash::AlbanianLek,                      // ALL  L
        Cash::AlgerianDinar,                    // DZD  د.ج
        Cash::AngolanKwanza,                    // AOA  Kz
        Cash::ArgentinePeso,                    // ARS  $
        Cash::ArmenianDram,                     // AMD  ֏
        Cash::AustralianDollar,                 // AUD  $
        Cash::AzerbaijaniManat,                 // AZN  ₼
        Cash::BangladeshiTaka,                  // BDT  ৳
        Cash::BelarusianRuble,                  // BYN  Br
        Cash::BolivianBoliviano,                // BOB  Bs.
        Cash::BotswanaPula,                     // BWP  P
        Cash::BrazilianReal,                    // BRL  R$
        Cash::BritishPound,                     // GBP  £
        Cash::BurmeseKyat,                      // MMK  Ks
        Cash::BurundianFranc,                   // BIF  Fr
        Cash::CambodianRiel,                    // KHR  ៛
        Cash::CanadianDollar,                   // CAD  $
        Cash::ChileanPeso,                      // CLP  $
        Cash::ChineseYuan,                      // CNY  元
        Cash::ColombianPeso,                    // COP  $
        Cash::CongoleseFranc,                   // CDF  Fr
        Cash::CostaRicanColon,                  // CRC  ₡
        Cash::CubanPeso,                        // CUP  $
        Cash::CzechKoruna,                      // CZK  Kč
        Cash::DominicanPeso,                    // DOP  RD$
        Cash::EgyptianPound,                    // EGP  ج.م
        Cash::EthiopianBirr,                    // ETB  Br
        Cash::Euro,                             // EUR  €
        Cash::FijianDollar,                     // FJD  $
        Cash::GambianDalasi,                    // GMD  D
        Cash::GeorgianLari,                     // GEL  ₾
        Cash::GhanaianCedi,                     // GHS  ₵
        Cash::GuatemalanQuetzal,                // GTQ  Q
        Cash::GuineanFranc,                     // GNF  Fr
        Cash::GuyaneseDollar,                   // GYD  $
        Cash::HaitianGourde,                    // HTG  G
        Cash::HonduranLempira,                  // HNL  L
        Cash::HungarianForint,                  // HUF  Ft
        Cash::IcelandicKrona,                   // ISK  kr
        Cash::IndianRupee,                      // INR  ₹
        Cash::IndonesianRupiah,                 // IDR  Rp
        Cash::IranianRial,                      // IRR  ﷼
        Cash::IraqiDinar,                       // IQD  ع.د
        Cash::IsraeliNewShekel,                 // ILS  ₪
        Cash::JamaicanDollar,                   // JMD  $
        Cash::JapaneseYen,                      // JPY  円
        Cash::KazakhstaniTenge,                 // KZT  ₸
        Cash::KenyanShilling,                   // KES  Sh
        Cash::KyrgyzstaniSom,                   // KGS  c
        Cash::LaoKip,                           // LAK  ₭
        Cash::LiberianDollar,                   // LRD  $
        Cash::LibyanDinar,                      // LYD  ل.د
        Cash::MacedonianDenar,                  // MKD  ден
        Cash::MalagasyAriary,                   // MGA  Ar
        Cash::MalawianKwacha,                   // MWK  MK
        Cash::MalaysianRinggit,                 // MYR  RM
        Cash::MauritanianOuguiya,               // MRU  UM
        Cash::MauritianRupee,                   // MUR  Rs
        Cash::MexicanPeso,                      // MXN  $
        Cash::MoldovanLeu,                      // MDL  L
        Cash::MongolianTogrog,                  // MNT  ₮
        Cash::MozambicanMetical,                // MZN  MT
        Cash::NewTaiwanDollar,                  // TWD  $
        Cash::NewZealandDollar,                 // NZD  $
        Cash::NicaraguanCordoba,                // NIO  C$
        Cash::NigerianNaira,                    // NGN  ₦
        Cash::NorthKoreanWon,                   // KPW  ₩
        Cash::NorwegianKrone,                   // NOK  kr
        Cash::PakistaniRupee,                   // PKR  Rs
        Cash::PapuaNewGuineanKina,              // PGK  K
        Cash::ParaguayanGuarani,                // PYG  ₲
        Cash::PeruvianSol,                      // PEN  S/.
        Cash::PhilippinePeso,                   // PHP  ₱
        Cash::PolishZloty,                      // PLN  zł
        Cash::RomanianLeu,                      // RON  lei
        Cash::RwandanFranc,                     // RWF  Fr
        Cash::RussianRuble,                     // RUB  ₽
        Cash::SamoanTala,                       // WST  T
        Cash::SerbianDinar,                     // RSD  дин.
        Cash::SeychelloisRupee,                 // SCR  Rs
        Cash::SierraLeoneanLeone,               // SLL  Le
        Cash::SingaporeDollar,                  // SGD  $
        Cash::SolomonIslandsDollar,             // SBD  $
        Cash::SomaliShilling,                   // SOS  Sh
        Cash::SomalilandShilling,               // SLS  Sl
        Cash::SouthAfricanRand,                 // ZAR  R
        Cash::SouthKoreanWon,                   // KRW  ₩
        Cash::SouthSudanesePound,               // SSP  £
        Cash::SriLankanRupee,                   // LKR  ரூ
        Cash::SudanesePound,                    // SDG  ج.س.
        Cash::SurinameseDollar,                 // SRD  $
        Cash::SwedishKrona,                     // SEK  kr
        Cash::SwissFranc,                       // CHF  Fr.
        Cash::SyrianPound,                      // SYP  ل.س
        Cash::TajikistaniSomoni,                // TJS  с.
        Cash::TanzanianShilling,                // TZS  Sh
        Cash::ThaiBaht,                         // THB  ฿
        Cash::TonganPaAnga,                     // TOP  T$
        Cash::TransnistrianRuble,               // PRB  p.
        Cash::TrinidadandTobagoDollar,          // TTD  $
        Cash::TunisianDinar,                    // TND  د.ت
        Cash::TurkishLira,                      // TRY  ₺
        Cash::TurkmenistanManat,                // TMT  m.
        Cash::UgandanShilling,                  // UGX  USh
        Cash::UkrainianHryvnia,                 // UAH  ₴
        Cash::UnitedArabEmiratesDirham,         // AED د.إ
        Cash::UnitedStatesDollar,               // USD  $
        Cash::UruguayanPeso,                    // UYU  $
        Cash::UzbekistaniSom,                   // UZS  Sʻ
        Cash::VanuatuVatu,                      // VUV  Vt
        Cash::VenezuelanBolivarSoberano,        // VES Bs.
        Cash::VietnameseDong,                   // VND  ₫
        Cash::YemeniRial,                       // YER  ﷼
        Cash::ZambianKwacha,                    // ZMW  ZK
        Cash::RTGSdollar,                       // ZWB  RTGS$
    ];

    pub fn symbol(&self) -> String {
        match self {
            Cash::AfghanAfghani => format!("؋"),                     // AFN  ؋
            Cash::AlbanianLek => format!("L"),                       // ALL  L
            Cash::AlgerianDinar => format!("د.ج"),                     // DZD  د.ج
            Cash::AngolanKwanza => format!("Kz"),                     // AOA  Kz
            Cash::ArgentinePeso => format!("$"),                     // ARS  $
            Cash::ArmenianDram => format!("֏"),                      // AMD  ֏
            Cash::AustralianDollar => format!("$"),                  // AUD  $
            Cash::AzerbaijaniManat => format!("₼"),                  // AZN  ₼
            Cash::BangladeshiTaka => format!("৳"),                   // BDT  ৳
            Cash::BelarusianRuble => format!("Br"),                   // BYN  Br
            Cash::BolivianBoliviano => format!("Bs."),                 // BOB  Bs.
            Cash::BotswanaPula => format!("P"),                      // BWP  P
            Cash::BrazilianReal => format!("R$"),                     // BRL  R$
            Cash::BritishPound => format!("£"),                      // GBP  £
            Cash::BurmeseKyat => format!("Ks"),                       // MMK  Ks
            Cash::BurundianFranc => format!("Fr"),                    // BIF  Fr
            Cash::CambodianRiel => format!("៛"),                     // KHR  ៛
            Cash::CanadianDollar => format!("$"),                    // CAD  $
            Cash::ChileanPeso => format!("$"),                       // CLP  $
            Cash::ChineseYuan => format!("元"),                       // CNY  元
            Cash::ColombianPeso => format!("$"),                     // COP  $
            Cash::CongoleseFranc => format!("Fr"),                    // CDF  Fr
            Cash::CostaRicanColon => format!("₡"),                   // CRC  ₡
            Cash::CubanPeso => format!("$"),                         // CUP  $
            Cash::CzechKoruna => format!("Kč"),                       // CZK  Kč
            Cash::DominicanPeso => format!("RD$"),                     // DOP  RD$
            Cash::EgyptianPound => format!("ج.م"),                     // EGP  ج.م
            Cash::EthiopianBirr => format!("Br"),                     // ETB  Br
            Cash::Euro => format!("€"),                              // EUR  €
            Cash::FijianDollar => format!("$"),                      // FJD  $
            Cash::GambianDalasi => format!("D"),                     // GMD  D
            Cash::GeorgianLari => format!("₾"),                      // GEL  ₾
            Cash::GhanaianCedi => format!("₵"),                      // GHS  ₵
            Cash::GuatemalanQuetzal => format!("Q"),                 // GTQ  Q
            Cash::GuineanFranc => format!("Fr"),                      // GNF  Fr
            Cash::GuyaneseDollar => format!("$"),                    // GYD  $
            Cash::HaitianGourde => format!("G"),                     // HTG  G
            Cash::HonduranLempira => format!("L"),                   // HNL  L
            Cash::HungarianForint => format!("Ft"),                   // HUF  Ft
            Cash::IcelandicKrona => format!("kr"),                    // ISK  kr
            Cash::IndianRupee => format!("₹"),                       // INR  ₹
            Cash::IndonesianRupiah => format!("Rp"),                  // IDR  Rp
            Cash::IranianRial => format!("﷼"),                       // IRR  ﷼
            Cash::IraqiDinar => format!("ع.د"),                        // IQD  ع.د
            Cash::IsraeliNewShekel => format!("₪"),                  // ILS  ₪
            Cash::JamaicanDollar => format!("$"),                    // JMD  $
            Cash::JapaneseYen => format!("円"),                       // JPY  円
            Cash::KazakhstaniTenge => format!("₸"),                  // KZT  ₸
            Cash::KenyanShilling => format!("Sh"),                    // KES  Sh
            Cash::KyrgyzstaniSom => format!("c"),                    // KGS  c
            Cash::LaoKip => format!("₭"),                            // LAK  ₭
            Cash::LiberianDollar => format!("$"),                    // LRD  $
            Cash::LibyanDinar => format!("ل.د"),                       // LYD  ل.د
            Cash::MacedonianDenar => format!("ден"),                   // MKD  ден
            Cash::MalagasyAriary => format!("Ar"),                    // MGA  Ar
            Cash::MalawianKwacha => format!("MK"),                    // MWK  MK
            Cash::MalaysianRinggit => format!("RM"),                  // MYR  RM
            Cash::MauritanianOuguiya => format!("UM"),                // MRU  UM
            Cash::MauritianRupee => format!("Rs"),                    // MUR  Rs
            Cash::MexicanPeso => format!("$"),                       // MXN  $
            Cash::MoldovanLeu => format!("L"),                       // MDL  L
            Cash::MongolianTogrog => format!("₮"),                   // MNT  ₮
            Cash::MozambicanMetical => format!("MT"),                 // MZN  MT
            Cash::NewTaiwanDollar => format!("$"),                   // TWD  $
            Cash::NewZealandDollar => format!("$"),                  // NZD  $
            Cash::NicaraguanCordoba => format!("C$"),                 // NIO  C$
            Cash::NigerianNaira => format!("₦"),                     // NGN  ₦
            Cash::NorthKoreanWon => format!("₩"),                    // KPW  ₩
            Cash::NorwegianKrone => format!("kr"),                    // NOK  kr
            Cash::PakistaniRupee => format!("Rs"),                    // PKR  Rs
            Cash::PapuaNewGuineanKina => format!("K"),               // PGK  K
            Cash::ParaguayanGuarani => format!("₲"),                 // PYG  ₲
            Cash::PeruvianSol => format!("S/."),                       // PEN  S/.
            Cash::PhilippinePeso => format!("₱"),                    // PHP  ₱
            Cash::PolishZloty => format!("zł"),                       // PLN  zł
            Cash::RomanianLeu => format!("lei"),                       // RON  lei
            Cash::RwandanFranc => format!("Fr"),                      // RWF  Fr
            Cash::RussianRuble => format!("₽"),                      // RUB  ₽
            Cash::SamoanTala => format!("T"),                        // WST  T
            Cash::SerbianDinar => format!("дин."),                      // RSD  дин.
            Cash::SeychelloisRupee => format!("Rs"),                  // SCR  Rs
            Cash::SierraLeoneanLeone => format!("Le"),                // SLL  Le
            Cash::SingaporeDollar => format!("$"),                   // SGD  $
            Cash::SolomonIslandsDollar => format!("$"),              // SBD  $
            Cash::SomaliShilling => format!("Sh"),                    // SOS  Sh
            Cash::SomalilandShilling => format!("Sl"),                // SLS  Sl
            Cash::SouthAfricanRand => format!("R"),                  // ZAR  R
            Cash::SouthKoreanWon => format!("₩"),                    // KRW  ₩
            Cash::SouthSudanesePound => format!("£"),                // SSP  £
            Cash::SriLankanRupee => format!("ரூ"),                    // LKR  ரூ
            Cash::SudanesePound => format!(""),                     // SDG  ج.س.
            Cash::SurinameseDollar => format!("ج.س."),                  // SRD  $
            Cash::SwedishKrona => format!("$"),                      // SEK  kr
            Cash::SwissFranc => format!("Fr."),                        // CHF  Fr.
            Cash::SyrianPound => format!("ل.س"),                       // SYP  ل.س
            Cash::TajikistaniSomoni => format!("с."),                 // TJS  с.
            Cash::TanzanianShilling => format!("Sh"),                 // TZS  Sh
            Cash::ThaiBaht => format!("฿"),                          // THB  ฿
            Cash::TonganPaAnga => format!("T$"),                      // TOP  T$
            Cash::TransnistrianRuble => format!("p."),                // PRB  p.
            Cash::TrinidadandTobagoDollar => format!("$"),           // TTD  $
            Cash::TunisianDinar => format!("د.ت"),                     // TND  د.ت
            Cash::TurkishLira => format!("₺"),                       // TRY  ₺
            Cash::TurkmenistanManat => format!("m."),                 // TMT  m.
            Cash::UgandanShilling => format!("USh"),                   // UGX  USh
            Cash::UkrainianHryvnia => format!("₴"),                  // UAH  ₴
            Cash::UnitedArabEmiratesDirham => format!("د.إ"),          // AED د.إ
            Cash::UnitedStatesDollar => format!("$"),                // USD  $
            Cash::UruguayanPeso => format!("$"),                     // UYU  $
            Cash::UzbekistaniSom => format!("Sʻ"),                    // UZS  Sʻ
            Cash::VanuatuVatu => format!("Vt"),                       // VUV  Vt
            Cash::VenezuelanBolivarSoberano => format!("Bs."),         // VES Bs.
            Cash::VietnameseDong => format!("₫"),                    // VND  ₫
            Cash::YemeniRial => format!("﷼"),                        // YER  ﷼
            Cash::ZambianKwacha => format!("ZK"),                     // ZMW  ZK
            Cash::RTGSdollar => format!("RTGS$"),                        // ZWB  RTGS$
        }
    }

    pub fn ISO_code(&self) -> String {
        match self {
            Cash::AfghanAfghani => format!("AFN"),                     // AFN  ؋
            Cash::AlbanianLek => format!("ALL"),                       // ALL  L
            Cash::AlgerianDinar => format!("DZD"),                     // DZD  د.ج
            Cash::AngolanKwanza => format!("AOA"),                     // AOA  Kz
            Cash::ArgentinePeso => format!("ARS"),                     // ARS  $
            Cash::ArmenianDram => format!("AMD"),                      // AMD  ֏
            Cash::AustralianDollar => format!("AUD"),                  // AUD  $
            Cash::AzerbaijaniManat => format!("AZN"),                  // AZN  ₼
            Cash::BangladeshiTaka => format!("BDT"),                   // BDT  ৳
            Cash::BelarusianRuble => format!("BYN"),                   // BYN  Br
            Cash::BolivianBoliviano => format!("BOB"),                 // BOB  Bs.
            Cash::BotswanaPula => format!("BWP"),                      // BWP  P
            Cash::BrazilianReal => format!("BRL"),                     // BRL  R$
            Cash::BritishPound => format!("GBP"),                      // GBP  £
            Cash::BurmeseKyat => format!("MMK"),                       // MMK  Ks
            Cash::BurundianFranc => format!("BIF"),                    // BIF  Fr
            Cash::CambodianRiel => format!("KHR"),                     // KHR  ៛
            Cash::CanadianDollar => format!("CAD"),                    // CAD  $
            Cash::ChileanPeso => format!("CLP"),                       // CLP  $
            Cash::ChineseYuan => format!("CNY"),                       // CNY  元
            Cash::ColombianPeso => format!("COP"),                     // COP  $
            Cash::CongoleseFranc => format!("CDF"),                    // CDF  Fr
            Cash::CostaRicanColon => format!("CRC"),                   // CRC  ₡
            Cash::CubanPeso => format!("CUP"),                         // CUP  $
            Cash::CzechKoruna => format!("CZK"),                       // CZK  Kč
            Cash::DominicanPeso => format!("DOP"),                     // DOP  RD$
            Cash::EgyptianPound => format!("EGP"),                     // EGP  ج.م
            Cash::EthiopianBirr => format!("ETB"),                     // ETB  Br
            Cash::Euro => format!("EUR"),                              // EUR  €
            Cash::FijianDollar => format!("FJD"),                      // FJD  $
            Cash::GambianDalasi => format!("GMD"),                     // GMD  D
            Cash::GeorgianLari => format!("GEL"),                      // GEL  ₾
            Cash::GhanaianCedi => format!("GHS"),                      // GHS  ₵
            Cash::GuatemalanQuetzal => format!("GTQ"),                 // GTQ  Q
            Cash::GuineanFranc => format!("GNF"),                      // GNF  Fr
            Cash::GuyaneseDollar => format!("GYD"),                    // GYD  $
            Cash::HaitianGourde => format!("HTG"),                     // HTG  G
            Cash::HonduranLempira => format!("HNL"),                   // HNL  L
            Cash::HungarianForint => format!("HUF"),                   // HUF  Ft
            Cash::IcelandicKrona => format!("ISK"),                    // ISK  kr
            Cash::IndianRupee => format!("INR"),                       // INR  ₹
            Cash::IndonesianRupiah => format!("IDR"),                  // IDR  Rp
            Cash::IranianRial => format!("IRR"),                       // IRR  ﷼
            Cash::IraqiDinar => format!("IQD"),                        // IQD  ع.د
            Cash::IsraeliNewShekel => format!("ILS"),                  // ILS  ₪
            Cash::JamaicanDollar => format!("JMD"),                    // JMD  $
            Cash::JapaneseYen => format!("JPY"),                       // JPY  円
            Cash::KazakhstaniTenge => format!("KZT"),                  // KZT  ₸
            Cash::KenyanShilling => format!("KES"),                    // KES  Sh
            Cash::KyrgyzstaniSom => format!("KGS"),                    // KGS  c
            Cash::LaoKip => format!("LAK"),                            // LAK  ₭
            Cash::LiberianDollar => format!("LRD"),                    // LRD  $
            Cash::LibyanDinar => format!("LYD"),                       // LYD  ل.د
            Cash::MacedonianDenar => format!("MKD"),                   // MKD  ден
            Cash::MalagasyAriary => format!("MGA"),                    // MGA  Ar
            Cash::MalawianKwacha => format!("MWK"),                    // MWK  MK
            Cash::MalaysianRinggit => format!("MYR"),                  // MYR  RM
            Cash::MauritanianOuguiya => format!("MRU"),                // MRU  UM
            Cash::MauritianRupee => format!("MUR"),                    // MUR  Rs
            Cash::MexicanPeso => format!("MXN"),                       // MXN  $
            Cash::MoldovanLeu => format!("MDL"),                       // MDL  L
            Cash::MongolianTogrog => format!("MNT"),                   // MNT  ₮
            Cash::MozambicanMetical => format!("MZN"),                 // MZN  MT
            Cash::NewTaiwanDollar => format!("TWD"),                   // TWD  $
            Cash::NewZealandDollar => format!("NZD"),                  // NZD  $
            Cash::NicaraguanCordoba => format!("NIO"),                 // NIO  C$
            Cash::NigerianNaira => format!("NGN"),                     // NGN  ₦
            Cash::NorthKoreanWon => format!("KPW"),                    // KPW  ₩
            Cash::NorwegianKrone => format!("NOK"),                    // NOK  kr
            Cash::PakistaniRupee => format!("PKR"),                    // PKR  Rs
            Cash::PapuaNewGuineanKina => format!("PGK"),               // PGK  K
            Cash::ParaguayanGuarani => format!("PYG"),                 // PYG  ₲
            Cash::PeruvianSol => format!("PEN"),                       // PEN  S/.
            Cash::PhilippinePeso => format!("PHP"),                    // PHP  ₱
            Cash::PolishZloty => format!("PLN"),                       // PLN  zł
            Cash::RomanianLeu => format!("RON"),                       // RON  lei
            Cash::RwandanFranc => format!("RWF"),                      // RWF  Fr
            Cash::RussianRuble => format!("RUB"),                      // RUB  ₽
            Cash::SamoanTala => format!("WST"),                        // WST  T
            Cash::SerbianDinar => format!("RSD"),                      // RSD  дин.
            Cash::SeychelloisRupee => format!("SCR"),                  // SCR  Rs
            Cash::SierraLeoneanLeone => format!("SLL"),                // SLL  Le
            Cash::SingaporeDollar => format!("SGD"),                   // SGD  $
            Cash::SolomonIslandsDollar => format!("SBD"),              // SBD  $
            Cash::SomaliShilling => format!("SOS"),                    // SOS  Sh
            Cash::SomalilandShilling => format!("SLS"),                // SLS  Sl
            Cash::SouthAfricanRand => format!("ZAR"),                  // ZAR  R
            Cash::SouthKoreanWon => format!("KRW"),                    // KRW  ₩
            Cash::SouthSudanesePound => format!("SSP"),                // SSP  £
            Cash::SriLankanRupee => format!("LKR"),                    // LKR  ரூ
            Cash::SudanesePound => format!("SDG"),                     // SDG  ج.س.
            Cash::SurinameseDollar => format!("SRD"),                  // SRD  $
            Cash::SwedishKrona => format!("SEK"),                      // SEK  kr
            Cash::SwissFranc => format!("CHF"),                        // CHF  Fr.
            Cash::SyrianPound => format!("SYP"),                       // SYP  ل.س
            Cash::TajikistaniSomoni => format!("TJS"),                 // TJS  с.
            Cash::TanzanianShilling => format!("TZS"),                 // TZS  Sh
            Cash::ThaiBaht => format!("THB"),                          // THB  ฿
            Cash::TonganPaAnga => format!("TOP"),                      // TOP  T$
            Cash::TransnistrianRuble => format!("PRB"),                // PRB  p.
            Cash::TrinidadandTobagoDollar => format!("TTD"),           // TTD  $
            Cash::TunisianDinar => format!("TND"),                     // TND  د.ت
            Cash::TurkishLira => format!("TRY"),                       // TRY  ₺
            Cash::TurkmenistanManat => format!("TMT"),                 // TMT  m.
            Cash::UgandanShilling => format!("UGX"),                   // UGX  USh
            Cash::UkrainianHryvnia => format!("UAH"),                  // UAH  ₴
            Cash::UnitedArabEmiratesDirham => format!("AED"),          // AED د.إ
            Cash::UnitedStatesDollar => format!("USD"),                // USD  $
            Cash::UruguayanPeso => format!("UYU"),                     // UYU  $
            Cash::UzbekistaniSom => format!("UZS"),                    // UZS  Sʻ
            Cash::VanuatuVatu => format!("VUV"),                       // VUV  Vt
            Cash::VenezuelanBolivarSoberano => format!("VES"),         // VES Bs.
            Cash::VietnameseDong => format!("VND"),                    // VND  ₫
            Cash::YemeniRial => format!("YER"),                        // YER  ﷼
            Cash::ZambianKwacha => format!("ZMW"),                     // ZMW  ZK
            Cash::RTGSdollar => format!("ZWB"),                        // ZWB  RTGS$
        }
    }
}