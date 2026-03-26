use serde::Serialize;

/// Breakdown of a single Chinese character into its semantic components.
#[derive(Debug, Serialize, Clone)]
pub struct CharBreakdown {
    pub character: char,
    pub pinyin: String,
    pub english: String,
    pub components: Vec<ComponentInfo>,
    pub mnemonic: String,
}

/// A semantic component of a Chinese character.
#[derive(Debug, Serialize, Clone)]
pub struct ComponentInfo {
    pub component: String,
    pub radical_number: Option<u16>,
    pub meaning: String,
}

struct CharEntry {
    character: char,
    pinyin: &'static str,
    english: &'static str,
    components: &'static [(&'static str, Option<u16>, &'static str)],
    mnemonic: &'static str,
}

const CHAR_TABLE: &[CharEntry] = &[
    CharEntry { character: '\u{4F11}', pinyin: "xi\u{016B}", english: "rest",
        components: &[("\u{4EBB}", Some(9), "person"), ("\u{6728}", Some(75), "tree")],
        mnemonic: "A person (\u{4EBB}) leaning against a tree (\u{6728}) \u{2014} to rest" },
    CharEntry { character: '\u{660E}', pinyin: "m\u{00ED}ng", english: "bright",
        components: &[("\u{65E5}", Some(72), "sun"), ("\u{6708}", Some(74), "moon")],
        mnemonic: "The sun (\u{65E5}) and moon (\u{6708}) together \u{2014} brightness" },
    CharEntry { character: '\u{597D}', pinyin: "h\u{01CE}o", english: "good",
        components: &[("\u{5973}", Some(38), "woman"), ("\u{5B50}", Some(39), "child")],
        mnemonic: "A woman (\u{5973}) with her child (\u{5B50}) \u{2014} goodness" },
    CharEntry { character: '\u{68EE}', pinyin: "s\u{0113}n", english: "forest",
        components: &[("\u{6728}", Some(75), "tree"), ("\u{6728}", Some(75), "tree"), ("\u{6728}", Some(75), "tree")],
        mnemonic: "Three trees (\u{6728}\u{6728}\u{6728}) together \u{2014} a dense forest" },
    CharEntry { character: '\u{4FE1}', pinyin: "x\u{00EC}n", english: "trust",
        components: &[("\u{4EBB}", Some(9), "person"), ("\u{8A00}", Some(149), "speech")],
        mnemonic: "A person (\u{4EBB}) standing by their word (\u{8A00}) \u{2014} trust" },
    CharEntry { character: '\u{5B89}', pinyin: "\u{0101}n", english: "peace",
        components: &[("\u{5B80}", Some(40), "roof"), ("\u{5973}", Some(38), "woman")],
        mnemonic: "A woman (\u{5973}) safe under a roof (\u{5B80}) \u{2014} peace" },
    CharEntry { character: '\u{5BB6}', pinyin: "ji\u{0101}", english: "home",
        components: &[("\u{5B80}", Some(40), "roof"), ("\u{8C55}", Some(152), "pig")],
        mnemonic: "A pig (\u{8C55}) under a roof (\u{5B80}) \u{2014} livestock means home" },
    CharEntry { character: '\u{60F3}', pinyin: "xi\u{01CE}ng", english: "think",
        components: &[("\u{76F8}", None, "mutual/appearance"), ("\u{5FC3}", Some(61), "heart")],
        mnemonic: "Images (\u{76F8}) arising in the heart (\u{5FC3}) \u{2014} to think" },
    CharEntry { character: '\u{611B}', pinyin: "\u{00E0}i", english: "love",
        components: &[("\u{7230}", None, "claws/hand"), ("\u{5FC3}", Some(61), "heart"), ("\u{53CB}", None, "friend")],
        mnemonic: "A hand reaching down, with heart (\u{5FC3}) inside \u{2014} love" },
    CharEntry { character: '\u{9053}', pinyin: "d\u{00E0}o", english: "way",
        components: &[("\u{8FB5}", Some(162), "walk"), ("\u{9996}", Some(185), "head")],
        mnemonic: "Walking (\u{8FB5}) with your head (\u{9996}) forward \u{2014} the way" },
    CharEntry { character: '\u{6797}', pinyin: "l\u{00ED}n", english: "grove",
        components: &[("\u{6728}", Some(75), "tree"), ("\u{6728}", Some(75), "tree")],
        mnemonic: "Two trees (\u{6728}\u{6728}) side by side \u{2014} a grove" },
    CharEntry { character: '\u{7537}', pinyin: "n\u{00E1}n", english: "man",
        components: &[("\u{7530}", Some(102), "field"), ("\u{529B}", Some(19), "power")],
        mnemonic: "Strength (\u{529B}) working the field (\u{7530}) \u{2014} a man" },
    CharEntry { character: '\u{5B57}', pinyin: "z\u{00EC}", english: "character/letter",
        components: &[("\u{5B80}", Some(40), "roof"), ("\u{5B50}", Some(39), "child")],
        mnemonic: "A child (\u{5B50}) learning under a roof (\u{5B80}) \u{2014} written character" },
    CharEntry { character: '\u{5C16}', pinyin: "ji\u{0101}n", english: "sharp/pointed",
        components: &[("\u{5C0F}", Some(42), "small"), ("\u{5927}", Some(37), "big")],
        mnemonic: "Small (\u{5C0F}) on top, big (\u{5927}) below \u{2014} tapering to a point" },
    CharEntry { character: '\u{5C96}', pinyin: "qi\u{00ED}", english: "high and steep",
        components: &[("\u{5C71}", Some(46), "mountain"), ("\u{5947}", None, "strange")],
        mnemonic: "A strange (\u{5947}) mountain (\u{5C71}) \u{2014} towering and steep" },
    CharEntry { character: '\u{709C}', pinyin: "m\u{00E8}n", english: "bored/stuffy",
        components: &[("\u{706B}", Some(86), "fire"), ("\u{9580}", Some(169), "gate")],
        mnemonic: "Fire (\u{706B}) behind a closed gate (\u{9580}) \u{2014} stifling" },
    CharEntry { character: '\u{4F53}', pinyin: "t\u{01D0}", english: "body",
        components: &[("\u{4EBB}", Some(9), "person"), ("\u{672C}", None, "root/origin")],
        mnemonic: "A person (\u{4EBB}) and their root (\u{672C}) \u{2014} the body" },
    CharEntry { character: '\u{5F00}', pinyin: "k\u{0101}i", english: "open",
        components: &[("\u{4E00}", Some(1), "one"), ("\u{5F00}", None, "two hands")],
        mnemonic: "Two hands lifting a bar (\u{4E00}) \u{2014} to open" },
    CharEntry { character: '\u{5C11}', pinyin: "sh\u{01CE}o", english: "few/less",
        components: &[("\u{5C0F}", Some(42), "small"), ("\u{4E3F}", Some(4), "slash")],
        mnemonic: "Something small (\u{5C0F}) cut down (\u{4E3F}) \u{2014} even fewer" },
    CharEntry { character: '\u{706B}', pinyin: "hu\u{01D2}", english: "fire",
        components: &[("\u{706B}", Some(86), "fire")],
        mnemonic: "Pictograph of flames rising \u{2014} fire itself" },
    CharEntry { character: '\u{5C71}', pinyin: "sh\u{0101}n", english: "mountain",
        components: &[("\u{5C71}", Some(46), "mountain")],
        mnemonic: "Three peaks rising from the earth \u{2014} a mountain" },
    CharEntry { character: '\u{6C34}', pinyin: "shu\u{01D0}", english: "water",
        components: &[("\u{6C34}", Some(85), "water")],
        mnemonic: "A stream flowing with splashes on each side \u{2014} water" },
    CharEntry { character: '\u{6728}', pinyin: "m\u{00F9}", english: "tree",
        components: &[("\u{6728}", Some(75), "tree")],
        mnemonic: "Trunk rising with branches above and roots below \u{2014} tree" },
    CharEntry { character: '\u{571F}', pinyin: "t\u{01D4}", english: "earth",
        components: &[("\u{571F}", Some(32), "earth")],
        mnemonic: "A mound of soil on the ground \u{2014} earth" },
    CharEntry { character: '\u{91D1}', pinyin: "j\u{012B}n", english: "gold/metal",
        components: &[("\u{91D1}", Some(167), "gold")],
        mnemonic: "Nuggets buried in the earth \u{2014} gold and metal" },
    CharEntry { character: '\u{65E5}', pinyin: "r\u{00EC}", english: "sun/day",
        components: &[("\u{65E5}", Some(72), "sun")],
        mnemonic: "A circle with a line through it \u{2014} the sun" },
    CharEntry { character: '\u{6708}', pinyin: "yu\u{00E8}", english: "moon/month",
        components: &[("\u{6708}", Some(74), "moon")],
        mnemonic: "A crescent shape \u{2014} the moon" },
    CharEntry { character: '\u{53E3}', pinyin: "k\u{01D2}u", english: "mouth",
        components: &[("\u{53E3}", Some(30), "mouth")],
        mnemonic: "An open square \u{2014} a mouth" },
    CharEntry { character: '\u{76EE}', pinyin: "m\u{00F9}", english: "eye",
        components: &[("\u{76EE}", Some(109), "eye")],
        mnemonic: "An eye turned on its side \u{2014} the eye" },
    CharEntry { character: '\u{8033}', pinyin: "\u{011B}r", english: "ear",
        components: &[("\u{8033}", Some(128), "ear")],
        mnemonic: "An ear drawn from the side \u{2014} the ear" },
    CharEntry { character: '\u{5FC3}', pinyin: "x\u{012B}n", english: "heart",
        components: &[("\u{5FC3}", Some(61), "heart")],
        mnemonic: "An organ with chambers \u{2014} the heart" },
    CharEntry { character: '\u{624B}', pinyin: "sh\u{01D2}u", english: "hand",
        components: &[("\u{624B}", Some(64), "hand")],
        mnemonic: "Five fingers spread out \u{2014} the hand" },
    CharEntry { character: '\u{8DB3}', pinyin: "z\u{00FA}", english: "foot",
        components: &[("\u{8DB3}", Some(157), "foot")],
        mnemonic: "A foot with ankle and toes \u{2014} the foot" },
    CharEntry { character: '\u{95E8}', pinyin: "m\u{00E9}n", english: "gate",
        components: &[("\u{95E8}", Some(169), "gate")],
        mnemonic: "Two swinging doors \u{2014} a gate" },
    CharEntry { character: '\u{9A6C}', pinyin: "m\u{01CE}", english: "horse",
        components: &[("\u{99AC}", Some(187), "horse")],
        mnemonic: "A galloping animal with flowing mane \u{2014} horse" },
    CharEntry { character: '\u{9C7C}', pinyin: "y\u{00FA}", english: "fish",
        components: &[("\u{9B5A}", Some(195), "fish")],
        mnemonic: "A creature with scales and tail in water \u{2014} fish" },
    CharEntry { character: '\u{9E1F}', pinyin: "ni\u{01CE}o", english: "bird",
        components: &[("\u{9CE5}", Some(196), "bird")],
        mnemonic: "A feathered creature with beak and claws \u{2014} bird" },
    CharEntry { character: '\u{5973}', pinyin: "n\u{01DA}", english: "woman",
        components: &[("\u{5973}", Some(38), "woman")],
        mnemonic: "A figure kneeling gracefully \u{2014} woman" },
    CharEntry { character: '\u{5B50}', pinyin: "z\u{01D0}", english: "child",
        components: &[("\u{5B50}", Some(39), "child")],
        mnemonic: "A small figure with arms outstretched \u{2014} a child" },
    CharEntry { character: '\u{4EBA}', pinyin: "r\u{00E9}n", english: "person",
        components: &[("\u{4EBA}", Some(9), "person")],
        mnemonic: "Two strokes leaning on each other \u{2014} a person" },
    CharEntry { character: '\u{5927}', pinyin: "d\u{00E0}", english: "big",
        components: &[("\u{5927}", Some(37), "big")],
        mnemonic: "A person stretching arms wide \u{2014} big" },
    CharEntry { character: '\u{5929}', pinyin: "ti\u{0101}n", english: "sky/heaven",
        components: &[("\u{4E00}", Some(1), "one"), ("\u{5927}", Some(37), "big")],
        mnemonic: "Above (\u{4E00}) the great (\u{5927}) person \u{2014} the sky" },
    CharEntry { character: '\u{592B}', pinyin: "f\u{016B}", english: "husband",
        components: &[("\u{4E00}", Some(1), "one"), ("\u{5927}", Some(37), "big")],
        mnemonic: "A big (\u{5927}) man with a pin (\u{4E00}) in his hair \u{2014} a grown man" },
    CharEntry { character: '\u{592A}', pinyin: "t\u{00E0}i", english: "too/very",
        components: &[("\u{5927}", Some(37), "big"), ("\u{4E36}", Some(3), "dot")],
        mnemonic: "Something big (\u{5927}) with extra emphasis (\u{4E36}) \u{2014} too much" },
    CharEntry { character: '\u{5C17}', pinyin: "sh\u{00E0}ng", english: "still/yet",
        components: &[("\u{5C0F}", Some(42), "small"), ("\u{5182}", Some(13), "down box")],
        mnemonic: "A window (\u{5182}) over something small (\u{5C0F}) \u{2014} value what is modest" },
    CharEntry { character: '\u{5473}', pinyin: "w\u{00E8}i", english: "flavor/taste",
        components: &[("\u{53E3}", Some(30), "mouth"), ("\u{672A}", None, "not yet")],
        mnemonic: "The mouth (\u{53E3}) sensing what is not yet (\u{672A}) known \u{2014} taste" },
    CharEntry { character: '\u{5403}', pinyin: "ch\u{012B}", english: "eat",
        components: &[("\u{53E3}", Some(30), "mouth"), ("\u{4E5E}", None, "beg")],
        mnemonic: "A mouth (\u{53E3}) begging (\u{4E5E}) for food \u{2014} to eat" },
    CharEntry { character: '\u{5410}', pinyin: "t\u{01D4}", english: "spit",
        components: &[("\u{53E3}", Some(30), "mouth"), ("\u{571F}", Some(32), "earth")],
        mnemonic: "Something earthy (\u{571F}) coming from the mouth (\u{53E3}) \u{2014} to spit" },
    CharEntry { character: '\u{5524}', pinyin: "hu\u{00E0}n", english: "call/summon",
        components: &[("\u{53E3}", Some(30), "mouth"), ("\u{5944}", None, "abundant")],
        mnemonic: "A mouth (\u{53E3}) calling out abundantly \u{2014} to summon" },
    CharEntry { character: '\u{5976}', pinyin: "n\u{01CE}i", english: "milk/breast",
        components: &[("\u{5973}", Some(38), "woman"), ("\u{4E43}", None, "thus")],
        mnemonic: "A woman (\u{5973}) nursing \u{2014} milk" },
    CharEntry { character: '\u{59B9}', pinyin: "m\u{00E8}i", english: "younger sister",
        components: &[("\u{5973}", Some(38), "woman"), ("\u{672A}", None, "not yet")],
        mnemonic: "A woman (\u{5973}) not yet (\u{672A}) grown \u{2014} younger sister" },
    CharEntry { character: '\u{59C8}', pinyin: "ji\u{011B}", english: "older sister",
        components: &[("\u{5973}", Some(38), "woman"), ("\u{59D0}", None, "elder")],
        mnemonic: "A woman (\u{5973}) of senior standing \u{2014} older sister" },
    CharEntry { character: '\u{5988}', pinyin: "m\u{0101}", english: "mother",
        components: &[("\u{5973}", Some(38), "woman"), ("\u{99AC}", Some(187), "horse")],
        mnemonic: "A woman (\u{5973}) with the sound of horse (\u{99AC}) \u{2014} mama" },
    CharEntry { character: '\u{6CB3}', pinyin: "h\u{00E9}", english: "river",
        components: &[("\u{6C35}", Some(85), "water"), ("\u{53EF}", None, "can")],
        mnemonic: "Water (\u{6C35}) that can (\u{53EF}) flow freely \u{2014} a river" },
    CharEntry { character: '\u{6D77}', pinyin: "h\u{01CE}i", english: "sea",
        components: &[("\u{6C35}", Some(85), "water"), ("\u{6BCF}", None, "every")],
        mnemonic: "Water (\u{6C35}) in every (\u{6BCF}) direction \u{2014} the sea" },
    CharEntry { character: '\u{6C60}', pinyin: "ch\u{00ED}", english: "pool/pond",
        components: &[("\u{6C35}", Some(85), "water"), ("\u{4E5F}", None, "also")],
        mnemonic: "Water (\u{6C35}) that also (\u{4E5F}) gathers \u{2014} a pond" },
    CharEntry { character: '\u{6C57}', pinyin: "h\u{00E0}n", english: "sweat",
        components: &[("\u{6C35}", Some(85), "water"), ("\u{5E72}", Some(51), "dry")],
        mnemonic: "Water (\u{6C35}) that dries (\u{5E72}) on the skin \u{2014} sweat" },
    CharEntry { character: '\u{6CE3}', pinyin: "q\u{00EC}", english: "weep",
        components: &[("\u{6C35}", Some(85), "water"), ("\u{7ACB}", Some(117), "stand")],
        mnemonic: "Water (\u{6C35}) standing (\u{7ACB}) in the eyes \u{2014} to weep" },
    CharEntry { character: '\u{70E7}', pinyin: "sh\u{0101}o", english: "burn",
        components: &[("\u{706B}", Some(86), "fire"), ("\u{5C27}", None, "Yao")],
        mnemonic: "Fire (\u{706B}) blazing upward \u{2014} to burn" },
    CharEntry { character: '\u{7167}', pinyin: "zh\u{00E0}o", english: "shine/illuminate",
        components: &[("\u{65E5}", Some(72), "sun"), ("\u{662D}", None, "bright")],
        mnemonic: "The sun (\u{65E5}) shining brightly (\u{662D}) \u{2014} to illuminate" },
    CharEntry { character: '\u{6653}', pinyin: "xi\u{01CE}o", english: "dawn",
        components: &[("\u{65E5}", Some(72), "sun"), ("\u{5C27}", None, "Yao")],
        mnemonic: "The sun (\u{65E5}) just beginning to rise \u{2014} dawn" },
    CharEntry { character: '\u{6697}', pinyin: "\u{00E0}n", english: "dark",
        components: &[("\u{65E5}", Some(72), "sun"), ("\u{97F3}", Some(180), "sound")],
        mnemonic: "When the sun (\u{65E5}) can only be heard (\u{97F3}) not seen \u{2014} darkness" },
    CharEntry { character: '\u{6674}', pinyin: "q\u{00ED}ng", english: "clear weather",
        components: &[("\u{65E5}", Some(72), "sun"), ("\u{9752}", Some(174), "blue")],
        mnemonic: "The sun (\u{65E5}) in a blue (\u{9752}) sky \u{2014} clear weather" },
    CharEntry { character: '\u{6811}', pinyin: "sh\u{00F9}", english: "tree (planted)",
        components: &[("\u{6728}", Some(75), "tree"), ("\u{53C8}", Some(29), "again"), ("\u{5BF8}", Some(41), "inch")],
        mnemonic: "Carefully (\u{5BF8}) planting wood (\u{6728}) again (\u{53C8}) \u{2014} a cultivated tree" },
    CharEntry { character: '\u{677E}', pinyin: "s\u{014D}ng", english: "pine tree",
        components: &[("\u{6728}", Some(75), "tree"), ("\u{516C}", None, "public")],
        mnemonic: "A tree (\u{6728}) that belongs to all (\u{516C}) \u{2014} the evergreen pine" },
    CharEntry { character: '\u{82B1}', pinyin: "hu\u{0101}", english: "flower",
        components: &[("\u{8278}", Some(140), "grass"), ("\u{5316}", None, "change")],
        mnemonic: "Grass (\u{8278}) that transforms (\u{5316}) \u{2014} a flower blooming" },
    CharEntry { character: '\u{8349}', pinyin: "c\u{01CE}o", english: "grass",
        components: &[("\u{8278}", Some(140), "grass"), ("\u{65E9}", None, "early")],
        mnemonic: "Early (\u{65E9}) growing plants (\u{8278}) \u{2014} grass" },
    CharEntry { character: '\u{82D7}', pinyin: "mi\u{00E1}o", english: "seedling",
        components: &[("\u{8278}", Some(140), "grass"), ("\u{7530}", Some(102), "field")],
        mnemonic: "Grass (\u{8278}) sprouting in the field (\u{7530}) \u{2014} a seedling" },
    CharEntry { character: '\u{8336}', pinyin: "ch\u{00E1}", english: "tea",
        components: &[("\u{8278}", Some(140), "grass"), ("\u{4EBA}", Some(9), "person"), ("\u{6728}", Some(75), "tree")],
        mnemonic: "A person (\u{4EBA}) among plants (\u{8278}) and trees (\u{6728}) \u{2014} tea" },
    CharEntry { character: '\u{5065}', pinyin: "ji\u{00E0}n", english: "healthy/strong",
        components: &[("\u{4EBB}", Some(9), "person"), ("\u{5EFA}", None, "build")],
        mnemonic: "A person (\u{4EBB}) who builds (\u{5EFA}) themselves up \u{2014} healthy" },
    CharEntry { character: '\u{4F1A}', pinyin: "hu\u{00EC}", english: "meet/gather",
        components: &[("\u{4EBA}", Some(9), "person"), ("\u{4E91}", None, "cloud")],
        mnemonic: "People (\u{4EBA}) gathering like clouds (\u{4E91}) \u{2014} to meet" },
    CharEntry { character: '\u{4F4D}', pinyin: "w\u{00E8}i", english: "position/seat",
        components: &[("\u{4EBB}", Some(9), "person"), ("\u{7ACB}", Some(117), "stand")],
        mnemonic: "Where a person (\u{4EBB}) stands (\u{7ACB}) \u{2014} their position" },
    CharEntry { character: '\u{4F4F}', pinyin: "zh\u{00F9}", english: "live/reside",
        components: &[("\u{4EBB}", Some(9), "person"), ("\u{4E3B}", None, "master")],
        mnemonic: "A person (\u{4EBB}) who is master (\u{4E3B}) of a place \u{2014} to reside" },
    CharEntry { character: '\u{4F5C}', pinyin: "zu\u{00F2}", english: "do/make",
        components: &[("\u{4EBB}", Some(9), "person"), ("\u{4E4D}", None, "suddenly")],
        mnemonic: "A person (\u{4EBB}) springing into action \u{2014} to make" },
    CharEntry { character: '\u{4FDD}', pinyin: "b\u{01CE}o", english: "protect",
        components: &[("\u{4EBB}", Some(9), "person"), ("\u{5446}", None, "stay")],
        mnemonic: "A person (\u{4EBB}) wrapping something safely \u{2014} to protect" },
    CharEntry { character: '\u{6B65}', pinyin: "b\u{00F9}", english: "step/walk",
        components: &[("\u{6B62}", Some(77), "stop"), ("\u{5C11}", None, "few")],
        mnemonic: "Stopping (\u{6B62}) and starting again \u{2014} walking step by step" },
    CharEntry { character: '\u{6B63}', pinyin: "zh\u{00E8}ng", english: "correct/straight",
        components: &[("\u{4E00}", Some(1), "one"), ("\u{6B62}", Some(77), "stop")],
        mnemonic: "Stopping (\u{6B62}) at the right line (\u{4E00}) \u{2014} correct" },
    CharEntry { character: '\u{6B7B}', pinyin: "s\u{01D0}", english: "die/death",
        components: &[("\u{6B79}", Some(78), "death"), ("\u{5315}", Some(21), "spoon")],
        mnemonic: "A fallen figure (\u{6B79}) and bones (\u{5315}) \u{2014} death" },
    CharEntry { character: '\u{770B}', pinyin: "k\u{00E0}n", english: "look/see",
        components: &[("\u{624B}", Some(64), "hand"), ("\u{76EE}", Some(109), "eye")],
        mnemonic: "A hand (\u{624B}) shading the eye (\u{76EE}) to look into the distance" },
    CharEntry { character: '\u{7761}', pinyin: "shu\u{00EC}", english: "sleep",
        components: &[("\u{76EE}", Some(109), "eye"), ("\u{5782}", None, "droop")],
        mnemonic: "Eyes (\u{76EE}) drooping (\u{5782}) shut \u{2014} sleep" },
    CharEntry { character: '\u{76F8}', pinyin: "xi\u{0101}ng", english: "mutual/appearance",
        components: &[("\u{6728}", Some(75), "tree"), ("\u{76EE}", Some(109), "eye")],
        mnemonic: "An eye (\u{76EE}) examining a tree (\u{6728}) \u{2014} to observe, mutuality" },
    CharEntry { character: '\u{77ED}', pinyin: "du\u{01CE}n", english: "short",
        components: &[("\u{77F3}", Some(112), "stone"), ("\u{8C46}", Some(151), "bean")],
        mnemonic: "As short as a stone (\u{77F3}) next to a bean (\u{8C46}) \u{2014} short" },
    CharEntry { character: '\u{7965}', pinyin: "xi\u{00E1}ng", english: "auspicious",
        components: &[("\u{793A}", Some(113), "spirit/altar"), ("\u{7F8A}", Some(123), "sheep")],
        mnemonic: "A sheep (\u{7F8A}) offered at the altar (\u{793A}) \u{2014} auspicious" },
    CharEntry { character: '\u{795E}', pinyin: "sh\u{00E9}n", english: "god/spirit",
        components: &[("\u{793A}", Some(113), "spirit/altar"), ("\u{7533}", None, "extend")],
        mnemonic: "The spirit (\u{793A}) extending (\u{7533}) its power \u{2014} a god" },
    CharEntry { character: '\u{7956}', pinyin: "z\u{01D4}", english: "ancestor",
        components: &[("\u{793A}", Some(113), "spirit/altar"), ("\u{4E14}", None, "moreover")],
        mnemonic: "Spirits (\u{793A}) of those who came before \u{2014} ancestors" },
    CharEntry { character: '\u{7B11}', pinyin: "xi\u{00E0}o", english: "laugh/smile",
        components: &[("\u{7AF9}", Some(118), "bamboo"), ("\u{5929}", None, "heaven")],
        mnemonic: "Bamboo (\u{7AF9}) swaying like a joyful face \u{2014} to laugh" },
    CharEntry { character: '\u{7B49}', pinyin: "d\u{011B}ng", english: "wait/equal",
        components: &[("\u{7AF9}", Some(118), "bamboo"), ("\u{5BFA}", None, "temple")],
        mnemonic: "Bamboo (\u{7AF9}) slips sorted at the temple (\u{5BFA}) \u{2014} equal rank" },
    CharEntry { character: '\u{7C7B}', pinyin: "l\u{00E8}i", english: "kind/type",
        components: &[("\u{7C73}", Some(119), "rice"), ("\u{5927}", Some(37), "big")],
        mnemonic: "Sorting big (\u{5927}) grains of rice (\u{7C73}) by type" },
    CharEntry { character: '\u{7EA2}', pinyin: "h\u{00F3}ng", english: "red",
        components: &[("\u{7CF8}", Some(120), "silk"), ("\u{5DE5}", Some(48), "work")],
        mnemonic: "Silk (\u{7CF8}) dyed with worked (\u{5DE5}) pigment \u{2014} red" },
    CharEntry { character: '\u{7EBF}', pinyin: "xi\u{00E0}n", english: "thread/line",
        components: &[("\u{7CF8}", Some(120), "silk"), ("\u{6CC9}", None, "spring")],
        mnemonic: "Silk (\u{7CF8}) drawn out thin like a spring's flow \u{2014} a thread" },
    CharEntry { character: '\u{8BDD}', pinyin: "hu\u{00E0}", english: "speech/words",
        components: &[("\u{8A00}", Some(149), "speech"), ("\u{820C}", Some(135), "tongue")],
        mnemonic: "Speech (\u{8A00}) shaped by the tongue (\u{820C}) \u{2014} words" },
    CharEntry { character: '\u{8BA4}', pinyin: "r\u{00E8}n", english: "recognize",
        components: &[("\u{8A00}", Some(149), "speech"), ("\u{4EBA}", Some(9), "person")],
        mnemonic: "Speech (\u{8A00}) that identifies a person (\u{4EBA}) \u{2014} to recognize" },
    CharEntry { character: '\u{8BFB}', pinyin: "d\u{00FA}", english: "read",
        components: &[("\u{8A00}", Some(149), "speech"), ("\u{5356}", None, "sell")],
        mnemonic: "Giving voice (\u{8A00}) to written text \u{2014} to read aloud" },
    CharEntry { character: '\u{8D70}', pinyin: "z\u{01D2}u", english: "walk/go",
        components: &[("\u{8D70}", Some(156), "run")],
        mnemonic: "A person striding forward with swinging arms \u{2014} to walk" },
    CharEntry { character: '\u{8DEF}', pinyin: "l\u{00F9}", english: "road/path",
        components: &[("\u{8DB3}", Some(157), "foot"), ("\u{5404}", None, "each")],
        mnemonic: "Where each (\u{5404}) foot (\u{8DB3}) treads \u{2014} a road" },
    CharEntry { character: '\u{8DF3}', pinyin: "ti\u{00E0}o", english: "jump",
        components: &[("\u{8DB3}", Some(157), "foot"), ("\u{5146}", None, "omen")],
        mnemonic: "Feet (\u{8DB3}) springing upward \u{2014} to jump" },
    CharEntry { character: '\u{94C1}', pinyin: "ti\u{011B}", english: "iron",
        components: &[("\u{91D1}", Some(167), "gold/metal"), ("\u{5931}", None, "lose")],
        mnemonic: "A metal (\u{91D1}) that is common, not precious \u{2014} iron" },
    CharEntry { character: '\u{94F6}', pinyin: "y\u{00ED}n", english: "silver",
        components: &[("\u{91D1}", Some(167), "gold/metal"), ("\u{826E}", Some(138), "stopping")],
        mnemonic: "A metal (\u{91D1}) of gentle (\u{826E}) luster \u{2014} silver" },
    CharEntry { character: '\u{95EA}', pinyin: "sh\u{01CE}n", english: "flash/dodge",
        components: &[("\u{95E8}", Some(169), "gate"), ("\u{4EBA}", Some(9), "person")],
        mnemonic: "A person (\u{4EBA}) darting through a gate (\u{95E8}) \u{2014} a flash" },
    CharEntry { character: '\u{95F4}', pinyin: "ji\u{0101}n", english: "between/space",
        components: &[("\u{95E8}", Some(169), "gate"), ("\u{65E5}", Some(72), "sun")],
        mnemonic: "Sunlight (\u{65E5}) peeking through a gate (\u{95E8}) \u{2014} a gap between" },
    CharEntry { character: '\u{95EE}', pinyin: "w\u{00E8}n", english: "ask",
        components: &[("\u{95E8}", Some(169), "gate"), ("\u{53E3}", Some(30), "mouth")],
        mnemonic: "A mouth (\u{53E3}) at the gate (\u{95E8}) \u{2014} to ask" },
    CharEntry { character: '\u{5B66}', pinyin: "xu\u{00E9}", english: "study/learn",
        components: &[("\u{5B50}", Some(39), "child"), ("\u{5B80}", Some(40), "roof")],
        mnemonic: "A child (\u{5B50}) under a roof (\u{5B80}) with books \u{2014} to study" },
    CharEntry { character: '\u{5B88}', pinyin: "sh\u{01D2}u", english: "guard/defend",
        components: &[("\u{5B80}", Some(40), "roof"), ("\u{5BF8}", Some(41), "inch")],
        mnemonic: "Measured (\u{5BF8}) protection under a roof (\u{5B80}) \u{2014} to guard" },
    CharEntry { character: '\u{5B8C}', pinyin: "w\u{00E1}n", english: "finish/complete",
        components: &[("\u{5B80}", Some(40), "roof"), ("\u{5143}", None, "origin")],
        mnemonic: "Everything (\u{5143}) gathered under the roof (\u{5B80}) \u{2014} complete" },
];

/// Look up the decomposition of a Chinese character.
pub fn decompose_char(ch: char) -> Option<CharBreakdown> {
    CHAR_TABLE.iter().find(|e| e.character == ch).map(|e| {
        CharBreakdown {
            character: e.character,
            pinyin: e.pinyin.to_string(),
            english: e.english.to_string(),
            components: e.components.iter().map(|&(comp, rad, meaning)| {
                ComponentInfo {
                    component: comp.to_string(),
                    radical_number: rad,
                    meaning: meaning.to_string(),
                }
            }).collect(),
            mnemonic: e.mnemonic.to_string(),
        }
    })
}

/// Generate a narrative memory story for a character breakdown.
pub fn char_narrative(breakdown: &CharBreakdown) -> String {
    if breakdown.components.is_empty() {
        return format!(
            "{} ({}, {}) \u{2014} a character in its elemental form.",
            breakdown.character, breakdown.pinyin, breakdown.english
        );
    }

    let component_phrases: Vec<String> = breakdown.components.iter().map(|c| {
        format!("{} ({})", c.component, c.meaning)
    }).collect();

    let components_str = if component_phrases.len() == 1 {
        component_phrases[0].clone()
    } else if component_phrases.len() == 2 {
        format!("{} and {}", component_phrases[0], component_phrases[1])
    } else {
        let last = component_phrases.last().unwrap();
        let rest = &component_phrases[..component_phrases.len() - 1];
        format!("{}, and {}", rest.join(", "), last)
    };

    format!(
        "{} ({}, \"{}\") = {} \u{2014} {}",
        breakdown.character,
        breakdown.pinyin,
        breakdown.english,
        components_str,
        breakdown.mnemonic
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_rest() {
        let b = decompose_char('\u{4F11}').expect("should find rest");
        assert_eq!(b.character, '\u{4F11}');
        assert_eq!(b.english, "rest");
        assert_eq!(b.components.len(), 2);
        assert_eq!(b.components[0].meaning, "person");
        assert_eq!(b.components[1].meaning, "tree");
        assert_eq!(b.components[0].radical_number, Some(9));
        assert_eq!(b.components[1].radical_number, Some(75));
    }

    #[test]
    fn test_decompose_bright() {
        let b = decompose_char('\u{660E}').expect("should find bright");
        assert_eq!(b.english, "bright");
        assert_eq!(b.components.len(), 2);
        assert_eq!(b.components[0].meaning, "sun");
        assert_eq!(b.components[1].meaning, "moon");
    }

    #[test]
    fn test_decompose_good() {
        let b = decompose_char('\u{597D}').expect("should find good");
        assert_eq!(b.english, "good");
        assert_eq!(b.components[0].meaning, "woman");
        assert_eq!(b.components[1].meaning, "child");
    }

    #[test]
    fn test_decompose_forest() {
        let b = decompose_char('\u{68EE}').expect("should find forest");
        assert_eq!(b.english, "forest");
        assert_eq!(b.components.len(), 3);
        for c in &b.components {
            assert_eq!(c.meaning, "tree");
        }
    }

    #[test]
    fn test_decompose_trust() {
        let b = decompose_char('\u{4FE1}').expect("should find trust");
        assert_eq!(b.english, "trust");
        assert_eq!(b.components[0].meaning, "person");
        assert_eq!(b.components[1].meaning, "speech");
    }

    #[test]
    fn test_decompose_peace() {
        let b = decompose_char('\u{5B89}').expect("should find peace");
        assert_eq!(b.english, "peace");
        assert_eq!(b.components[0].meaning, "roof");
        assert_eq!(b.components[1].meaning, "woman");
    }

    #[test]
    fn test_decompose_home() {
        let b = decompose_char('\u{5BB6}').expect("should find home");
        assert_eq!(b.english, "home");
        assert_eq!(b.components[0].meaning, "roof");
        assert_eq!(b.components[1].meaning, "pig");
    }

    #[test]
    fn test_decompose_think() {
        let b = decompose_char('\u{60F3}').expect("should find think");
        assert_eq!(b.english, "think");
        assert_eq!(b.components[1].meaning, "heart");
    }

    #[test]
    fn test_decompose_love() {
        let b = decompose_char('\u{611B}').expect("should find love");
        assert_eq!(b.english, "love");
        assert!(b.components.iter().any(|c| c.meaning == "heart"));
    }

    #[test]
    fn test_decompose_way() {
        let b = decompose_char('\u{9053}').expect("should find way");
        assert_eq!(b.english, "way");
        assert_eq!(b.components[0].meaning, "walk");
        assert_eq!(b.components[1].meaning, "head");
    }

    #[test]
    fn test_decompose_missing() {
        assert!(decompose_char('Z').is_none());
    }

    #[test]
    fn test_char_count() {
        assert!(CHAR_TABLE.len() >= 100, "Should have at least 100 characters, got {}", CHAR_TABLE.len());
    }

    #[test]
    fn test_narrative_rest() {
        let b = decompose_char('\u{4F11}').unwrap();
        let n = char_narrative(&b);
        assert!(n.contains('\u{4F11}'));
        assert!(n.contains("rest"));
        assert!(n.contains("person"));
        assert!(n.contains("tree"));
    }

    #[test]
    fn test_narrative_forest() {
        let b = decompose_char('\u{68EE}').unwrap();
        let n = char_narrative(&b);
        assert!(n.contains("forest"));
        assert!(n.contains("tree"));
    }

    #[test]
    fn test_narrative_single_component() {
        let b = decompose_char('\u{706B}').unwrap();
        let n = char_narrative(&b);
        assert!(n.contains("fire"));
    }

    #[test]
    fn test_all_entries_have_components() {
        for entry in CHAR_TABLE {
            assert!(!entry.components.is_empty(),
                "Character {} should have at least one component", entry.character);
        }
    }

    #[test]
    fn test_all_entries_have_mnemonic() {
        for entry in CHAR_TABLE {
            assert!(!entry.mnemonic.is_empty(),
                "Character {} should have a mnemonic", entry.character);
        }
    }
}
