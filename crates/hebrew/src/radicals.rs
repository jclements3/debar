/// The 214 Kangxi radicals with their data.
pub struct RadicalInfo {
    pub number: u16,
    pub radical: char,
    pub strokes: u8,
    pub meaning: &'static str,
    pub pinyin: &'static str,
}

pub const RADICALS: &[RadicalInfo] = &[
    // 1 stroke
    RadicalInfo { number: 1,   radical: '\u{4E00}', strokes: 1, meaning: "one",              pinyin: "y\u{012B}" },
    RadicalInfo { number: 2,   radical: '\u{4E28}', strokes: 1, meaning: "line",             pinyin: "g\u{01D4}n" },
    RadicalInfo { number: 3,   radical: '\u{4E36}', strokes: 1, meaning: "dot",              pinyin: "zh\u{01D4}" },
    RadicalInfo { number: 4,   radical: '\u{4E3F}', strokes: 1, meaning: "slash",            pinyin: "pi\u{011B}" },
    RadicalInfo { number: 5,   radical: '\u{4E59}', strokes: 1, meaning: "second",           pinyin: "y\u{01D0}" },
    RadicalInfo { number: 6,   radical: '\u{4E85}', strokes: 1, meaning: "hook",             pinyin: "ju\u{00E9}" },
    // 2 strokes
    RadicalInfo { number: 7,   radical: '\u{4E8C}', strokes: 2, meaning: "two",              pinyin: "\u{00E8}r" },
    RadicalInfo { number: 8,   radical: '\u{4EA0}', strokes: 2, meaning: "lid",              pinyin: "t\u{00F3}u" },
    RadicalInfo { number: 9,   radical: '\u{4EBA}', strokes: 2, meaning: "person",           pinyin: "r\u{00E9}n" },
    RadicalInfo { number: 10,  radical: '\u{513F}', strokes: 2, meaning: "legs",             pinyin: "\u{00E9}r" },
    RadicalInfo { number: 11,  radical: '\u{5165}', strokes: 2, meaning: "enter",            pinyin: "r\u{00F9}" },
    RadicalInfo { number: 12,  radical: '\u{516B}', strokes: 2, meaning: "eight",            pinyin: "b\u{0101}" },
    RadicalInfo { number: 13,  radical: '\u{5182}', strokes: 2, meaning: "down box",         pinyin: "ji\u{014D}ng" },
    RadicalInfo { number: 14,  radical: '\u{5196}', strokes: 2, meaning: "cover",            pinyin: "m\u{00EC}" },
    RadicalInfo { number: 15,  radical: '\u{51AB}', strokes: 2, meaning: "ice",              pinyin: "b\u{012B}ng" },
    RadicalInfo { number: 16,  radical: '\u{51E0}', strokes: 2, meaning: "table",            pinyin: "j\u{012B}" },
    RadicalInfo { number: 17,  radical: '\u{51F5}', strokes: 2, meaning: "open box",         pinyin: "q\u{01D4}" },
    RadicalInfo { number: 18,  radical: '\u{5200}', strokes: 2, meaning: "knife",            pinyin: "d\u{0101}o" },
    RadicalInfo { number: 19,  radical: '\u{529B}', strokes: 2, meaning: "power",            pinyin: "l\u{00EC}" },
    RadicalInfo { number: 20,  radical: '\u{52F9}', strokes: 2, meaning: "wrap",             pinyin: "b\u{0101}o" },
    RadicalInfo { number: 21,  radical: '\u{5315}', strokes: 2, meaning: "spoon",            pinyin: "b\u{01D0}" },
    RadicalInfo { number: 22,  radical: '\u{531A}', strokes: 2, meaning: "right open box",   pinyin: "f\u{0101}ng" },
    RadicalInfo { number: 23,  radical: '\u{5338}', strokes: 2, meaning: "hiding enclosure", pinyin: "x\u{01D0}" },
    RadicalInfo { number: 24,  radical: '\u{5341}', strokes: 2, meaning: "ten",              pinyin: "sh\u{00ED}" },
    RadicalInfo { number: 25,  radical: '\u{535C}', strokes: 2, meaning: "divination",       pinyin: "b\u{01D4}" },
    RadicalInfo { number: 26,  radical: '\u{5369}', strokes: 2, meaning: "seal",             pinyin: "ji\u{00E9}" },
    RadicalInfo { number: 27,  radical: '\u{5382}', strokes: 2, meaning: "cliff",            pinyin: "h\u{01CE}n" },
    RadicalInfo { number: 28,  radical: '\u{53B6}', strokes: 2, meaning: "private",          pinyin: "s\u{012B}" },
    RadicalInfo { number: 29,  radical: '\u{53C8}', strokes: 2, meaning: "again",            pinyin: "y\u{00F2}u" },
    // 3 strokes
    RadicalInfo { number: 30,  radical: '\u{53E3}', strokes: 3, meaning: "mouth",            pinyin: "k\u{01D2}u" },
    RadicalInfo { number: 31,  radical: '\u{56D7}', strokes: 3, meaning: "enclosure",        pinyin: "w\u{00E9}i" },
    RadicalInfo { number: 32,  radical: '\u{571F}', strokes: 3, meaning: "earth",            pinyin: "t\u{01D4}" },
    RadicalInfo { number: 33,  radical: '\u{58EB}', strokes: 3, meaning: "scholar",          pinyin: "sh\u{00EC}" },
    RadicalInfo { number: 34,  radical: '\u{5902}', strokes: 3, meaning: "go",               pinyin: "zh\u{01D0}" },
    RadicalInfo { number: 35,  radical: '\u{590A}', strokes: 3, meaning: "go slowly",        pinyin: "su\u{012B}" },
    RadicalInfo { number: 36,  radical: '\u{5915}', strokes: 3, meaning: "evening",          pinyin: "x\u{012B}" },
    RadicalInfo { number: 37,  radical: '\u{5927}', strokes: 3, meaning: "big",              pinyin: "d\u{00E0}" },
    RadicalInfo { number: 38,  radical: '\u{5973}', strokes: 3, meaning: "woman",            pinyin: "n\u{01DA}" },
    RadicalInfo { number: 39,  radical: '\u{5B50}', strokes: 3, meaning: "child",            pinyin: "z\u{01D0}" },
    RadicalInfo { number: 40,  radical: '\u{5B80}', strokes: 3, meaning: "roof",             pinyin: "mi\u{00E1}n" },
    RadicalInfo { number: 41,  radical: '\u{5BF8}', strokes: 3, meaning: "inch",             pinyin: "c\u{00F9}n" },
    RadicalInfo { number: 42,  radical: '\u{5C0F}', strokes: 3, meaning: "small",            pinyin: "xi\u{01CE}o" },
    RadicalInfo { number: 43,  radical: '\u{5C22}', strokes: 3, meaning: "lame",             pinyin: "w\u{0101}ng" },
    RadicalInfo { number: 44,  radical: '\u{5C38}', strokes: 3, meaning: "corpse",           pinyin: "sh\u{012B}" },
    RadicalInfo { number: 45,  radical: '\u{5C6E}', strokes: 3, meaning: "sprout",           pinyin: "ch\u{00E8}" },
    RadicalInfo { number: 46,  radical: '\u{5C71}', strokes: 3, meaning: "mountain",         pinyin: "sh\u{0101}n" },
    RadicalInfo { number: 47,  radical: '\u{5DDB}', strokes: 3, meaning: "river",            pinyin: "chu\u{0101}n" },
    RadicalInfo { number: 48,  radical: '\u{5DE5}', strokes: 3, meaning: "work",             pinyin: "g\u{014D}ng" },
    RadicalInfo { number: 49,  radical: '\u{5DF1}', strokes: 3, meaning: "oneself",          pinyin: "j\u{01D0}" },
    RadicalInfo { number: 50,  radical: '\u{5DFE}', strokes: 3, meaning: "turban",           pinyin: "j\u{012B}n" },
    RadicalInfo { number: 51,  radical: '\u{5E72}', strokes: 3, meaning: "dry",              pinyin: "g\u{0101}n" },
    RadicalInfo { number: 52,  radical: '\u{5E7A}', strokes: 3, meaning: "short thread",     pinyin: "y\u{0101}o" },
    RadicalInfo { number: 53,  radical: '\u{5E7F}', strokes: 3, meaning: "dotted cliff",     pinyin: "gu\u{01CE}ng" },
    RadicalInfo { number: 54,  radical: '\u{5EF4}', strokes: 3, meaning: "long stride",      pinyin: "y\u{01D0}n" },
    RadicalInfo { number: 55,  radical: '\u{5EFE}', strokes: 3, meaning: "two hands",        pinyin: "g\u{01D2}ng" },
    RadicalInfo { number: 56,  radical: '\u{5F0B}', strokes: 3, meaning: "shoot",            pinyin: "y\u{00EC}" },
    RadicalInfo { number: 57,  radical: '\u{5F13}', strokes: 3, meaning: "bow",              pinyin: "g\u{014D}ng" },
    RadicalInfo { number: 58,  radical: '\u{5F50}', strokes: 3, meaning: "snout",            pinyin: "j\u{00EC}" },
    RadicalInfo { number: 59,  radical: '\u{5F61}', strokes: 3, meaning: "bristle",          pinyin: "sh\u{0101}n" },
    RadicalInfo { number: 60,  radical: '\u{5F73}', strokes: 3, meaning: "step",             pinyin: "ch\u{00EC}" },
    // 4 strokes
    RadicalInfo { number: 61,  radical: '\u{5FC3}', strokes: 4, meaning: "heart",            pinyin: "x\u{012B}n" },
    RadicalInfo { number: 62,  radical: '\u{6208}', strokes: 4, meaning: "halberd",          pinyin: "g\u{0113}" },
    RadicalInfo { number: 63,  radical: '\u{6236}', strokes: 4, meaning: "door",             pinyin: "h\u{00F9}" },
    RadicalInfo { number: 64,  radical: '\u{624B}', strokes: 4, meaning: "hand",             pinyin: "sh\u{01D2}u" },
    RadicalInfo { number: 65,  radical: '\u{652F}', strokes: 4, meaning: "branch",           pinyin: "zh\u{012B}" },
    RadicalInfo { number: 66,  radical: '\u{6534}', strokes: 4, meaning: "rap",              pinyin: "p\u{016B}" },
    RadicalInfo { number: 67,  radical: '\u{6587}', strokes: 4, meaning: "script",           pinyin: "w\u{00E9}n" },
    RadicalInfo { number: 68,  radical: '\u{6597}', strokes: 4, meaning: "dipper",           pinyin: "d\u{01D2}u" },
    RadicalInfo { number: 69,  radical: '\u{65A4}', strokes: 4, meaning: "axe",              pinyin: "j\u{012B}n" },
    RadicalInfo { number: 70,  radical: '\u{65B9}', strokes: 4, meaning: "square",           pinyin: "f\u{0101}ng" },
    RadicalInfo { number: 71,  radical: '\u{65E0}', strokes: 4, meaning: "not",              pinyin: "w\u{00FA}" },
    RadicalInfo { number: 72,  radical: '\u{65E5}', strokes: 4, meaning: "sun",              pinyin: "r\u{00EC}" },
    RadicalInfo { number: 73,  radical: '\u{66F0}', strokes: 4, meaning: "say",              pinyin: "yu\u{0113}" },
    RadicalInfo { number: 74,  radical: '\u{6708}', strokes: 4, meaning: "moon",             pinyin: "yu\u{00E8}" },
    RadicalInfo { number: 75,  radical: '\u{6728}', strokes: 4, meaning: "tree",             pinyin: "m\u{00F9}" },
    RadicalInfo { number: 76,  radical: '\u{6B20}', strokes: 4, meaning: "lack",             pinyin: "qi\u{00E0}n" },
    RadicalInfo { number: 77,  radical: '\u{6B62}', strokes: 4, meaning: "stop",             pinyin: "zh\u{01D0}" },
    RadicalInfo { number: 78,  radical: '\u{6B79}', strokes: 4, meaning: "death",            pinyin: "d\u{00E0}i" },
    RadicalInfo { number: 79,  radical: '\u{6BB3}', strokes: 4, meaning: "weapon",           pinyin: "sh\u{016B}" },
    RadicalInfo { number: 80,  radical: '\u{6BCB}', strokes: 4, meaning: "do not",           pinyin: "w\u{00FA}" },
    RadicalInfo { number: 81,  radical: '\u{6BD4}', strokes: 4, meaning: "compare",          pinyin: "b\u{01D0}" },
    RadicalInfo { number: 82,  radical: '\u{6BDB}', strokes: 4, meaning: "fur",              pinyin: "m\u{00E1}o" },
    RadicalInfo { number: 83,  radical: '\u{6C0F}', strokes: 4, meaning: "clan",             pinyin: "sh\u{00EC}" },
    RadicalInfo { number: 84,  radical: '\u{6C14}', strokes: 4, meaning: "steam",            pinyin: "q\u{00EC}" },
    RadicalInfo { number: 85,  radical: '\u{6C34}', strokes: 4, meaning: "water",            pinyin: "shu\u{01D0}" },
    RadicalInfo { number: 86,  radical: '\u{706B}', strokes: 4, meaning: "fire",             pinyin: "hu\u{01D2}" },
    RadicalInfo { number: 87,  radical: '\u{722A}', strokes: 4, meaning: "claw",             pinyin: "zh\u{01CE}o" },
    RadicalInfo { number: 88,  radical: '\u{7236}', strokes: 4, meaning: "father",           pinyin: "f\u{00F9}" },
    RadicalInfo { number: 89,  radical: '\u{723B}', strokes: 4, meaning: "double x",         pinyin: "y\u{00E1}o" },
    RadicalInfo { number: 90,  radical: '\u{723F}', strokes: 4, meaning: "half tree trunk",  pinyin: "qi\u{00E1}ng" },
    RadicalInfo { number: 91,  radical: '\u{7247}', strokes: 4, meaning: "slice",            pinyin: "pi\u{00E0}n" },
    RadicalInfo { number: 92,  radical: '\u{7259}', strokes: 4, meaning: "fang",             pinyin: "y\u{00E1}" },
    RadicalInfo { number: 93,  radical: '\u{725B}', strokes: 4, meaning: "cow",              pinyin: "ni\u{00FA}" },
    RadicalInfo { number: 94,  radical: '\u{72AC}', strokes: 4, meaning: "dog",              pinyin: "qu\u{01CE}n" },
    // 5 strokes
    RadicalInfo { number: 95,  radical: '\u{7384}', strokes: 5, meaning: "profound",         pinyin: "xu\u{00E1}n" },
    RadicalInfo { number: 96,  radical: '\u{7389}', strokes: 5, meaning: "jade",             pinyin: "y\u{00F9}" },
    RadicalInfo { number: 97,  radical: '\u{74DC}', strokes: 5, meaning: "melon",            pinyin: "gu\u{0101}" },
    RadicalInfo { number: 98,  radical: '\u{74E6}', strokes: 5, meaning: "tile",             pinyin: "w\u{01CE}" },
    RadicalInfo { number: 99,  radical: '\u{7518}', strokes: 5, meaning: "sweet",            pinyin: "g\u{0101}n" },
    RadicalInfo { number: 100, radical: '\u{751F}', strokes: 5, meaning: "life",             pinyin: "sh\u{0113}ng" },
    RadicalInfo { number: 101, radical: '\u{7528}', strokes: 5, meaning: "use",              pinyin: "y\u{00F2}ng" },
    RadicalInfo { number: 102, radical: '\u{7530}', strokes: 5, meaning: "field",            pinyin: "ti\u{00E1}n" },
    RadicalInfo { number: 103, radical: '\u{758B}', strokes: 5, meaning: "bolt of cloth",    pinyin: "p\u{01D0}" },
    RadicalInfo { number: 104, radical: '\u{7592}', strokes: 5, meaning: "sickness",         pinyin: "n\u{00E8}" },
    RadicalInfo { number: 105, radical: '\u{7676}', strokes: 5, meaning: "dotted tent",      pinyin: "b\u{00F2}" },
    RadicalInfo { number: 106, radical: '\u{767D}', strokes: 5, meaning: "white",            pinyin: "b\u{00E1}i" },
    RadicalInfo { number: 107, radical: '\u{76AE}', strokes: 5, meaning: "skin",             pinyin: "p\u{00ED}" },
    RadicalInfo { number: 108, radical: '\u{76BF}', strokes: 5, meaning: "dish",             pinyin: "m\u{01D0}n" },
    RadicalInfo { number: 109, radical: '\u{76EE}', strokes: 5, meaning: "eye",              pinyin: "m\u{00F9}" },
    RadicalInfo { number: 110, radical: '\u{77DB}', strokes: 5, meaning: "spear",            pinyin: "m\u{00E1}o" },
    RadicalInfo { number: 111, radical: '\u{77E2}', strokes: 5, meaning: "arrow",            pinyin: "sh\u{01D0}" },
    RadicalInfo { number: 112, radical: '\u{77F3}', strokes: 5, meaning: "stone",            pinyin: "sh\u{00ED}" },
    RadicalInfo { number: 113, radical: '\u{793A}', strokes: 5, meaning: "spirit",           pinyin: "sh\u{00EC}" },
    RadicalInfo { number: 114, radical: '\u{79B8}', strokes: 5, meaning: "track",            pinyin: "r\u{00F3}u" },
    RadicalInfo { number: 115, radical: '\u{79BE}', strokes: 5, meaning: "grain",            pinyin: "h\u{00E9}" },
    RadicalInfo { number: 116, radical: '\u{7A74}', strokes: 5, meaning: "cave",             pinyin: "xu\u{00E9}" },
    RadicalInfo { number: 117, radical: '\u{7ACB}', strokes: 5, meaning: "stand",            pinyin: "l\u{00EC}" },
    // 6 strokes
    RadicalInfo { number: 118, radical: '\u{7AF9}', strokes: 6, meaning: "bamboo",           pinyin: "zh\u{00FA}" },
    RadicalInfo { number: 119, radical: '\u{7C73}', strokes: 6, meaning: "rice",             pinyin: "m\u{01D0}" },
    RadicalInfo { number: 120, radical: '\u{7CF8}', strokes: 6, meaning: "silk",             pinyin: "m\u{00EC}" },
    RadicalInfo { number: 121, radical: '\u{7F36}', strokes: 6, meaning: "jar",              pinyin: "f\u{01D2}u" },
    RadicalInfo { number: 122, radical: '\u{7F51}', strokes: 6, meaning: "net",              pinyin: "w\u{01CE}ng" },
    RadicalInfo { number: 123, radical: '\u{7F8A}', strokes: 6, meaning: "sheep",            pinyin: "y\u{00E1}ng" },
    RadicalInfo { number: 124, radical: '\u{7FBD}', strokes: 6, meaning: "feather",          pinyin: "y\u{01D4}" },
    RadicalInfo { number: 125, radical: '\u{8001}', strokes: 6, meaning: "old",              pinyin: "l\u{01CE}o" },
    RadicalInfo { number: 126, radical: '\u{800C}', strokes: 6, meaning: "and",              pinyin: "\u{00E9}r" },
    RadicalInfo { number: 127, radical: '\u{8012}', strokes: 6, meaning: "plow",             pinyin: "l\u{011B}i" },
    RadicalInfo { number: 128, radical: '\u{8033}', strokes: 6, meaning: "ear",              pinyin: "\u{011B}r" },
    RadicalInfo { number: 129, radical: '\u{807F}', strokes: 6, meaning: "brush",            pinyin: "y\u{00F9}" },
    RadicalInfo { number: 130, radical: '\u{8089}', strokes: 6, meaning: "meat",             pinyin: "r\u{00F2}u" },
    RadicalInfo { number: 131, radical: '\u{81E3}', strokes: 6, meaning: "minister",         pinyin: "ch\u{00E9}n" },
    RadicalInfo { number: 132, radical: '\u{81EA}', strokes: 6, meaning: "self",             pinyin: "z\u{00EC}" },
    RadicalInfo { number: 133, radical: '\u{81F3}', strokes: 6, meaning: "arrive",           pinyin: "zh\u{00EC}" },
    RadicalInfo { number: 134, radical: '\u{81FC}', strokes: 6, meaning: "mortar",           pinyin: "ji\u{00F9}" },
    RadicalInfo { number: 135, radical: '\u{820C}', strokes: 6, meaning: "tongue",           pinyin: "sh\u{00E9}" },
    RadicalInfo { number: 136, radical: '\u{821B}', strokes: 6, meaning: "oppose",           pinyin: "chu\u{01CE}n" },
    RadicalInfo { number: 137, radical: '\u{821F}', strokes: 6, meaning: "boat",             pinyin: "zh\u{014D}u" },
    RadicalInfo { number: 138, radical: '\u{826E}', strokes: 6, meaning: "stopping",         pinyin: "g\u{00E8}n" },
    RadicalInfo { number: 139, radical: '\u{8272}', strokes: 6, meaning: "color",            pinyin: "s\u{00E8}" },
    RadicalInfo { number: 140, radical: '\u{8278}', strokes: 6, meaning: "grass",            pinyin: "c\u{01CE}o" },
    RadicalInfo { number: 141, radical: '\u{864D}', strokes: 6, meaning: "tiger",            pinyin: "h\u{01D4}" },
    RadicalInfo { number: 142, radical: '\u{866B}', strokes: 6, meaning: "insect",           pinyin: "ch\u{00F3}ng" },
    RadicalInfo { number: 143, radical: '\u{8840}', strokes: 6, meaning: "blood",            pinyin: "xu\u{00E8}" },
    RadicalInfo { number: 144, radical: '\u{884C}', strokes: 6, meaning: "walk",             pinyin: "x\u{00ED}ng" },
    RadicalInfo { number: 145, radical: '\u{8863}', strokes: 6, meaning: "clothes",          pinyin: "y\u{012B}" },
    RadicalInfo { number: 146, radical: '\u{897E}', strokes: 6, meaning: "west",             pinyin: "x\u{012B}" },
    // 7 strokes
    RadicalInfo { number: 147, radical: '\u{898B}', strokes: 7, meaning: "see",              pinyin: "ji\u{00E0}n" },
    RadicalInfo { number: 148, radical: '\u{89D2}', strokes: 7, meaning: "horn",             pinyin: "ji\u{01CE}o" },
    RadicalInfo { number: 149, radical: '\u{8A00}', strokes: 7, meaning: "speech",           pinyin: "y\u{00E1}n" },
    RadicalInfo { number: 150, radical: '\u{8C37}', strokes: 7, meaning: "valley",           pinyin: "g\u{01D4}" },
    RadicalInfo { number: 151, radical: '\u{8C46}', strokes: 7, meaning: "bean",             pinyin: "d\u{00F2}u" },
    RadicalInfo { number: 152, radical: '\u{8C55}', strokes: 7, meaning: "pig",              pinyin: "sh\u{01D0}" },
    RadicalInfo { number: 153, radical: '\u{8C78}', strokes: 7, meaning: "badger",           pinyin: "zh\u{00EC}" },
    RadicalInfo { number: 154, radical: '\u{8C9D}', strokes: 7, meaning: "shell",            pinyin: "b\u{00E8}i" },
    RadicalInfo { number: 155, radical: '\u{8D64}', strokes: 7, meaning: "red",              pinyin: "ch\u{00EC}" },
    RadicalInfo { number: 156, radical: '\u{8D70}', strokes: 7, meaning: "run",              pinyin: "z\u{01D2}u" },
    RadicalInfo { number: 157, radical: '\u{8DB3}', strokes: 7, meaning: "foot",             pinyin: "z\u{00FA}" },
    RadicalInfo { number: 158, radical: '\u{8EAB}', strokes: 7, meaning: "body",             pinyin: "sh\u{0113}n" },
    RadicalInfo { number: 159, radical: '\u{8ECA}', strokes: 7, meaning: "cart",             pinyin: "ch\u{0113}" },
    RadicalInfo { number: 160, radical: '\u{8F9B}', strokes: 7, meaning: "bitter",           pinyin: "x\u{012B}n" },
    RadicalInfo { number: 161, radical: '\u{8FB0}', strokes: 7, meaning: "morning",          pinyin: "ch\u{00E9}n" },
    RadicalInfo { number: 162, radical: '\u{8FB5}', strokes: 7, meaning: "walk",             pinyin: "chu\u{00F2}" },
    RadicalInfo { number: 163, radical: '\u{9091}', strokes: 7, meaning: "city",             pinyin: "y\u{00EC}" },
    RadicalInfo { number: 164, radical: '\u{9149}', strokes: 7, meaning: "wine",             pinyin: "y\u{01D2}u" },
    RadicalInfo { number: 165, radical: '\u{91C6}', strokes: 7, meaning: "distinguish",      pinyin: "bi\u{00E0}n" },
    RadicalInfo { number: 166, radical: '\u{91CC}', strokes: 7, meaning: "village",          pinyin: "l\u{01D0}" },
    // 8 strokes
    RadicalInfo { number: 167, radical: '\u{91D1}', strokes: 8, meaning: "gold",             pinyin: "j\u{012B}n" },
    RadicalInfo { number: 168, radical: '\u{9577}', strokes: 8, meaning: "long",             pinyin: "ch\u{00E1}ng" },
    RadicalInfo { number: 169, radical: '\u{9580}', strokes: 8, meaning: "gate",             pinyin: "m\u{00E9}n" },
    RadicalInfo { number: 170, radical: '\u{961C}', strokes: 8, meaning: "mound",            pinyin: "f\u{00F9}" },
    RadicalInfo { number: 171, radical: '\u{96B6}', strokes: 8, meaning: "slave",            pinyin: "l\u{00EC}" },
    RadicalInfo { number: 172, radical: '\u{96B9}', strokes: 8, meaning: "short-tailed bird", pinyin: "zhu\u{012B}" },
    RadicalInfo { number: 173, radical: '\u{96E8}', strokes: 8, meaning: "rain",             pinyin: "y\u{01D4}" },
    RadicalInfo { number: 174, radical: '\u{9751}', strokes: 8, meaning: "blue",             pinyin: "q\u{012B}ng" },
    RadicalInfo { number: 175, radical: '\u{975E}', strokes: 8, meaning: "wrong",            pinyin: "f\u{0113}i" },
    // 9 strokes
    RadicalInfo { number: 176, radical: '\u{9762}', strokes: 9, meaning: "face",             pinyin: "mi\u{00E0}n" },
    RadicalInfo { number: 177, radical: '\u{9769}', strokes: 9, meaning: "leather",          pinyin: "g\u{00E9}" },
    RadicalInfo { number: 178, radical: '\u{97CB}', strokes: 9, meaning: "tanned leather",   pinyin: "w\u{00E9}i" },
    RadicalInfo { number: 179, radical: '\u{97ED}', strokes: 9, meaning: "leek",             pinyin: "ji\u{01D4}" },
    RadicalInfo { number: 180, radical: '\u{97F3}', strokes: 9, meaning: "sound",            pinyin: "y\u{012B}n" },
    RadicalInfo { number: 181, radical: '\u{9801}', strokes: 9, meaning: "leaf",             pinyin: "y\u{00E8}" },
    RadicalInfo { number: 182, radical: '\u{98A8}', strokes: 9, meaning: "wind",             pinyin: "f\u{0113}ng" },
    RadicalInfo { number: 183, radical: '\u{98DB}', strokes: 9, meaning: "fly",              pinyin: "f\u{0113}i" },
    RadicalInfo { number: 184, radical: '\u{98DF}', strokes: 9, meaning: "eat",              pinyin: "sh\u{00ED}" },
    RadicalInfo { number: 185, radical: '\u{9996}', strokes: 9, meaning: "head",             pinyin: "sh\u{01D2}u" },
    RadicalInfo { number: 186, radical: '\u{9999}', strokes: 9, meaning: "fragrant",         pinyin: "xi\u{0101}ng" },
    // 10 strokes
    RadicalInfo { number: 187, radical: '\u{99AC}', strokes: 10, meaning: "horse",           pinyin: "m\u{01CE}" },
    RadicalInfo { number: 188, radical: '\u{9AA8}', strokes: 10, meaning: "bone",            pinyin: "g\u{01D4}" },
    RadicalInfo { number: 189, radical: '\u{9AD8}', strokes: 10, meaning: "tall",            pinyin: "g\u{0101}o" },
    RadicalInfo { number: 190, radical: '\u{9ADF}', strokes: 10, meaning: "hair",            pinyin: "bi\u{0101}o" },
    RadicalInfo { number: 191, radical: '\u{9B25}', strokes: 10, meaning: "fight",           pinyin: "d\u{00F2}u" },
    RadicalInfo { number: 192, radical: '\u{9B2F}', strokes: 10, meaning: "sacrificial wine", pinyin: "ch\u{00E0}ng" },
    RadicalInfo { number: 193, radical: '\u{9B32}', strokes: 10, meaning: "cauldron",        pinyin: "l\u{00EC}" },
    RadicalInfo { number: 194, radical: '\u{9B3C}', strokes: 10, meaning: "ghost",           pinyin: "gu\u{01D0}" },
    // 11 strokes
    RadicalInfo { number: 195, radical: '\u{9B5A}', strokes: 11, meaning: "fish",            pinyin: "y\u{00FA}" },
    RadicalInfo { number: 196, radical: '\u{9CE5}', strokes: 11, meaning: "bird",            pinyin: "ni\u{01CE}o" },
    RadicalInfo { number: 197, radical: '\u{9E75}', strokes: 11, meaning: "salt",            pinyin: "l\u{01D4}" },
    RadicalInfo { number: 198, radical: '\u{9E7F}', strokes: 11, meaning: "deer",            pinyin: "l\u{00F9}" },
    // 12 strokes
    RadicalInfo { number: 199, radical: '\u{9EA5}', strokes: 11, meaning: "wheat",           pinyin: "m\u{00E0}i" },
    RadicalInfo { number: 200, radical: '\u{9EBB}', strokes: 11, meaning: "hemp",            pinyin: "m\u{00E1}" },
    // 12 strokes
    RadicalInfo { number: 201, radical: '\u{9EC3}', strokes: 12, meaning: "yellow",          pinyin: "hu\u{00E1}ng" },
    RadicalInfo { number: 202, radical: '\u{9ECD}', strokes: 12, meaning: "millet",          pinyin: "sh\u{01D4}" },
    RadicalInfo { number: 203, radical: '\u{9ED1}', strokes: 12, meaning: "black",           pinyin: "h\u{0113}i" },
    RadicalInfo { number: 204, radical: '\u{9EF9}', strokes: 12, meaning: "embroidery",      pinyin: "zh\u{01D0}" },
    // 13 strokes
    RadicalInfo { number: 205, radical: '\u{9EFD}', strokes: 13, meaning: "frog",            pinyin: "m\u{01D0}n" },
    RadicalInfo { number: 206, radical: '\u{9F0E}', strokes: 13, meaning: "tripod",          pinyin: "d\u{01D0}ng" },
    RadicalInfo { number: 207, radical: '\u{9F13}', strokes: 13, meaning: "drum",            pinyin: "g\u{01D4}" },
    RadicalInfo { number: 208, radical: '\u{9F20}', strokes: 13, meaning: "rat",             pinyin: "sh\u{01D4}" },
    // 14 strokes
    RadicalInfo { number: 209, radical: '\u{9F3B}', strokes: 14, meaning: "nose",            pinyin: "b\u{00ED}" },
    RadicalInfo { number: 210, radical: '\u{9F4A}', strokes: 14, meaning: "even",            pinyin: "q\u{00ED}" },
    // 15 strokes
    RadicalInfo { number: 211, radical: '\u{9F52}', strokes: 15, meaning: "tooth",           pinyin: "ch\u{01D0}" },
    // 16 strokes
    RadicalInfo { number: 212, radical: '\u{9F8D}', strokes: 16, meaning: "dragon",          pinyin: "l\u{00F3}ng" },
    RadicalInfo { number: 213, radical: '\u{9F9C}', strokes: 16, meaning: "turtle",          pinyin: "gu\u{012B}" },
    // 17 strokes
    RadicalInfo { number: 214, radical: '\u{9FA0}', strokes: 17, meaning: "flute",           pinyin: "yu\u{00E8}" },
];

/// Look up a Kangxi radical by its character.
pub fn lookup_radical(ch: char) -> Option<&'static RadicalInfo> {
    RADICALS.iter().find(|r| r.radical == ch)
}

/// Look up a Kangxi radical by its number (1-214).
pub fn lookup_by_number(num: u16) -> Option<&'static RadicalInfo> {
    RADICALS.iter().find(|r| r.number == num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_count() {
        assert_eq!(RADICALS.len(), 214);
    }

    #[test]
    fn test_numbers_sequential() {
        for (i, r) in RADICALS.iter().enumerate() {
            assert_eq!(r.number, (i + 1) as u16, "Radical at index {} has wrong number", i);
        }
    }

    #[test]
    fn test_checkpoint_radicals() {
        // Verify the key checkpoints from the requirements
        let checks: &[(u16, char, &str, &str)] = &[
            (1,   '\u{4E00}', "one",       "y\u{012B}"),
            (9,   '\u{4EBA}', "person",    "r\u{00E9}n"),
            (30,  '\u{53E3}', "mouth",     "k\u{01D2}u"),
            (32,  '\u{571F}', "earth",     "t\u{01D4}"),
            (37,  '\u{5927}', "big",       "d\u{00E0}"),
            (38,  '\u{5973}', "woman",     "n\u{01DA}"),
            (39,  '\u{5B50}', "child",     "z\u{01D0}"),
            (40,  '\u{5B80}', "roof",      "mi\u{00E1}n"),
            (46,  '\u{5C71}', "mountain",  "sh\u{0101}n"),
            (61,  '\u{5FC3}', "heart",     "x\u{012B}n"),
            (72,  '\u{65E5}', "sun",       "r\u{00EC}"),
            (74,  '\u{6708}', "moon",      "yu\u{00E8}"),
            (75,  '\u{6728}', "tree",      "m\u{00F9}"),
            (85,  '\u{6C34}', "water",     "shu\u{01D0}"),
            (86,  '\u{706B}', "fire",      "hu\u{01D2}"),
            (96,  '\u{7389}', "jade",      "y\u{00F9}"),
            (104, '\u{7592}', "sickness",  "n\u{00E8}"),
            (109, '\u{76EE}', "eye",       "m\u{00F9}"),
            (112, '\u{77F3}', "stone",     "sh\u{00ED}"),
            (113, '\u{793A}', "spirit",    "sh\u{00EC}"),
            (118, '\u{7AF9}', "bamboo",    "zh\u{00FA}"),
            (119, '\u{7C73}', "rice",      "m\u{01D0}"),
            (120, '\u{7CF8}', "silk",      "m\u{00EC}"),
            (130, '\u{8089}', "meat",      "r\u{00F2}u"),
            (140, '\u{8278}', "grass",     "c\u{01CE}o"),
            (142, '\u{866B}', "insect",    "ch\u{00F3}ng"),
            (149, '\u{8A00}', "speech",    "y\u{00E1}n"),
            (154, '\u{8C9D}', "shell",     "b\u{00E8}i"),
            (157, '\u{8DB3}', "foot",      "z\u{00FA}"),
            (162, '\u{8FB5}', "walk",      "chu\u{00F2}"),
            (167, '\u{91D1}', "gold",      "j\u{012B}n"),
            (169, '\u{9580}', "gate",      "m\u{00E9}n"),
            (181, '\u{9801}', "leaf",      "y\u{00E8}"),
            (187, '\u{99AC}', "horse",     "m\u{01CE}"),
            (195, '\u{9B5A}', "fish",      "y\u{00FA}"),
            (196, '\u{9CE5}', "bird",      "ni\u{01CE}o"),
        ];

        for &(num, ch, meaning, pinyin) in checks {
            let r = lookup_by_number(num).unwrap_or_else(|| panic!("Missing radical #{}", num));
            assert_eq!(r.radical, ch, "Radical #{} wrong char", num);
            assert_eq!(r.meaning, meaning, "Radical #{} wrong meaning", num);
            assert_eq!(r.pinyin, pinyin, "Radical #{} wrong pinyin", num);

            // Also test char lookup
            let r2 = lookup_radical(ch).unwrap();
            assert_eq!(r2.number, num);
        }
    }

    #[test]
    fn test_lookup_missing() {
        assert!(lookup_radical('A').is_none());
        assert!(lookup_by_number(0).is_none());
        assert!(lookup_by_number(215).is_none());
    }
}
