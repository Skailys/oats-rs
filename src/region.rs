#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Region {
    /// Solomon Islands
    AG = 0,

    /// Nauru
    AN = 1,

    /// Papua New Guinea
    AY = 2,

    /// Greenland
    BG = 3,

    /// Iceland
    BI = 4,

    /// Kosovo
    BK = 5,

    /// Canada
    C = 6,

    /// Algeria
    DA = 7,

    /// Benin
    DB = 8,

    /// Burkina Faso
    DF = 9,

    /// Ghana
    DG = 10,

    /// Côte d'Ivoire
    DI = 11,

    /// Nigeria
    DN = 12,

    /// Niger
    DR = 13,

    /// Tunisia
    DT = 14,

    /// Togo
    DX = 15,

    /// Belgium
    EB = 16,

    /// Germany (civil)
    ED = 17,

    /// Estonia
    EE = 18,

    /// Finland
    EF = 19,

    /// United Kingdom (and Crown Dependencies)
    EG = 20,

    /// Netherlands
    EH = 21,

    /// Ireland
    EI = 22,

    /// Denmark and the Faroe Islands
    EK = 23,

    /// Luxembourg
    EL = 24,

    /// Norway
    EN = 25,

    /// Poland
    EP = 26,

    /// Sweden
    ES = 27,

    /// Germany (military)
    ET = 28,

    /// Latvia
    EV = 29,

    /// Lithuania
    EY = 30,

    /// South Africa
    FA = 31,

    /// Botswana
    FB = 32,

    /// Republic of the Congo
    FC = 33,

    /// Eswatini
    FD = 34,

    /// Central African Republic
    FE = 35,

    /// Equatorial Guinea
    FG = 36,

    /// Saint Helena, Ascension and Tristan da Cunha
    FH = 37,

    /// Mauritius
    FI = 38,

    /// British Indian Ocean Territory
    FJ = 39,

    /// Cameroon
    FK = 40,

    /// Zambia
    FL = 41,

    /// Comoros, France (Mayotte and Réunion), and Madagascar
    FM = 42,

    /// Angola
    FN = 43,

    /// Gabon
    FO = 44,

    /// São Tomé and Príncipe
    FP = 45,

    /// Mozambique
    FQ = 46,

    /// Seychelles
    FS = 47,

    /// Chad
    FT = 48,

    /// Zimbabwe
    FV = 49,

    /// Malawi
    FW = 50,

    /// Lesotho
    FX = 51,

    /// Namibia
    FY = 52,

    /// Democratic Republic of the Congo
    FZ = 53,

    /// Mali
    GA = 54,

    /// The Gambia
    GB = 55,

    /// Spain (Canary Islands)
    GC = 56,

    /// Spain (Ceuta and Melilla)
    GE = 57,

    /// Sierra Leone
    GF = 58,

    /// Guinea-Bissau
    GG = 59,

    /// Liberia
    GL = 60,

    /// Morocco
    GM = 61,

    /// Senegal
    GO = 62,

    /// Mauritania
    GQ = 63,

    /// Western Sahara
    GS = 64,

    /// Guinea
    GU = 65,

    /// Cape Verde
    GV = 66,

    /// Ethiopia
    HA = 67,

    /// Burundi
    HB = 68,

    /// Somalia (including Somaliland)
    HC = 69,

    /// Djibouti
    HD = 70,

    /// Egypt
    HE = 71,

    /// Eritrea
    HH = 72,

    /// South Sudan
    HJ = 73,

    /// Kenya
    HK = 74,

    /// Libya
    HL = 75,

    /// Rwanda
    HR = 76,

    /// Sudan
    HS = 77,

    /// Tanzania
    HT = 78,

    /// Uganda
    HU = 79,

    /// Contiguous United States
    K = 80,

    /// Albania
    LA = 81,

    /// Bulgaria
    LB = 82,

    /// Cyprus
    LC = 83,

    /// Croatia
    LD = 84,

    /// Spain (mainland section and Balearic Islands)
    LE = 85,

    /// France (Metropolitan France; including Saint-Pierre and Miquelon)
    LF = 86,

    /// Greece
    LG = 87,

    /// Hungary
    LH = 88,

    /// Italy (and San Marino)
    LI = 89,

    /// Slovenia
    LJ = 90,

    /// Czech Republic
    LK = 91,

    /// Israel
    LL = 92,

    /// Malta
    LM = 93,

    /// Monaco
    LN = 94,

    /// Austria
    LO = 95,

    /// Portugal (including the Azores and Madeira)
    LP = 96,

    /// Bosnia and Herzegovina
    LQ = 97,

    /// Romania
    LR = 98,

    /// Switzerland
    LS = 99,

    /// Turkey
    LT = 100,

    /// Moldova
    LU = 101,

    /// Palestine/Palestinian territories
    LV = 102,

    /// North Macedonia
    LW = 103,

    /// Gibraltar
    LX = 104,

    /// Serbia and Montenegro
    LY = 105,

    /// Slovakia
    LZ = 106,

    /// Turks and Caicos Islands
    MB = 107,

    /// Dominican Republic
    MD = 108,

    /// Guatemala
    MG = 109,

    /// Honduras
    MH = 110,

    /// Jamaica
    MK = 111,

    /// Mexico
    MM = 112,

    /// Nicaragua
    MN = 113,

    /// Panama
    MP = 114,

    /// Costa Rica
    MR = 115,

    /// El Salvador
    MS = 116,

    /// Haiti
    MT = 117,

    /// Cuba
    MU = 118,

    /// Cayman Islands
    MW = 119,

    /// Bahamas
    MY = 120,

    /// Belize
    MZ = 121,

    /// Cook Islands
    NC = 122,

    /// Fiji, Tonga
    NF = 123,

    /// Kiribati (Gilbert Islands), Tuvalu
    NG = 124,

    /// Niue
    NI = 125,

    /// France (Wallis and Futuna)
    NL = 126,

    /// Samoa, United States (American Samoa)
    NS = 127,

    /// France (French Polynesia)
    NT = 128,

    /// Vanuatu
    NV = 129,

    /// France (New Caledonia)
    NW = 130,

    /// New Zealand, parts of Antarctica
    NZ = 131,

    /// Afghanistan
    OA = 132,

    /// Bahrain
    OB = 133,

    /// Saudi Arabia
    OE = 134,

    /// Iran
    OI = 135,

    /// Jordan and the West Bank
    OJ = 136,

    /// Kuwait
    OK = 137,

    /// Lebanon
    OL = 138,

    /// United Arab Emirates
    OM = 139,

    /// Oman
    OO = 140,

    /// Pakistan
    OP = 141,

    /// Iraq
    OR = 142,

    /// Syria
    OS = 143,

    /// Qatar
    OT = 144,

    /// Yemen
    OY = 145,

    /// US (Alaska) (also PF, PO and PP)
    PA = 146,

    /// US (Baker Island)
    PB = 147,

    /// Kiribati (Canton Airfield, Phoenix Islands)
    PC = 148,

    /// US (Alaska) (also PA, PO and PP)
    PF = 149,

    /// US (Guam, Northern Mariana Islands)
    PG = 150,

    /// US (Hawaii)
    PH = 151,

    /// US (Johnston Atoll)
    PJ = 152,

    /// Marshall Islands
    PK = 153,

    /// Kiribati (Line Islands)
    PL = 154,

    /// US (Midway Island)
    PM = 155,

    /// US (Alaska) (also PA, PF and PP)
    PO = 156,

    /// US (Alaska) (also PA, PF and PO)
    PP = 157,

    /// Federated States of Micronesia, Palau
    PT = 158,

    /// US (Wake Island)
    PW = 159,

    /// Republic of China (Taiwan)
    RC = 160,

    /// Japan (Mainland)
    RJ = 161,

    /// South Korea (Republic of Korea)
    RK = 162,

    /// Japan (Okinawa)
    RO = 163,

    /// Philippines
    RP = 164,

    /// Argentina (including parts of Antarctica)
    SA = 165,

    /// Brazil (also SD, SI, SJ, SN, SS and SW)
    SB = 166,

    /// Chile (including Easter Island and parts of Antarctica) (also SH)
    SC = 167,

    /// Brazil (also SB, SI, SJ, SN, SS and SW)
    SD = 168,

    /// Ecuador
    SE = 169,

    /// United Kingdom (Falkland Islands)
    SF = 170,

    /// Paraguay
    SG = 171,

    /// Chile (also SC)
    SH = 172,

    /// Brazil (also SB, SD, SJ, SN, SS and SW)
    SI = 173,

    /// Brazil (also SB, SD, SI, SN, SS and SW)
    SJ = 174,

    /// Colombia
    SK = 175,

    /// Bolivia
    SL = 176,

    /// Suriname
    SM = 177,

    /// Brazil (also SB, SD, SI, SJ, SS and SW)
    SN = 178,

    /// France (French Guiana)
    SO = 179,

    /// Peru
    SP = 180,

    /// Brazil (also SB, SD, SI, SJ, SN and SW)
    SS = 181,

    /// Uruguay
    SU = 182,

    /// Venezuela
    SV = 183,

    /// Brazil (also SB, SD, SI, SJ, SN and SS)
    SW = 184,

    /// Guyana
    SY = 185,

    /// Antigua and Barbuda
    TA = 186,

    /// Barbados
    TB = 187,

    /// Dominica
    TD = 188,

    /// France (Guadeloupe, Martinique, Saint Barthélemy, Saint Martin)
    TF = 189,

    /// Grenada
    TG = 190,

    /// US (U.S. Virgin Islands)
    TI = 191,

    /// US (Puerto Rico)
    TJ = 192,

    /// Saint Kitts and Nevis
    TK = 193,

    /// Saint Lucia
    TL = 194,

    /// Caribbean Netherlands, Aruba, Curaçao, Sint Maarten
    TN = 195,

    /// UK (Anguilla)
    TQ = 196,

    /// UK (Montserrat)
    TR = 197,

    /// Trinidad and Tobago
    TT = 198,

    /// UK (British Virgin Islands)
    TU = 199,

    /// Saint Vincent and the Grenadines
    TV = 200,

    /// UK (Bermuda)
    TX = 201,

    /// Russia (except as below)
    U = 202,

    /// Kazakhstan
    UA = 203,

    /// Azerbaijan
    UB = 204,

    /// Kyrgyzstan
    UC = 205,

    /// Armenia
    UD = 206,

    /// Georgia
    UG = 207,

    /// Ukraine
    UK = 208,

    /// Belarus and Russia (Kaliningrad Oblast)
    UM = 209,

    /// Tajikistan, Turkmenistan, Uzbekistan
    UT = 210,

    /// India (West India)
    VA = 211,

    /// Sri Lanka
    VC = 212,

    /// Cambodia
    VD = 213,

    /// India (East India)
    VE = 214,

    /// Bangladesh
    VG = 215,

    /// Hong Kong
    VH = 216,

    /// India (North India)
    VI = 217,

    /// Laos
    VL = 218,

    /// Macau
    VM = 219,

    /// Nepal
    VN = 220,

    /// India (South India)
    VO = 221,

    /// Bhutan
    VQ = 222,

    /// Maldives
    VR = 223,

    /// Thailand
    VT = 224,

    /// Vietnam
    VV = 225,

    /// Myanmar
    VY = 226,

    /// Indonesia (also WI, WQ and WR)
    WA = 227,

    /// Brunei, Malaysia (East Malaysia)
    WB = 228,

    /// Indonesia (also WA, WQ and WR)
    WI = 229,

    /// Malaysia (Peninsular Malaysia)
    WM = 230,

    /// Timor-Leste
    WP = 231,

    /// Indonesia (also WA, WI and WR)
    WQ = 232,

    /// Indonesia (also WA, WI and WQ)
    WR = 233,

    /// Singapore
    WS = 234,

    /// Australia (including Norfolk Island, Christmas Island, Cocos (Keeling) Islands and Australian Antarctic Territory)
    Y = 235,

    /// Mainland China (except ZK and ZM)
    Z = 236,

    /// North Korea
    ZK = 237,

    /// Mongolia
    ZM = 238,

}