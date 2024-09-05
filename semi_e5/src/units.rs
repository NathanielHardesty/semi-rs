//! # UNITS OF MEASURE
//! **Based on SEMI E5§12**
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## TO BE DONE
//! 
//! - Fully implement this module.

pub struct Unit {
  pub identifier: Identifier,
  pub exponent: Option<i64>,
}

pub enum Identifier {
  // ==== UNITLESS ==========================================================
  None                                 , //Null String
  //                                     ===== LINEAR SCALING ===============
  Percent                              , //%      | 1/100        |
  PartsPerMillion                      , //ppm    | 1/1,000,000  |
  //                                     ===== LOGARITHMIC SCALING ==========
  Bel                  (Option<Prefix>), //B      |              |
  Neper                (Option<Prefix>), //Np     | 0.1151       | dB
  PH                                   , //pH     |              |
  // ===== BASE UNITS =======================================================
  // T+1                                 ===== TIME =========================
  Second               (Option<Prefix>), //s      |              | SI
  Minute                               , //min    | 60           | s
  Hour                                 , //h      | 60           | min
  DayMeanSolar                         , //d      | 24           | h
  Month                                , //mo     |              |
  Year                                 , //yr     |              |
  //     L+1                             ===== LENGTH =======================
  Meter                (Option<Prefix>), //m      |              | SI
  Angstrom             (Option<Prefix>), //Ang    | 1e-10        | m
  Micron                               , //um     | 1e-6         | m
  MilliMicron                          , //nm     | 1e-9         | m
  NauticalMile                         , //nmi    | 1852         | m
  Inch                                 , //in     | 25.4         | mm
  Foot                                 , //ft     | 12           | in
  Mil                                  , //mil    | 1e-3         | in
  Mile                                 , //mile   | 5280         | ft
  //         M+1                         ===== MASS =========================
  Gram                 (Option<Prefix>), //g      |              | SI
  AtomicMass                           , //u      | 1.660531e-27 | kg
  Slug                                 , //slug   | 14.4939      | kg
  Pound                                , //lb     | 0.0310810    | slug
  //             I+1                     ===== ELECTRIC CURRENT =============
  Ampere               (Option<Prefix>), //A      |              | SI
  //                 H+1                 ===== TEMPERATURE ==================
  Kelvin                               , //K      |              | SI
  DegreeCelsius                        , //degC   |              |
  DegreeFarenheit                      , //degF   |              |
  //                     N+1             ===== AMOUNT OF SUBSTANCE ==========
  Mole                                 , //mol    | 6.02252e23   | SI
  //                         J+1         ===== LUMINOUS INTENSITY ===========
  Candela              (Option<Prefix>), //cd     |              | SI
  //                             P+1     ===== PLANAR ANGLE =================
  Radian               (Option<Prefix>), //rad    |              | SI
  Cycle                (Option<Prefix>), //c      | 2*pi         | rad
  Revolution                           , //r      | 1            | c
  DegreePlanar                         , //deg    | pi/180       | rad
  MinutePlanar                         , //mins   | 1/60         | deg
  SecondPlanar                         , //sec    | 1/60         | mins
  //                                 S+1 ===== SOLID ANGLE ==================
  Steradian            (Option<Prefix>), //Sr     |              | SI
  // ===== KINEMATICS =======================================================
  // T-1                         P+1     ===== FREQUENCY ====================
  Hertz                (Option<Prefix>), //Hz     | 1            | c/s
  Becquerel            (Option<Prefix>), //Bq     | 1            | Hz
  Curie                                , //Ci     | 3.7e10       | Bq
  // T-1 L+1                             ===== VELOCITY =====================
  Knot                                 , //kn     | 1            | nmi/h
  // T-2 L+1                             ===== ACCELERATION =================
  Gal                  (Option<Prefix>), //Gal    | 1            | cm/s^2
  //     L+2                             ===== AREA =========================
  Barn                 (Option<Prefix>), //barn   | 1e-28        | m^2
  Darcy                                , //D      | 0.986923     | um^2
  // T-1 L+2                             ===== KINEMATIC VISCOSITY ==========
  Stokes               (Option<Prefix>), //St     | 1            | cm^2/s
  //     L+3                             ===== VOLUME =======================
  Liter                (Option<Prefix>), //l      | 1e-3         | m^3
  Barrel                               , //bbl    | 158.99       | l
  Gallon                               , //gal    | 3.7854       | l
  GallonUK                             , //galUK  | 4.5461       | l
  PintUK                               , //ptUK   | 0.56826      | l
  PintUSDry                            , //ptUS   | 0.55061      | l
  PintUSLiquid                         , //pt     | 0.47318      | l
  QuartUK                              , //qtUK   | 1.1365       | l
  QuartUSDry                           , //qtUS   | 1.1012       | l
  QuartUSLiquid                        , //qt     | 0.94635      | l
  // T-1 L+3                             ===== FLOW =========================
  StandardCubicCentimeterPerMinute     , //sccm   | 1            | cm^3/min
  StandardLiterPerMinute               , //slpm   | 1            | l/min
  // ===== MECHANICS ========================================================
  // T-2 L+1 M+1                         ===== FORCE ========================
  Newton               (Option<Prefix>), //N      | 1            | kg*m/s^2
  Dyne                 (Option<Prefix>), //dyn    | 1e-5         | N
  GramForce            (Option<Prefix>), //gf     | 9.80665e-3   | N
  MetricTon                            , //t      | 10^3         | kgf
  PoundForce                           , //lbf    | 4.4482217    | N
  TonShort                             , //ton    | 2000         | lbf
  KiloPoundForce                       , //klbf   | 1000         | lbf
  Poundal                              , //pdl    | 0.0310810    | lbf
  OunceAvoirdupois                     , //oz     | 1/16         | lbf
  Grain                                , //gr     | 0.0022857143 | oz
  // T-2 L+2 M+1                         ===== ENERGY =======================
  Joule                (Option<Prefix>), //J      | 1            | N*m
  WattHour             (Option<Prefix>), //Wh     | 3600         | J
  BritishThermal                       , //Btu    | 1054.35      | J
  Therm                                , //thm    | 1e5          | Btu
  CalorieInternational (Option<Prefix>), //callIT | 4.1868       | J
  Calorie              (Option<Prefix>), //cal    | 4.1840       | J
  ElectronVolt         (Option<Prefix>), //eV     | 1.60209e-19  | J
  Erg                  (Option<Prefix>), //erg    | 1e-7         | J
  // T-3 L+2 M+1                         ===== POWER ========================
  Watt                 (Option<Prefix>), //W      | 1            | J/s
  Horsepower                           , //hp     | 746          | W
  Var                  (Option<Prefix>), //var    |              |
  // T-1 L-1 M+1                         ===== DYNAMIC VISCOSITY ============
  Poise                (Option<Prefix>), //P      , 36           | kg/m*s
  // T-2 L-1 M+1                         ===== PRESSURE =====================
  Pascal               (Option<Prefix>), //Pa     | 1            | N/m^2
  Bar                  (Option<Prefix>), //bar    | 100          | kPa
  AtmosphereStandard                   , //atm    | 101.325      | Pa
  AtmosphereTechnical                  , //at     | 1            | kgf/cm^2
  InchMercury                          , //inHg   | 3386.4       | Pa
  InchWater                            , //inH2O  | 249.09       | Pa
  MicronMercury                        , //umHg   | 133.32e-3    | Pa
  MilliMeterMercury                    , //mmHg   | 133.322      | Pa
  Torr                 (Option<Prefix>), //torr   | 1            | mmHg
  // ===== ELECTROMAGNETISM =================================================
  // T+1         I+1                     ===== ELECTRIC CHARGE ==============
  Coulomb              (Option<Prefix>), //C      | 1            | A*s
  // T-1         I+1                     ===== MAGNETIC FIELD STRENGTH ======
  Oersted              (Option<Prefix>), //Oe     | 79.477472    | A/m
  // T+3 L-2 M-1 I+2                     ===== CONDUCTANCE ==================
  Siemens              (Option<Prefix>), //S      | 1            | ohm^-1
  Mho                  (Option<Prefix>), //mho    | 1            | S
  // T+4 L-2 M-2 I+2                     ===== CAPACITANCE ==================
  Farad                (Option<Prefix>), //F      | 1            | A*s/V
  // T-2     M+1 I-1                     ===== MAGNETIC FLUX DENSITY ========
  Tesla                (Option<Prefix>), //T      | 1            | N/A*m
  Gauss                (Option<Prefix>), //G      | 1            | Mx/cm^2
  // T-2 L+2 M+2 I-1                     ===== MAGNETIC FLUX ================
  Weber                (Option<Prefix>), //Wb     | 1            | V*s
  Maxwell              (Option<Prefix>), //Mx     | 1e-8         | Wb
  // T-3 L+2 M+2 I-1                     ===== VOLTAGE ======================
  Volt                 (Option<Prefix>), //V      | 1            | W/A
  // T-2 L+2 M+1 I-2                     ===== INDUCTANCE ===================
  Henry                (Option<Prefix>), //H      | 1            | V*s/A
  // T-3 L+2 M+1 I-2                     ===== RESISTANCE ===================
  Ohm                  (Option<Prefix>), //ohm    | 1            | V/A
  //             I+1             P+1     ===== MAGNETOMOTIVE FORCE ==========
  AmpereTurn           (Option<Prefix>), //AT     | 1            | A*c
  Gilbert              (Option<Prefix>), //Gb     | 10/4*pi      | AT
  // ===== PHOTOMETRY =======================================================
  //                         J+1     S+1 ===== LUMINOUS FLUX ================
  Lumen                (Option<Prefix>), //lm     | 1            | cd*sr
  //     L-2                 J+1         ===== LUMINANCE ====================
  Nit                  (Option<Prefix>), //nt     | 1            | cd/m^2
  Stilb                (Option<Prefix>), //sb     | 1            | cd/cm^2
  Lambert              (Option<Prefix>), //L      | 1/pi         | cd/cm^2
  FootLambert                          , //FL     | 1/pi         | cd/ft^2
  //     L-2                 J+1     S-1 ===== ILLUMINANCE ==================
  Lux                  (Option<Prefix>), //lx     | 1            | lm/m^2
  Phot                 (Option<Prefix>), //ph     | 1            | lm/cm^2
  FootCandle                           , //Fc     | 1            | lm/ft^2
  // ===== RADIOACTIVITY ====================================================
  // T-2 L+2                             ===== ABSORBED DOSE ================
  Sievert              (Option<Prefix>), //Sv     | 1            | J/kg
  Rem                  (Option<Prefix>), //rem    | 1e-2         | Sv
  Gray                 (Option<Prefix>), //Gy     | 1            | J/kg
  Rad                  (Option<Prefix>), //rd     | 1e-2         | Gy
  // T+1     M-1 I+1                     ===== RADIATION EXPOSURE ===========
  Roentgen                             , //R      | 2.58e-4      | C/kg
  // ===== INFORMATION THEORY ===============================================
  //                                     ===== DATA =========================
  Bit                  (Option<Prefix>), //bit    |              |
  Byte                 (Option<Prefix>), //byte   | 8            | bit
  // T-1                                 ===== DATA RATE ====================
  Baud                 (Option<Prefix>), //Bd     | 1            | bit/s
  // ===== SECS SPECIAL UNITS ===============================================
  Ion                                  , //ion       | Atom that carries an electric charge as a result of losing or gaining electrons.
  Substrate                            , //substrate | Entity of material being operated on, processed, or fabricated.
  Ingot                                , //ing       | Entity of semiconductor manufacture from which wafers are made.
  Wafer                                , //wfr       | Entity of material on which semiconductor devices are fabricated.
  Die                                  , //die       | Individual integrated circuit both on a wafer and after wafer separation. Also known as bar or chip.
  Package                              , //pkg       | Individual entity both as a place for the die to reside and as a completed unit.
  Lot                                  , //lot       | Grouping of material which is undergoing the same processing operations. The amount of material represented by "1 lot" is situational.
  Boat                 (Option<Suffix>), //boat      | Holder for wafers or packages with discrete positions, whose capacity is specified by the suffix.
  Carrier              (Option<Suffix>), //carrier   | Holder for substrates, wafers, or wafer frames, whose capacity is specified by the suffix.
  Cassette             (Option<Suffix>), //css       | Holder for wafers or wafer frames, whose capacity is specified by the suffix.
  LeadFrame            (Option<Suffix>), //ldfr      | Structure for leads which is removed after packaging, whose capacity is specified by the suffix. May be a fixed length or a reel.
  Magazine             (Option<Suffix>), //mgz       | Holder for fixed length leadframes, whose capacity is specified by the suffix.
  Plate                (Option<Suffix>), //plt       | Temporary fixture used to hold die during assembly, whose capacity is specified by the suffix.
  Tube                 (Option<Suffix>), //tube      | Holder for packages arranged in a flow, whose capacity is specified by the suffix.
  WaferFrame           (Option<Suffix>), //wffr      | Temporary fixture for wafers, whose capacity is specified by the suffix.
}

pub enum Prefix {
  Exa,   //E  | 1e18
  Peta,  //P  | 1e15
  Tera,  //T  | 1e12
  Giga,  //G  | 1e9
  Mega,  //M  | 1e6
  Kilo,  //k  | 1e3
  Hecto, //h  | 1e2
  Deca,  //d  | 1e1
  Deci,  //da | 1e-1
  Centi, //c  | 1e-2
  Milli, //m  | 1e-3
  Micro, //u  | 1e-6
  Nano,  //n  | 1e-9
  Pico,  //p  | 1e-12
  Femto, //f  | 1e-15
  Atto,  //a  | 1e-18
}

pub struct Suffix(pub u64);
