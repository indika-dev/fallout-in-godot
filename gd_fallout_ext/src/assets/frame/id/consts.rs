use super::*;

#[allow(clippy::unusual_byte_groupings)]
impl FrameId {
    pub const BLANK: FrameId = FrameId::Generic(Generic(0x6000_000));
    pub const MOUSE_HEX: FrameId = FrameId::Generic(Generic(0x6000_001));
    pub const EGG: FrameId = FrameId::Generic(Generic(0x6000_002));
    pub const MOUSE_HEX_GREEN: FrameId = FrameId::Generic(Generic(0x6000_003));
    pub const ACTIONH: FrameId = FrameId::Generic(Generic(0x6000_004));
    pub const ACTIONM: FrameId = FrameId::Generic(Generic(0x6000_005));
    pub const BIG_RED_BUTTON_UP: FrameId = FrameId::Generic(Generic(0x6000_006));
    pub const BIG_RED_BUTTON_DOWN: FrameId = FrameId::Generic(Generic(0x6000_007));
    pub const SMALL_RED_BUTTON_UP: FrameId = FrameId::Generic(Generic(0x6000_008));
    pub const SMALL_RED_BUTTON_DOWN: FrameId = FrameId::Generic(Generic(0x6000_009));
    pub const MAP_BUTTON_DOWN: FrameId = FrameId::Generic(Generic(0x6000_00a));
    pub const AMAPHV: FrameId = FrameId::Generic(Generic(0x6000_00b));
    pub const MAPMK: FrameId = FrameId::Generic(Generic(0x6000_00c));
    pub const MAP_BUTTON_UP: FrameId = FrameId::Generic(Generic(0x6000_00d));
    pub const ENDTDN: FrameId = FrameId::Generic(Generic(0x6000_00e));
    pub const ENDTUP: FrameId = FrameId::Generic(Generic(0x6000_00f));
    pub const IFACE: FrameId = FrameId::Generic(Generic(0x6000_010));
    pub const OPTIONS_BUTTON_DOWN: FrameId = FrameId::Generic(Generic(0x6000_011));
    pub const OPTIONS_BUTTON_UP: FrameId = FrameId::Generic(Generic(0x6000_012));
    pub const TMAPDN: FrameId = FrameId::Generic(Generic(0x6000_013));
    pub const TMAPHV: FrameId = FrameId::Generic(Generic(0x6000_014));
    pub const TMAPUP: FrameId = FrameId::Generic(Generic(0x6000_015));
    pub const WMAPDN: FrameId = FrameId::Generic(Generic(0x6000_016));
    pub const WMAPHV: FrameId = FrameId::Generic(Generic(0x6000_017));
    pub const WMAPUP: FrameId = FrameId::Generic(Generic(0x6000_018));
    pub const LATTKBDN: FrameId = FrameId::Generic(Generic(0x6000_019));
    pub const LATTKBDS: FrameId = FrameId::Generic(Generic(0x6000_01a));
    pub const LATTKBUP: FrameId = FrameId::Generic(Generic(0x6000_01b));
    pub const LATTKSDN: FrameId = FrameId::Generic(Generic(0x6000_01c));
    pub const LATTKSDS: FrameId = FrameId::Generic(Generic(0x6000_01d));
    pub const LATTKSUP: FrameId = FrameId::Generic(Generic(0x6000_01e));
    pub const SINGLE_ATTACK_BUTTON_DOWN: FrameId = FrameId::Generic(Generic(0x6000_01f));
    pub const SINGLE_ATTACK_BUTTON_UP: FrameId = FrameId::Generic(Generic(0x6000_020));
    pub const SATTKSUP: FrameId = FrameId::Generic(Generic(0x6000_021));
    pub const UATTKBDN: FrameId = FrameId::Generic(Generic(0x6000_022));
    pub const UATTKBUP: FrameId = FrameId::Generic(Generic(0x6000_023));
    pub const UATTKSDN: FrameId = FrameId::Generic(Generic(0x6000_024));
    pub const UATTKSUP: FrameId = FrameId::Generic(Generic(0x6000_025));
    pub const KARMAFDR: FrameId = FrameId::Generic(Generic(0x6000_026));
    pub const TBLCRD: FrameId = FrameId::Generic(Generic(0x6000_027));
    pub const BURST: FrameId = FrameId::Generic(Generic(0x6000_028));
    pub const KICK: FrameId = FrameId::Generic(Generic(0x6000_029));
    pub const PUNCH: FrameId = FrameId::Generic(Generic(0x6000_02a));
    pub const SINGLE: FrameId = FrameId::Generic(Generic(0x6000_02b));
    pub const SWING: FrameId = FrameId::Generic(Generic(0x6000_02c));
    pub const THRUST: FrameId = FrameId::Generic(Generic(0x6000_02d));
    pub const INVENTORY_BUTTON_DOWN: FrameId = FrameId::Generic(Generic(0x6000_02e));
    pub const INVENTORY_BUTTON_UP: FrameId = FrameId::Generic(Generic(0x6000_02f));
    pub const INVENTORY_WINDOW: FrameId = FrameId::Generic(Generic(0x6000_030));
    pub const INVENTORY_SCROLL_UP_UP: FrameId = FrameId::Generic(Generic(0x6000_031));
    pub const INVENTORY_SCROLL_UP_DOWN: FrameId = FrameId::Generic(Generic(0x6000_032));
    pub const INVENTORY_SCROLL_DOWN_UP: FrameId = FrameId::Generic(Generic(0x6000_033));
    pub const INVENTORY_SCROLL_DOWN_DOWN: FrameId = FrameId::Generic(Generic(0x6000_034));
    pub const INVENTORY_SCROLL_UP_DISABLED: FrameId = FrameId::Generic(Generic(0x6000_035));
    pub const INVENTORY_SCROLL_DOWN_DISABLED: FrameId = FrameId::Generic(Generic(0x6000_036));
    pub const AUTOHI: FrameId = FrameId::Generic(Generic(0x6000_037));
    pub const CHARACTER_BUTTON_DOWN: FrameId = FrameId::Generic(Generic(0x6000_038));
    pub const CHARACTER_BUTTON_UP: FrameId = FrameId::Generic(Generic(0x6000_039));
    pub const PIP_BUTTON_DOWN: FrameId = FrameId::Generic(Generic(0x6000_03a));
    pub const PIP_BUTTON_UP: FrameId = FrameId::Generic(Generic(0x6000_03b));
    pub const TINREDDN: FrameId = FrameId::Generic(Generic(0x6000_03c));
    pub const TINREDUP: FrameId = FrameId::Generic(Generic(0x6000_03d));
    pub const TOWNDIM: FrameId = FrameId::Generic(Generic(0x6000_03e));
    pub const TOWNHI: FrameId = FrameId::Generic(Generic(0x6000_03f));
    pub const WORLDDIM: FrameId = FrameId::Generic(Generic(0x6000_040));
    pub const WORLDHI: FrameId = FrameId::Generic(Generic(0x6000_041));
    pub const LATURBDN: FrameId = FrameId::Generic(Generic(0x6000_042));
    pub const LATURBDS: FrameId = FrameId::Generic(Generic(0x6000_043));
    pub const LATURBUP: FrameId = FrameId::Generic(Generic(0x6000_044));
    pub const LATURSDN: FrameId = FrameId::Generic(Generic(0x6000_045));
    pub const LATURSDS: FrameId = FrameId::Generic(Generic(0x6000_046));
    pub const LATURSUP: FrameId = FrameId::Generic(Generic(0x6000_047));
    pub const SATURBDN: FrameId = FrameId::Generic(Generic(0x6000_048));
    pub const SATURBUP: FrameId = FrameId::Generic(Generic(0x6000_049));
    pub const SATURSUP: FrameId = FrameId::Generic(Generic(0x6000_04a));
    pub const UATURBDN: FrameId = FrameId::Generic(Generic(0x6000_04b));
    pub const UATURBUP: FrameId = FrameId::Generic(Generic(0x6000_04c));
    pub const UATURSDN: FrameId = FrameId::Generic(Generic(0x6000_04d));
    pub const UATURSUP: FrameId = FrameId::Generic(Generic(0x6000_04e));
    pub const SATURSDN: FrameId = FrameId::Generic(Generic(0x6000_04f));
    pub const ACTIONT: FrameId = FrameId::Generic(Generic(0x6000_050));
    pub const ACTIONI: FrameId = FrameId::Generic(Generic(0x6000_051));
    pub const NUMBERS: FrameId = FrameId::Generic(Generic(0x6000_052));
    pub const HLGRN: FrameId = FrameId::Generic(Generic(0x6000_053));
    pub const HLYEL: FrameId = FrameId::Generic(Generic(0x6000_054));
    pub const HLRED: FrameId = FrameId::Generic(Generic(0x6000_055));
    pub const PERKWIN: FrameId = FrameId::Generic(Generic(0x6000_056));
    pub const DI_BGDN1: FrameId = FrameId::Generic(Generic(0x6000_057));
    pub const DI_BGDN2: FrameId = FrameId::Generic(Generic(0x6000_058));
    pub const DI_BGUP1: FrameId = FrameId::Generic(Generic(0x6000_059));
    pub const DI_BGUP2: FrameId = FrameId::Generic(Generic(0x6000_05a));
    pub const DI_DONE1: FrameId = FrameId::Generic(Generic(0x6000_05b));
    pub const DI_DONE2: FrameId = FrameId::Generic(Generic(0x6000_05c));
    pub const DI_DOWN1: FrameId = FrameId::Generic(Generic(0x6000_05d));
    pub const DI_DOWN2: FrameId = FrameId::Generic(Generic(0x6000_05e));
    pub const DI_RDBT1: FrameId = FrameId::Generic(Generic(0x6000_05f));
    pub const DI_RDBT2: FrameId = FrameId::Generic(Generic(0x6000_060));
    pub const DI_REST1: FrameId = FrameId::Generic(Generic(0x6000_061));
    pub const DI_REST2: FrameId = FrameId::Generic(Generic(0x6000_062));
    pub const DI_TALK: FrameId = FrameId::Generic(Generic(0x6000_063));
    pub const DI_UP1: FrameId = FrameId::Generic(Generic(0x6000_064));
    pub const DI_UP2: FrameId = FrameId::Generic(Generic(0x6000_065));
    pub const REVIEW: FrameId = FrameId::Generic(Generic(0x6000_066));
    pub const ALLTLK: FrameId = FrameId::Generic(Generic(0x6000_067));
    pub const ENDANIM: FrameId = FrameId::Generic(Generic(0x6000_068));
    pub const ENDTURNU: FrameId = FrameId::Generic(Generic(0x6000_069));
    pub const ENDTURND: FrameId = FrameId::Generic(Generic(0x6000_06a));
    pub const ENDCMBTU: FrameId = FrameId::Generic(Generic(0x6000_06b));
    pub const ENDCMBTD: FrameId = FrameId::Generic(Generic(0x6000_06c));
    pub const ENDLTGRN: FrameId = FrameId::Generic(Generic(0x6000_06d));
    pub const ENDLTRED: FrameId = FrameId::Generic(Generic(0x6000_06e));
    pub const BARTER: FrameId = FrameId::Generic(Generic(0x6000_06f));
    pub const ABOUT: FrameId = FrameId::Generic(Generic(0x6000_070));
    pub const USE: FrameId = FrameId::Generic(Generic(0x6000_071));
    pub const LOOT: FrameId = FrameId::Generic(Generic(0x6000_072));
    pub const HILIGHT1: FrameId = FrameId::Generic(Generic(0x6000_073));
    pub const HILIGHT2: FrameId = FrameId::Generic(Generic(0x6000_074));
    pub const THROW: FrameId = FrameId::Generic(Generic(0x6000_075));
    pub const CALLED: FrameId = FrameId::Generic(Generic(0x6000_076));
    pub const SKILLDEX_BUTTON_DOWN: FrameId = FrameId::Generic(Generic(0x6000_077));
    pub const SKILLDEX_BUTTON_UP: FrameId = FrameId::Generic(Generic(0x6000_078));
    pub const SKILLDEX_WINDOW: FrameId = FrameId::Generic(Generic(0x6000_079));
    pub const SLU: FrameId = FrameId::Generic(Generic(0x6000_07a));
    pub const SLD: FrameId = FrameId::Generic(Generic(0x6000_07b));
    pub const SRU: FrameId = FrameId::Generic(Generic(0x6000_07c));
    pub const SRD: FrameId = FrameId::Generic(Generic(0x6000_07d));
    pub const WARNBOX: FrameId = FrameId::Generic(Generic(0x6000_07e));
    pub const PIP: FrameId = FrameId::Generic(Generic(0x6000_07f));
    pub const PIP2: FrameId = FrameId::Generic(Generic(0x6000_080));
    pub const MONTHS: FrameId = FrameId::Generic(Generic(0x6000_081));
    pub const NOTENUMS: FrameId = FrameId::Generic(Generic(0x6000_082));
    pub const ALARMIN: FrameId = FrameId::Generic(Generic(0x6000_083));
    pub const ALARMOUT: FrameId = FrameId::Generic(Generic(0x6000_084));
    pub const PIPX: FrameId = FrameId::Generic(Generic(0x6000_085));
    pub const MSEF002: FrameId = FrameId::Generic(Generic(0x6000_086));
    pub const WORLDMAP: FrameId = FrameId::Generic(Generic(0x6000_087));
    pub const WMAPBOX: FrameId = FrameId::Generic(Generic(0x6000_088));
    pub const WMAPLABS: FrameId = FrameId::Generic(Generic(0x6000_089));
    pub const WMAPLOC: FrameId = FrameId::Generic(Generic(0x6000_08a));
    pub const WMAPTARG: FrameId = FrameId::Generic(Generic(0x6000_08b));
    pub const MAINMENU: FrameId = FrameId::Generic(Generic(0x6000_08c));
    pub const EBUT_IN: FrameId = FrameId::Generic(Generic(0x6000_08d));
    pub const EBUT_OUT: FrameId = FrameId::Generic(Generic(0x6000_08e));
    pub const EL_BOS: FrameId = FrameId::Generic(Generic(0x6000_08f));
    pub const EL_MAST1: FrameId = FrameId::Generic(Generic(0x6000_090));
    pub const EL_MAST2: FrameId = FrameId::Generic(Generic(0x6000_091));
    pub const EL_MIL1: FrameId = FrameId::Generic(Generic(0x6000_092));
    pub const EL_MIL2: FrameId = FrameId::Generic(Generic(0x6000_093));
    pub const EL_VAULT: FrameId = FrameId::Generic(Generic(0x6000_094));
    pub const GAJ000: FrameId = FrameId::Generic(Generic(0x6000_095));
    pub const EL_BOS2: FrameId = FrameId::Generic(Generic(0x6000_096));
    pub const EL_MIL3: FrameId = FrameId::Generic(Generic(0x6000_097));
    pub const EL_MIL4: FrameId = FrameId::Generic(Generic(0x6000_098));
    pub const WMAPTRG2: FrameId = FrameId::Generic(Generic(0x6000_099));
    pub const WMAPFGT0: FrameId = FrameId::Generic(Generic(0x6000_09a));
    pub const WMAPFGT1: FrameId = FrameId::Generic(Generic(0x6000_09b));
    pub const TWNARROY: FrameId = FrameId::Generic(Generic(0x6000_09c));
    pub const TWNBROKN: FrameId = FrameId::Generic(Generic(0x6000_09d));
    pub const TWNKLAMA: FrameId = FrameId::Generic(Generic(0x6000_09e));
    pub const TWNMODOC: FrameId = FrameId::Generic(Generic(0x6000_09f));
    pub const TWNDEN: FrameId = FrameId::Generic(Generic(0x6000_0a0));
    pub const TWNGECKO: FrameId = FrameId::Generic(Generic(0x6000_0a1));
    pub const TWNNCR: FrameId = FrameId::Generic(Generic(0x6000_0a2));
    pub const TWNRENO: FrameId = FrameId::Generic(Generic(0x6000_0a3));
    pub const TWNSAD: FrameId = FrameId::Generic(Generic(0x6000_0a4));
    pub const TWNSANFR: FrameId = FrameId::Generic(Generic(0x6000_0a5));
    pub const TWNV13: FrameId = FrameId::Generic(Generic(0x6000_0a6));
    pub const TWNV15: FrameId = FrameId::Generic(Generic(0x6000_0a7));
    pub const HOTSPOT1: FrameId = FrameId::Generic(Generic(0x6000_0a8));
    pub const EDTRCRTE: FrameId = FrameId::Generic(Generic(0x6000_0a9));
    pub const BIG_NUMBERS: FrameId = FrameId::Generic(Generic(0x6000_0aa));
    pub const AUTOMAP: FrameId = FrameId::Generic(Generic(0x6000_0ab));
    pub const AUTOUP: FrameId = FrameId::Generic(Generic(0x6000_0ac));
    pub const AUTODWN: FrameId = FrameId::Generic(Generic(0x6000_0ad));
    pub const PICKCHAR: FrameId = FrameId::Generic(Generic(0x6000_0ae));
    pub const AGEMASK: FrameId = FrameId::Generic(Generic(0x6000_0af));
    pub const AGEOFF: FrameId = FrameId::Generic(Generic(0x6000_0b0));
    pub const EDTREDT: FrameId = FrameId::Generic(Generic(0x6000_0b1));
//    pub const KARMAFDR: Fid = Fid::Generic(GenericFid(0x6000_0b2));
    pub const KILLSFDR: FrameId = FrameId::Generic(Generic(0x6000_0b3));
    pub const PERKSFDR: FrameId = FrameId::Generic(Generic(0x6000_0b4));
    pub const DNARWOFF: FrameId = FrameId::Generic(Generic(0x6000_0b5));
    pub const DNARWON: FrameId = FrameId::Generic(Generic(0x6000_0b6));
    pub const NAMEMSK: FrameId = FrameId::Generic(Generic(0x6000_0b7));
    pub const NAMEON: FrameId = FrameId::Generic(Generic(0x6000_0b8));
    pub const NAMEOFF: FrameId = FrameId::Generic(Generic(0x6000_0b9));
    pub const FLDRMASK: FrameId = FrameId::Generic(Generic(0x6000_0ba));
    pub const SEXMASK: FrameId = FrameId::Generic(Generic(0x6000_0bb));
    pub const SEXOFF: FrameId = FrameId::Generic(Generic(0x6000_0bc));
    pub const SEXON: FrameId = FrameId::Generic(Generic(0x6000_0bd));
    pub const SLIDER: FrameId = FrameId::Generic(Generic(0x6000_0be));
    pub const BUTTON_MINUS_UP: FrameId = FrameId::Generic(Generic(0x6000_0bf));
    pub const BUTTON_MINUS_DOWN: FrameId = FrameId::Generic(Generic(0x6000_0c0));
    pub const BUTTON_PLUS_UP: FrameId = FrameId::Generic(Generic(0x6000_0c1));
    pub const BUTTON_PLUS_DOWN: FrameId = FrameId::Generic(Generic(0x6000_0c2));
    pub const STNEGOFF: FrameId = FrameId::Generic(Generic(0x6000_0c3));
    pub const STNEGON: FrameId = FrameId::Generic(Generic(0x6000_0c4));
    pub const STPLSOFF: FrameId = FrameId::Generic(Generic(0x6000_0c5));
    pub const STPLSON: FrameId = FrameId::Generic(Generic(0x6000_0c6));
    pub const UPARWOFF: FrameId = FrameId::Generic(Generic(0x6000_0c7));
    pub const UPARWON: FrameId = FrameId::Generic(Generic(0x6000_0c8));
    pub const COMBAT: FrameId = FrameId::Generic(Generic(0x6000_0c9));
    pub const STEALTH: FrameId = FrameId::Generic(Generic(0x6000_0ca));
    pub const DIPLOMAT: FrameId = FrameId::Generic(Generic(0x6000_0cb));
    pub const AGEON: FrameId = FrameId::Generic(Generic(0x6000_0cc));
    pub const AGEBOX: FrameId = FrameId::Generic(Generic(0x6000_0cd));
    pub const ATTRIBOX: FrameId = FrameId::Generic(Generic(0x6000_0ce));
    pub const ATTRIBWN: FrameId = FrameId::Generic(Generic(0x6000_0cf));
    pub const CHARWIN: FrameId = FrameId::Generic(Generic(0x6000_0d0));
    pub const DONEBOX: FrameId = FrameId::Generic(Generic(0x6000_0d1));
    pub const FEMOFF: FrameId = FrameId::Generic(Generic(0x6000_0d2));
    pub const FEMON: FrameId = FrameId::Generic(Generic(0x6000_0d3));
    pub const MALEOFF: FrameId = FrameId::Generic(Generic(0x6000_0d4));
    pub const MALEON: FrameId = FrameId::Generic(Generic(0x6000_0d5));
    pub const NAMEBOX: FrameId = FrameId::Generic(Generic(0x6000_0d6));
    pub const TGSKLOFF: FrameId = FrameId::Generic(Generic(0x6000_0d7));
    pub const TGSKLON: FrameId = FrameId::Generic(Generic(0x6000_0d8));
    pub const LGDIALOG: FrameId = FrameId::Generic(Generic(0x6000_0d9));
    pub const MEDIALOG: FrameId = FrameId::Generic(Generic(0x6000_0da));
    pub const BARARRWS: FrameId = FrameId::Generic(Generic(0x6000_0db));
    pub const OPBASE: FrameId = FrameId::Generic(Generic(0x6000_0dc));
    pub const OPBTNOFF: FrameId = FrameId::Generic(Generic(0x6000_0dd));
    pub const OPBTNON: FrameId = FrameId::Generic(Generic(0x6000_0de));
    pub const HOTSPOT2: FrameId = FrameId::Generic(Generic(0x6000_0df));
    pub const LOADBOX: FrameId = FrameId::Generic(Generic(0x6000_0e0));
    pub const SAVEBOX: FrameId = FrameId::Generic(Generic(0x6000_0e1));
    pub const BOMB1: FrameId = FrameId::Generic(Generic(0x6000_0e2));
    pub const UPSELL00: FrameId = FrameId::Generic(Generic(0x6000_0e3));
    pub const UPSELL01: FrameId = FrameId::Generic(Generic(0x6000_0e4));
    pub const UPSELL02: FrameId = FrameId::Generic(Generic(0x6000_0e5));
    pub const UPSELL03: FrameId = FrameId::Generic(Generic(0x6000_0e6));
    pub const UPSELL04: FrameId = FrameId::Generic(Generic(0x6000_0e7));
    pub const UPSELL05: FrameId = FrameId::Generic(Generic(0x6000_0e8));
    pub const UPSELL06: FrameId = FrameId::Generic(Generic(0x6000_0e9));
    pub const UPSELL07: FrameId = FrameId::Generic(Generic(0x6000_0ea));
    pub const UPSELL08: FrameId = FrameId::Generic(Generic(0x6000_0eb));
    pub const UPSELL09: FrameId = FrameId::Generic(Generic(0x6000_0ec));
    pub const LSGAME: FrameId = FrameId::Generic(Generic(0x6000_0ed));
    pub const LSGBOX: FrameId = FrameId::Generic(Generic(0x6000_0ee));
    pub const LSCOVER: FrameId = FrameId::Generic(Generic(0x6000_0ef));
    pub const PREFSCRN: FrameId = FrameId::Generic(Generic(0x6000_0f0));
    pub const PRFSLDOF: FrameId = FrameId::Generic(Generic(0x6000_0f1));
    pub const PRFBKNBS: FrameId = FrameId::Generic(Generic(0x6000_0f2));
    pub const PRFLKNBS: FrameId = FrameId::Generic(Generic(0x6000_0f3));
    pub const PRFXIN: FrameId = FrameId::Generic(Generic(0x6000_0f4));
    pub const PRFXOUT: FrameId = FrameId::Generic(Generic(0x6000_0f5));
    pub const PREFCVR: FrameId = FrameId::Generic(Generic(0x6000_0f6));
    pub const PRFSLDON: FrameId = FrameId::Generic(Generic(0x6000_0f7));
    pub const PREFTRCK: FrameId = FrameId::Generic(Generic(0x6000_0f8));
    pub const MOUSE_HEX_OUTLINE: FrameId = FrameId::Generic(Generic(0x6000_0f9));
    pub const ACTARROW: FrameId = FrameId::Generic(Generic(0x6000_0fa));
    pub const ACRSHAIR: FrameId = FrameId::Generic(Generic(0x6000_0fb));
    pub const CANCELH: FrameId = FrameId::Generic(Generic(0x6000_0fc));
    pub const CANCELN: FrameId = FrameId::Generic(Generic(0x6000_0fd));
    pub const DROPH: FrameId = FrameId::Generic(Generic(0x6000_0fe));
    pub const DROPN: FrameId = FrameId::Generic(Generic(0x6000_0ff));
    pub const INVENH: FrameId = FrameId::Generic(Generic(0x6000_100));
    pub const INVENN: FrameId = FrameId::Generic(Generic(0x6000_101));
    pub const LOOKH: FrameId = FrameId::Generic(Generic(0x6000_102));
    pub const LOOKN: FrameId = FrameId::Generic(Generic(0x6000_103));
    pub const ROTATEH: FrameId = FrameId::Generic(Generic(0x6000_104));
    pub const ROTATEN: FrameId = FrameId::Generic(Generic(0x6000_105));
    pub const TALKH: FrameId = FrameId::Generic(Generic(0x6000_106));
    pub const TALKN: FrameId = FrameId::Generic(Generic(0x6000_107));
    pub const USEGETH: FrameId = FrameId::Generic(Generic(0x6000_108));
    pub const USEGETN: FrameId = FrameId::Generic(Generic(0x6000_109));
    pub const BLANK_CURSOR: FrameId = FrameId::Generic(Generic(0x6000_10a));
    pub const STDARROW: FrameId = FrameId::Generic(Generic(0x6000_10b));
    pub const SUPARROW: FrameId = FrameId::Generic(Generic(0x6000_10c));
    pub const SDNARROW: FrameId = FrameId::Generic(Generic(0x6000_10d));
    pub const SCRNWEST: FrameId = FrameId::Generic(Generic(0x6000_10e));
    pub const SCRNORTH: FrameId = FrameId::Generic(Generic(0x6000_10f));
    pub const SCRNEAST: FrameId = FrameId::Generic(Generic(0x6000_110));
    pub const SCREAST: FrameId = FrameId::Generic(Generic(0x6000_111));
    pub const SCRSEAST: FrameId = FrameId::Generic(Generic(0x6000_112));
    pub const SCRSOUTH: FrameId = FrameId::Generic(Generic(0x6000_113));
    pub const SCRSWEST: FrameId = FrameId::Generic(Generic(0x6000_114));
    pub const SCRWEST: FrameId = FrameId::Generic(Generic(0x6000_115));
    pub const WAIT: FrameId = FrameId::Generic(Generic(0x6000_116));
    pub const CROSSHAIR_ATTACK: FrameId = FrameId::Generic(Generic(0x6000_117));
    pub const PLUS: FrameId = FrameId::Generic(Generic(0x6000_118));
    pub const DESTROY: FrameId = FrameId::Generic(Generic(0x6000_119));
    pub const ACTPICK: FrameId = FrameId::Generic(Generic(0x6000_11a));
    pub const ACTMENU: FrameId = FrameId::Generic(Generic(0x6000_11b));
    pub const ACTTOHIT: FrameId = FrameId::Generic(Generic(0x6000_11c));
    pub const ACTARROM: FrameId = FrameId::Generic(Generic(0x6000_11d));
    pub const HAND: FrameId = FrameId::Generic(Generic(0x6000_11e));
    pub const UNARMED: FrameId = FrameId::Generic(Generic(0x6000_11f));
    pub const BULLSEYE: FrameId = FrameId::Generic(Generic(0x6000_120));
    pub const MVEPNT: FrameId = FrameId::Generic(Generic(0x6000_121));
    pub const MVENUM: FrameId = FrameId::Generic(Generic(0x6000_122));
    pub const RELOAD: FrameId = FrameId::Generic(Generic(0x6000_123));
    pub const USET: FrameId = FrameId::Generic(Generic(0x6000_124));
    pub const CROSSHAIR_USE: FrameId = FrameId::Generic(Generic(0x6000_125));
    pub const USEON: FrameId = FrameId::Generic(Generic(0x6000_126));
    pub const WAIT2: FrameId = FrameId::Generic(Generic(0x6000_127));
    pub const FALLTEXT: FrameId = FrameId::Generic(Generic(0x6000_128));
    pub const HELPSCRN: FrameId = FrameId::Generic(Generic(0x6000_129));
    pub const HELPMAC: FrameId = FrameId::Generic(Generic(0x6000_12a));
    pub const MENUUP: FrameId = FrameId::Generic(Generic(0x6000_12b));
    pub const MENUDOWN: FrameId = FrameId::Generic(Generic(0x6000_12c));
    pub const UNLOADH: FrameId = FrameId::Generic(Generic(0x6000_12d));
    pub const UNLOADN: FrameId = FrameId::Generic(Generic(0x6000_12e));
    pub const SKILLH: FrameId = FrameId::Generic(Generic(0x6000_12f));
    pub const SKILLN: FrameId = FrameId::Generic(Generic(0x6000_130));
    pub const INVENTORY_MOVE_MULTIPLE_WINDOW: FrameId = FrameId::Generic(Generic(0x6000_131));
    pub const TIMER: FrameId = FrameId::Generic(Generic(0x6000_132));
    pub const BUTTON_ALL_UP: FrameId = FrameId::Generic(Generic(0x6000_133));
    pub const BUTTON_ALL_DOWN: FrameId = FrameId::Generic(Generic(0x6000_134));
    pub const DEATH: FrameId = FrameId::Generic(Generic(0x6000_135));
    pub const WATCH: FrameId = FrameId::Generic(Generic(0x6000_136));
    pub const SEQ2AD: FrameId = FrameId::Generic(Generic(0x6000_137));
    pub const SEQ2B: FrameId = FrameId::Generic(Generic(0x6000_138));
    pub const SEQ3A: FrameId = FrameId::Generic(Generic(0x6000_139));
    pub const SEQ3C: FrameId = FrameId::Generic(Generic(0x6000_13a));
    pub const SEQ5A: FrameId = FrameId::Generic(Generic(0x6000_13b));
    pub const SEQ5B: FrameId = FrameId::Generic(Generic(0x6000_13c));
    pub const SEQ5D: FrameId = FrameId::Generic(Generic(0x6000_13d));
    pub const SEQ6A: FrameId = FrameId::Generic(Generic(0x6000_13e));
    pub const SEQ6B: FrameId = FrameId::Generic(Generic(0x6000_13f));
    pub const SEQ8ABC: FrameId = FrameId::Generic(Generic(0x6000_140));
    pub const SHABAD3: FrameId = FrameId::Generic(Generic(0x6000_141));
    pub const SHAGOD2: FrameId = FrameId::Generic(Generic(0x6000_142));
    pub const SEQ4ABC: FrameId = FrameId::Generic(Generic(0x6000_143));
    pub const SEQ4DE: FrameId = FrameId::Generic(Generic(0x6000_144));
    pub const SEQ7A: FrameId = FrameId::Generic(Generic(0x6000_145));
    pub const SEQ7C: FrameId = FrameId::Generic(Generic(0x6000_146));
    pub const DP: FrameId = FrameId::Generic(Generic(0x6000_147));
    pub const SCREX: FrameId = FrameId::Generic(Generic(0x6000_148));
    pub const SCRNEX: FrameId = FrameId::Generic(Generic(0x6000_149));
    pub const SCRNWX: FrameId = FrameId::Generic(Generic(0x6000_14a));
    pub const SCRNX: FrameId = FrameId::Generic(Generic(0x6000_14b));
    pub const SCRSEX: FrameId = FrameId::Generic(Generic(0x6000_14c));
    pub const SCRSWX: FrameId = FrameId::Generic(Generic(0x6000_14d));
    pub const SCRSX: FrameId = FrameId::Generic(Generic(0x6000_14e));
    pub const SCRWX: FrameId = FrameId::Generic(Generic(0x6000_14f));
    pub const WRLDSPR0: FrameId = FrameId::Generic(Generic(0x6000_150));
    pub const WRLDSPR1: FrameId = FrameId::Generic(Generic(0x6000_151));
    pub const WRLDSPR2: FrameId = FrameId::Generic(Generic(0x6000_152));
    pub const WRLDMP00: FrameId = FrameId::Generic(Generic(0x6000_153));
    pub const WRLDMP01: FrameId = FrameId::Generic(Generic(0x6000_154));
    pub const WRLDMP02: FrameId = FrameId::Generic(Generic(0x6000_155));
    pub const WRLDMP03: FrameId = FrameId::Generic(Generic(0x6000_156));
    pub const WRLDMP04: FrameId = FrameId::Generic(Generic(0x6000_157));
    pub const WRLDMP05: FrameId = FrameId::Generic(Generic(0x6000_158));
    pub const WRLDMP06: FrameId = FrameId::Generic(Generic(0x6000_159));
    pub const WRLDMP07: FrameId = FrameId::Generic(Generic(0x6000_15a));
    pub const WRLDMP08: FrameId = FrameId::Generic(Generic(0x6000_15b));
    pub const WRLDMP09: FrameId = FrameId::Generic(Generic(0x6000_15c));
    pub const WRLDMP10: FrameId = FrameId::Generic(Generic(0x6000_15d));
    pub const WRLDMP11: FrameId = FrameId::Generic(Generic(0x6000_15e));
    pub const WRLDMP12: FrameId = FrameId::Generic(Generic(0x6000_15f));
    pub const WRLDMP13: FrameId = FrameId::Generic(Generic(0x6000_160));
    pub const WRLDMP14: FrameId = FrameId::Generic(Generic(0x6000_161));
    pub const WRLDMP15: FrameId = FrameId::Generic(Generic(0x6000_162));
    pub const WRLDMP16: FrameId = FrameId::Generic(Generic(0x6000_163));
    pub const WRLDMP17: FrameId = FrameId::Generic(Generic(0x6000_164));
    pub const WRLDMP18: FrameId = FrameId::Generic(Generic(0x6000_165));
    pub const WRLDMP19: FrameId = FrameId::Generic(Generic(0x6000_166));
    pub const WMINFCE0: FrameId = FrameId::Generic(Generic(0x6000_167));
    pub const WMINFCE1: FrameId = FrameId::Generic(Generic(0x6000_168));
    pub const WMINFCE2: FrameId = FrameId::Generic(Generic(0x6000_169));
    pub const WMINFCE3: FrameId = FrameId::Generic(Generic(0x6000_16a));
    pub const WMSCREEN: FrameId = FrameId::Generic(Generic(0x6000_16b));
    pub const WMTABS: FrameId = FrameId::Generic(Generic(0x6000_16c));
    pub const WMDIAL: FrameId = FrameId::Generic(Generic(0x6000_16d));
    pub const WMGLOBE: FrameId = FrameId::Generic(Generic(0x6000_16e));
    pub const WMTBEDGE: FrameId = FrameId::Generic(Generic(0x6000_16f));
    pub const WM_ABBEY: FrameId = FrameId::Generic(Generic(0x6000_170));
    pub const WM_ARMY: FrameId = FrameId::Generic(Generic(0x6000_171));
    pub const WM_ARROY: FrameId = FrameId::Generic(Generic(0x6000_172));
    pub const WM_NAVAR: FrameId = FrameId::Generic(Generic(0x6000_173));
    pub const WM_DEN: FrameId = FrameId::Generic(Generic(0x6000_174));
    pub const WM_ENCLV: FrameId = FrameId::Generic(Generic(0x6000_175));
    pub const WM_EPA: FrameId = FrameId::Generic(Generic(0x6000_176));
    pub const WM_GECKO: FrameId = FrameId::Generic(Generic(0x6000_177));
    pub const WM_HILLS: FrameId = FrameId::Generic(Generic(0x6000_178));
    pub const WM_KLAM: FrameId = FrameId::Generic(Generic(0x6000_179));
    pub const WM_MLTRY: FrameId = FrameId::Generic(Generic(0x6000_17a));
    pub const WM_MODOC: FrameId = FrameId::Generic(Generic(0x6000_17b));
    pub const WM_NCR: FrameId = FrameId::Generic(Generic(0x6000_17c));
    pub const WM_RED: FrameId = FrameId::Generic(Generic(0x6000_17d));
    pub const WM_RENO: FrameId = FrameId::Generic(Generic(0x6000_17e));
    pub const WM_SANFN: FrameId = FrameId::Generic(Generic(0x6000_17f));
    pub const WM_TRIBE: FrameId = FrameId::Generic(Generic(0x6000_180));
    pub const WM_VLT13: FrameId = FrameId::Generic(Generic(0x6000_181));
    pub const WM_VLT15: FrameId = FrameId::Generic(Generic(0x6000_182));
    pub const WM_VLTCT: FrameId = FrameId::Generic(Generic(0x6000_183));
    pub const EL_BASE1: FrameId = FrameId::Generic(Generic(0x6000_184));
    pub const DI_TALKP: FrameId = FrameId::Generic(Generic(0x6000_185));
    pub const CONTROL: FrameId = FrameId::Generic(Generic(0x6000_186));
    pub const CUSTOM: FrameId = FrameId::Generic(Generic(0x6000_187));
    pub const AGGDN: FrameId = FrameId::Generic(Generic(0x6000_188));
    pub const AGGOFF: FrameId = FrameId::Generic(Generic(0x6000_189));
    pub const AGGUP: FrameId = FrameId::Generic(Generic(0x6000_18a));
    pub const BERDN: FrameId = FrameId::Generic(Generic(0x6000_18b));
    pub const BEROFF: FrameId = FrameId::Generic(Generic(0x6000_18c));
    pub const BERUP: FrameId = FrameId::Generic(Generic(0x6000_18d));
    pub const COWDN: FrameId = FrameId::Generic(Generic(0x6000_18e));
    pub const COWOFF: FrameId = FrameId::Generic(Generic(0x6000_18f));
    pub const COWUP: FrameId = FrameId::Generic(Generic(0x6000_190));
    pub const CUSDN: FrameId = FrameId::Generic(Generic(0x6000_191));
    pub const CUSOFF: FrameId = FrameId::Generic(Generic(0x6000_192));
    pub const CUSUP: FrameId = FrameId::Generic(Generic(0x6000_193));
    pub const DEFDN: FrameId = FrameId::Generic(Generic(0x6000_194));
    pub const DEFOFF: FrameId = FrameId::Generic(Generic(0x6000_195));
    pub const DEFUP: FrameId = FrameId::Generic(Generic(0x6000_196));
    pub const ATTACKDN: FrameId = FrameId::Generic(Generic(0x6000_197));
    pub const ATTACKUP: FrameId = FrameId::Generic(Generic(0x6000_198));
    pub const BURSTDN: FrameId = FrameId::Generic(Generic(0x6000_199));
    pub const BURSTUP: FrameId = FrameId::Generic(Generic(0x6000_19a));
    pub const CHEMDN: FrameId = FrameId::Generic(Generic(0x6000_19b));
    pub const CHEMUP: FrameId = FrameId::Generic(Generic(0x6000_19c));
    pub const DISTDN: FrameId = FrameId::Generic(Generic(0x6000_19d));
    pub const DISTUP: FrameId = FrameId::Generic(Generic(0x6000_19e));
    pub const RUNDN: FrameId = FrameId::Generic(Generic(0x6000_19f));
    pub const RUNUP: FrameId = FrameId::Generic(Generic(0x6000_1a0));
    pub const WEAPDN: FrameId = FrameId::Generic(Generic(0x6000_1a1));
    pub const WEAPUP: FrameId = FrameId::Generic(Generic(0x6000_1a2));
    pub const CUSSEL: FrameId = FrameId::Generic(Generic(0x6000_1a3));
    pub const TRADE: FrameId = FrameId::Generic(Generic(0x6000_1a4));
    pub const CM_JAB: FrameId = FrameId::Generic(Generic(0x6000_1a5));
    pub const CM_PRCKK: FrameId = FrameId::Generic(Generic(0x6000_1a6));
    pub const CM_PLMST: FrameId = FrameId::Generic(Generic(0x6000_1a7));
    pub const CM_PSTRK: FrameId = FrameId::Generic(Generic(0x6000_1a8));
    pub const HAMPNCH: FrameId = FrameId::Generic(Generic(0x6000_1a9));
    pub const HIPK: FrameId = FrameId::Generic(Generic(0x6000_1aa));
    pub const CM_HOOKK: FrameId = FrameId::Generic(Generic(0x6000_1ab));
    pub const CM_HYMKR: FrameId = FrameId::Generic(Generic(0x6000_1ac));
    pub const CM_PWKCK: FrameId = FrameId::Generic(Generic(0x6000_1ad));
    pub const SKICK: FrameId = FrameId::Generic(Generic(0x6000_1ae));
    pub const SNAPKICK: FrameId = FrameId::Generic(Generic(0x6000_1af));
    pub const SPUNCH: FrameId = FrameId::Generic(Generic(0x6000_1b0));
    pub const WMCARMVE: FrameId = FrameId::Generic(Generic(0x6000_1b1));
    pub const PUSHH: FrameId = FrameId::Generic(Generic(0x6000_1b2));
    pub const PUSHN: FrameId = FrameId::Generic(Generic(0x6000_1b3));
    pub const INVMAUP: FrameId = FrameId::Generic(Generic(0x6000_1b4));
    pub const INVMADN: FrameId = FrameId::Generic(Generic(0x6000_1b5));
    pub const WMRNDEN2: FrameId = FrameId::Generic(Generic(0x6000_1b6));
    pub const WMRNDEN3: FrameId = FrameId::Generic(Generic(0x6000_1b7));
    pub const EG_ARO01: FrameId = FrameId::Generic(Generic(0x6000_1b8));
    pub const EG_BRO01: FrameId = FrameId::Generic(Generic(0x6000_1b9));
    pub const EG_DEN01: FrameId = FrameId::Generic(Generic(0x6000_1ba));
    pub const EG_GCK01: FrameId = FrameId::Generic(Generic(0x6000_1bb));
    pub const EG_GCK02: FrameId = FrameId::Generic(Generic(0x6000_1bc));
    pub const EG_MOD01: FrameId = FrameId::Generic(Generic(0x6000_1bd));
    pub const EG_MOD02: FrameId = FrameId::Generic(Generic(0x6000_1be));
    pub const EG_SAN01: FrameId = FrameId::Generic(Generic(0x6000_1bf));
    pub const EG_SAN02: FrameId = FrameId::Generic(Generic(0x6000_1c0));
    pub const EG_V1501: FrameId = FrameId::Generic(Generic(0x6000_1c1));
    pub const TWNNAVRO: FrameId = FrameId::Generic(Generic(0x6000_1c2));
    pub const TWNMAP04: FrameId = FrameId::Generic(Generic(0x6000_1c3));
    pub const TWNREDNG: FrameId = FrameId::Generic(Generic(0x6000_1c4));
    pub const TWNVCITY: FrameId = FrameId::Generic(Generic(0x6000_1c5));
    pub const EG_DEN02: FrameId = FrameId::Generic(Generic(0x6000_1c6));
    pub const EG_NCR01: FrameId = FrameId::Generic(Generic(0x6000_1c7));
    pub const EG_NCR02: FrameId = FrameId::Generic(Generic(0x6000_1c8));
    pub const EG_RED01: FrameId = FrameId::Generic(Generic(0x6000_1c9));
    pub const EG_RED02: FrameId = FrameId::Generic(Generic(0x6000_1ca));
    pub const EG_REN01: FrameId = FrameId::Generic(Generic(0x6000_1cb));
    pub const EG_REN02: FrameId = FrameId::Generic(Generic(0x6000_1cc));
    pub const EG_SAN03: FrameId = FrameId::Generic(Generic(0x6000_1cd));
    pub const EG_V1301: FrameId = FrameId::Generic(Generic(0x6000_1ce));
    pub const EG_V1502: FrameId = FrameId::Generic(Generic(0x6000_1cf));
    pub const EG_VCT01: FrameId = FrameId::Generic(Generic(0x6000_1d0));
    pub const EG_VCT02: FrameId = FrameId::Generic(Generic(0x6000_1d1));
    pub const EG_SAN04: FrameId = FrameId::Generic(Generic(0x6000_1d2));
    pub const EG_MYR01: FrameId = FrameId::Generic(Generic(0x6000_1d3));
    pub const TWNMILBS: FrameId = FrameId::Generic(Generic(0x6000_1d4));
}
