use serde::Serialize;

/// Information about the Oracle Bone Script form of a Chinese character.
#[derive(Debug, Serialize, Clone)]
pub struct OracleBoneInfo {
    pub modern: char,
    pub description: &'static str,
    pub pictograph: &'static str,
    pub evolution: &'static str,
}

struct OracleBoneEntry {
    modern: char,
    description: &'static str,
    pictograph: &'static str,
    evolution: &'static str,
}

const ORACLE_BONE_TABLE: &[OracleBoneEntry] = &[
    // ===== Nature / Elements =====
    OracleBoneEntry { modern: '\u{65E5}', // 日
        description: "Circle with a dot in the center, depicting the sun",
        pictograph: "(O)",
        evolution: "Picture of sun with center mark -> square with line -> modern 日" },
    OracleBoneEntry { modern: '\u{6708}', // 月
        description: "Crescent shape depicting the moon",
        pictograph: "C)",
        evolution: "Crescent moon shape -> filled crescent -> modern 月" },
    OracleBoneEntry { modern: '\u{6C34}', // 水
        description: "Flowing stream with droplets on either side of a central current",
        pictograph: "~.~",
        evolution: "Flowing water with splashes -> wavy lines -> modern 水" },
    OracleBoneEntry { modern: '\u{706B}', // 火
        description: "Flames rising upward from a base",
        pictograph: "/\\,",
        evolution: "Leaping flames -> stylized fire -> modern 火" },
    OracleBoneEntry { modern: '\u{5C71}', // 山
        description: "Three peaks, the middle one tallest",
        pictograph: "/|\\",
        evolution: "Three mountain peaks -> three vertical strokes -> modern 山" },
    OracleBoneEntry { modern: '\u{6728}', // 木
        description: "Tree with branches above and roots below",
        pictograph: "-|-",
        evolution: "Picture of tree with branches and roots -> simplified trunk -> modern 木" },
    OracleBoneEntry { modern: '\u{7530}', // 田
        description: "Square divided into plots, depicting cultivated farmland",
        pictograph: "[#]",
        evolution: "Divided field plot -> grid pattern -> modern 田" },
    OracleBoneEntry { modern: '\u{77F3}', // 石
        description: "A rock beneath a cliff overhang",
        pictograph: "n_O",
        evolution: "Rock under cliff -> angular form -> modern 石" },
    OracleBoneEntry { modern: '\u{96E8}', // 雨
        description: "Drops falling from a cloud or sky line",
        pictograph: "=,,=",
        evolution: "Raindrops from sky -> drops under horizontal line -> modern 雨" },
    OracleBoneEntry { modern: '\u{571F}', // 土
        description: "A mound of earth on the ground",
        pictograph: "_+_",
        evolution: "Mound of soil on ground line -> cross on line -> modern 土" },
    OracleBoneEntry { modern: '\u{98A8}', // 風
        description: "A mythical bird-like creature representing wind",
        pictograph: "~>",
        evolution: "Phoenix-like wind spirit -> stylized form -> modern 風" },
    OracleBoneEntry { modern: '\u{96F2}', // 雲
        description: "Swirling vapor rising under the sky",
        pictograph: "=S=",
        evolution: "Rising clouds under sky -> curling vapor -> modern 雲" },
    OracleBoneEntry { modern: '\u{96EA}', // 雪
        description: "Rain with a hand-like form below, sweeping away",
        pictograph: "=**=",
        evolution: "Rain plus broom/hand -> frozen precipitation -> modern 雪" },
    OracleBoneEntry { modern: '\u{5DDD}', // 川
        description: "Three parallel flowing streams",
        pictograph: "|||",
        evolution: "Three streams flowing -> vertical lines -> modern 川" },
    OracleBoneEntry { modern: '\u{6797}', // 林
        description: "Two trees side by side",
        pictograph: "TT",
        evolution: "Two trees together -> doubled 木 -> modern 林" },
    OracleBoneEntry { modern: '\u{68EE}', // 森
        description: "Three trees together, a dense forest",
        pictograph: "TTT",
        evolution: "Three trees clustered -> tripled 木 -> modern 森" },
    OracleBoneEntry { modern: '\u{6CB3}', // 河
        description: "Water flowing beside a speaking mouth (river name)",
        pictograph: "~||",
        evolution: "Water radical + phonetic component -> modern 河" },
    OracleBoneEntry { modern: '\u{6D77}', // 海
        description: "Water with a mother/dark element, vast waters",
        pictograph: "~**",
        evolution: "Water radical + 'every/dark' -> boundless water -> modern 海" },
    OracleBoneEntry { modern: '\u{661F}', // 星
        description: "Three small circles above a phonetic base, stars in sky",
        pictograph: "ooo",
        evolution: "Cluster of stars above -> dots over base -> modern 星" },

    // ===== People / Body =====
    OracleBoneEntry { modern: '\u{4EBA}', // 人
        description: "Side view of a person standing, with visible legs",
        pictograph: "/\\",
        evolution: "Side profile of standing figure -> two strokes -> modern 人" },
    OracleBoneEntry { modern: '\u{5973}', // 女
        description: "Kneeling figure with arms crossed over chest",
        pictograph: "k",
        evolution: "Kneeling woman with crossed arms -> curved strokes -> modern 女" },
    OracleBoneEntry { modern: '\u{5B50}', // 子
        description: "Baby with arms outstretched and a large head",
        pictograph: "o-|-",
        evolution: "Swaddled infant with head -> simplified baby -> modern 子" },
    OracleBoneEntry { modern: '\u{76EE}', // 目
        description: "Eye shape turned sideways with pupil visible",
        pictograph: "(o)",
        evolution: "Realistic eye with pupil -> rectangular eye -> modern 目" },
    OracleBoneEntry { modern: '\u{53E3}', // 口
        description: "Open mouth, a simple opening",
        pictograph: "O",
        evolution: "Open mouth shape -> square -> modern 口" },
    OracleBoneEntry { modern: '\u{8033}', // 耳
        description: "Ear shape with the outer rim and inner whorl",
        pictograph: "@)",
        evolution: "Realistic ear shape -> stylized ear -> modern 耳" },
    OracleBoneEntry { modern: '\u{624B}', // 手
        description: "Hand with five fingers spread out",
        pictograph: "W",
        evolution: "Open hand with five fingers -> three lines with stem -> modern 手" },
    OracleBoneEntry { modern: '\u{8DB3}', // 足
        description: "Foot with toes and ankle",
        pictograph: "L.",
        evolution: "Foot with toes -> simplified foot -> modern 足" },
    OracleBoneEntry { modern: '\u{5FC3}', // 心
        description: "Heart organ shape, the physical heart",
        pictograph: "<3",
        evolution: "Realistic heart organ -> three dots with hook -> modern 心" },
    OracleBoneEntry { modern: '\u{9996}', // 首
        description: "Head with hair on top and an eye",
        pictograph: "o=",
        evolution: "Head with eye and hair -> stylized head -> modern 首" },
    OracleBoneEntry { modern: '\u{9762}', // 面
        description: "Face with an eye in the center, framed by the outline of a head",
        pictograph: "[o]",
        evolution: "Face outline with eye -> enclosed eye form -> modern 面" },
    OracleBoneEntry { modern: '\u{6BDB}', // 毛
        description: "Hair or fur strands growing from skin",
        pictograph: "///",
        evolution: "Hair strands on surface -> angled strokes -> modern 毛" },
    OracleBoneEntry { modern: '\u{9AA8}', // 骨
        description: "Skeleton structure with joints visible",
        pictograph: "X-X",
        evolution: "Skeleton with rib-like structure -> complex form -> modern 骨" },
    OracleBoneEntry { modern: '\u{8840}', // 血
        description: "A vessel containing blood, a sacrificial bowl",
        pictograph: "U.",
        evolution: "Drop of blood in ritual vessel -> dish with drop -> modern 血" },
    OracleBoneEntry { modern: '\u{8EAB}', // 身
        description: "Pregnant woman in profile, body with swollen belly",
        pictograph: "D",
        evolution: "Pregnant figure in profile -> body outline -> modern 身" },
    OracleBoneEntry { modern: '\u{7236}', // 父
        description: "Hand holding a stone axe, symbol of fatherly authority",
        pictograph: "Y-",
        evolution: "Hand with axe/tool -> crossed strokes -> modern 父" },
    OracleBoneEntry { modern: '\u{6BCD}', // 母
        description: "Kneeling woman with two dots for breasts, a nursing mother",
        pictograph: "k..",
        evolution: "Nursing woman -> woman with two dots -> modern 母" },

    // ===== Animals =====
    OracleBoneEntry { modern: '\u{99AC}', // 馬
        description: "Horse with flowing mane, four legs, and tail",
        pictograph: "M~",
        evolution: "Realistic horse with mane and legs -> stylized horse -> modern 馬" },
    OracleBoneEntry { modern: '\u{725B}', // 牛
        description: "Cow or ox head seen from front, with two horns",
        pictograph: "Y",
        evolution: "Bull head with two horns from front -> V-shape with line -> modern 牛" },
    OracleBoneEntry { modern: '\u{7F8A}', // 羊
        description: "Sheep or goat head from front, with two curved horns",
        pictograph: "V",
        evolution: "Sheep head with curved horns -> horns atop cross -> modern 羊" },
    OracleBoneEntry { modern: '\u{72AC}', // 犬
        description: "Dog standing on hind legs, with tail raised",
        pictograph: "/\\~",
        evolution: "Standing dog with raised tail -> person-like with dot -> modern 犬" },
    OracleBoneEntry { modern: '\u{9CE5}', // 鳥
        description: "Bird with long tail feathers, beak, and claws",
        pictograph: ">v",
        evolution: "Detailed bird with tail and crest -> simplified bird -> modern 鳥" },
    OracleBoneEntry { modern: '\u{9B5A}', // 魚
        description: "Fish with head, body, scales, and tail fin",
        pictograph: "<><",
        evolution: "Realistic fish with scales -> stylized fish form -> modern 魚" },
    OracleBoneEntry { modern: '\u{9F8D}', // 龍
        description: "Serpentine creature with horns, open mouth, and coiling body",
        pictograph: "S~^",
        evolution: "Horned serpent with crown -> elaborate coiling form -> modern 龍" },
    OracleBoneEntry { modern: '\u{8C61}', // 象
        description: "Elephant with large trunk, tusks, and body",
        pictograph: "E~",
        evolution: "Elephant with trunk and tusks -> stylized form -> modern 象" },
    OracleBoneEntry { modern: '\u{9E7F}', // 鹿
        description: "Deer with prominent antlers and body",
        pictograph: "YY|",
        evolution: "Deer with branching antlers -> antlers over body -> modern 鹿" },
    OracleBoneEntry { modern: '\u{864E}', // 虎
        description: "Tiger with striped body, open jaws, and claws",
        pictograph: ">=<",
        evolution: "Fierce tiger with open mouth -> stylized tiger -> modern 虎" },
    OracleBoneEntry { modern: '\u{86C7}', // 蛇
        description: "Serpent with coiling body and head",
        pictograph: "~S",
        evolution: "Coiling snake -> insect radical + phonetic -> modern 蛇" },
    OracleBoneEntry { modern: '\u{9F9C}', // 龜
        description: "Turtle seen from above with shell pattern and legs",
        pictograph: "[=]~",
        evolution: "Turtle with shell markings and tail -> complex form -> modern 龜" },
    OracleBoneEntry { modern: '\u{8C6C}', // 豬
        description: "Pig with snout, round body, and short legs",
        pictograph: "o~",
        evolution: "Pig with prominent belly -> pig radical + phonetic -> modern 豬" },

    // ===== Actions =====
    OracleBoneEntry { modern: '\u{5927}', // 大
        description: "Person with arms and legs outstretched wide",
        pictograph: "/|\\",
        evolution: "Person spreading arms wide -> three strokes -> modern 大" },
    OracleBoneEntry { modern: '\u{7ACB}', // 立
        description: "Person standing firmly on the ground line",
        pictograph: "A_",
        evolution: "Person standing on ground -> strokes on base -> modern 立" },
    OracleBoneEntry { modern: '\u{8D70}', // 走
        description: "Person swinging arms while running or walking",
        pictograph: "/\\~",
        evolution: "Running figure with swinging arms -> modern 走" },
    OracleBoneEntry { modern: '\u{898B}', // 見
        description: "Large eye on top of legs, a person looking",
        pictograph: "(o)/\\",
        evolution: "Eye on walking legs -> eye over person -> modern 見" },
    OracleBoneEntry { modern: '\u{8A00}', // 言
        description: "Mouth with sound waves or lines coming out, representing speech",
        pictograph: "O=",
        evolution: "Mouth with speech marks -> tongue/mouth form -> modern 言" },
    OracleBoneEntry { modern: '\u{98DF}', // 食
        description: "Person bending over a food vessel with a lid",
        pictograph: "A_U",
        evolution: "Person at covered food vessel -> lid over container -> modern 食" },
    OracleBoneEntry { modern: '\u{53D6}', // 取
        description: "Hand grasping a severed ear, a war trophy",
        pictograph: "@-W",
        evolution: "Hand seizing ear (trophy of war) -> ear + hand -> modern 取" },
    OracleBoneEntry { modern: '\u{4F86}', // 來
        description: "Wheat plant with grain heads, meaning 'wheat' then 'come'",
        pictograph: "*|*",
        evolution: "Wheat stalk with grain -> borrowed for 'come' -> modern 來" },
    OracleBoneEntry { modern: '\u{53BB}', // 去
        description: "Person departing from a place, leaving an opening",
        pictograph: "/\\O",
        evolution: "Person moving away from opening -> modern 去" },
    OracleBoneEntry { modern: '\u{5165}', // 入
        description: "An arrow or wedge entering downward",
        pictograph: "V",
        evolution: "Downward-pointing wedge -> two strokes -> modern 入" },
    OracleBoneEntry { modern: '\u{51FA}', // 出
        description: "A foot stepping outside a container or boundary",
        pictograph: "|_|^",
        evolution: "Foot stepping out of enclosure -> stacked forms -> modern 出" },
    OracleBoneEntry { modern: '\u{6B62}', // 止
        description: "A footprint, showing toes at the top",
        pictograph: "...|",
        evolution: "Realistic footprint with toes -> three toes on stem -> modern 止" },
    OracleBoneEntry { modern: '\u{884C}', // 行
        description: "A crossroads, intersection of two paths",
        pictograph: "+",
        evolution: "Crossroads intersection -> left-right paths -> modern 行" },
    OracleBoneEntry { modern: '\u{5C04}', // 射
        description: "Hand drawing a bow with an arrow",
        pictograph: ")-->",
        evolution: "Bow with arrow being drawn -> body + inch -> modern 射" },
    OracleBoneEntry { modern: '\u{98DB}', // 飛
        description: "Bird with wings spread in flight",
        pictograph: ">=<",
        evolution: "Bird soaring with spread wings -> ascending form -> modern 飛" },
    OracleBoneEntry { modern: '\u{751F}', // 生
        description: "A plant sprout emerging from the ground",
        pictograph: "+_",
        evolution: "Sprouting seedling from earth -> plant on ground -> modern 生" },
    OracleBoneEntry { modern: '\u{6B7B}', // 死
        description: "A person kneeling beside bones, mourning the dead",
        pictograph: "X/\\",
        evolution: "Person beside remains -> bones + person -> modern 死" },

    // ===== Objects / Concepts =====
    OracleBoneEntry { modern: '\u{738B}', // 王
        description: "An axe head, symbol of royal authority and power",
        pictograph: "T_",
        evolution: "Battle axe (authority symbol) -> three lines connected -> modern 王" },
    OracleBoneEntry { modern: '\u{5929}', // 天
        description: "Person with a large head or emphasis on what is above the person",
        pictograph: "O/\\",
        evolution: "Person with emphasized head (what's above) -> modern 天" },
    OracleBoneEntry { modern: '\u{5730}', // 地
        description: "An altar or earthen mound, the ground itself",
        pictograph: "_+",
        evolution: "Earth mound + phonetic 'also' -> ground/earth -> modern 地" },
    OracleBoneEntry { modern: '\u{9580}', // 門
        description: "Two door panels of a gate seen from front",
        pictograph: "| |",
        evolution: "Two swinging door panels -> doubled door form -> modern 門" },
    OracleBoneEntry { modern: '\u{5200}', // 刀
        description: "Curved blade with a handle",
        pictograph: "J",
        evolution: "Curved knife blade -> simplified blade -> modern 刀" },
    OracleBoneEntry { modern: '\u{5F13}', // 弓
        description: "A drawn bow with string",
        pictograph: ")",
        evolution: "Curved bow with string -> angular bow shape -> modern 弓" },
    OracleBoneEntry { modern: '\u{8ECA}', // 車
        description: "Chariot seen from above showing wheels, axle, and carriage",
        pictograph: "--O--",
        evolution: "Top-down view of chariot with wheels -> structured form -> modern 車" },
    OracleBoneEntry { modern: '\u{821F}', // 舟
        description: "A dugout boat or canoe shape seen from above",
        pictograph: "<=>",
        evolution: "Boat outline from above -> narrow vessel form -> modern 舟" },
    OracleBoneEntry { modern: '\u{8863}', // 衣
        description: "A robe or garment with collar and flowing sides",
        pictograph: "T~",
        evolution: "Robe with collar and lapels -> garment shape -> modern 衣" },
    OracleBoneEntry { modern: '\u{5B9D}', // 寶
        description: "Jade, shells, and pottery under a roof: precious things",
        pictograph: "^$",
        evolution: "Treasures (jade, cowrie, pottery) under roof -> modern 寶" },
    OracleBoneEntry { modern: '\u{5BA4}', // 室
        description: "An arrow reaching a target under a roof, an inner room",
        pictograph: "^->",
        evolution: "Arrow arriving under roof -> destination/room -> modern 室" },
    OracleBoneEntry { modern: '\u{5BB6}', // 家
        description: "A pig under a roof, livestock in the home",
        pictograph: "^~o",
        evolution: "Pig under roof (domestic animal = home) -> modern 家" },
    OracleBoneEntry { modern: '\u{4E95}', // 井
        description: "A well frame seen from above, divided into sections",
        pictograph: "#",
        evolution: "Square well frame from above -> grid pattern -> modern 井" },
    OracleBoneEntry { modern: '\u{9F0E}', // 鼎
        description: "A large ritual bronze cooking vessel with legs and handles",
        pictograph: "U||",
        evolution: "Three-legged bronze cauldron -> stylized vessel -> modern 鼎" },
    OracleBoneEntry { modern: '\u{518A}', // 冊
        description: "Bamboo strips bound together with cord, a book",
        pictograph: "|||=",
        evolution: "Bound bamboo slips -> vertical lines with ties -> modern 冊" },
    OracleBoneEntry { modern: '\u{5E63}', // 幣
        description: "Cloth or silk used as currency offering",
        pictograph: "T$",
        evolution: "Ceremonial cloth/silk -> currency offering -> modern 幣" },

    // ===== Abstract / Spiritual =====
    OracleBoneEntry { modern: '\u{795E}', // 神
        description: "Altar with lightning bolt descending, divine revelation from heaven",
        pictograph: "T*!",
        evolution: "Altar (示) + lightning/spirit -> divine being -> modern 神" },
    OracleBoneEntry { modern: '\u{793A}', // 示
        description: "Altar table with offerings placed on top",
        pictograph: "T.",
        evolution: "Stone altar with offerings -> T-shaped table -> modern 示" },
    OracleBoneEntry { modern: '\u{796D}', // 祭
        description: "Hand placing dripping meat on an altar",
        pictograph: "W.T",
        evolution: "Hand offering meat on altar -> sacrifice ritual -> modern 祭" },
    OracleBoneEntry { modern: '\u{4E0A}', // 上
        description: "A short stroke above a longer reference line",
        pictograph: "._",
        evolution: "Mark above baseline -> dot/stroke above line -> modern 上" },
    OracleBoneEntry { modern: '\u{4E0B}', // 下
        description: "A short stroke below a longer reference line",
        pictograph: "_.",
        evolution: "Mark below baseline -> dot/stroke below line -> modern 下" },
    OracleBoneEntry { modern: '\u{4E2D}', // 中
        description: "A banner or flag on a pole, marking the center of a camp",
        pictograph: "d|",
        evolution: "Flag on pole (center marker) -> vertical through box -> modern 中" },
    OracleBoneEntry { modern: '\u{5584}', // 善
        description: "A sheep (auspicious) above a pair of speech marks, speaking well",
        pictograph: "V==",
        evolution: "Sheep (auspicious) + speech -> goodness -> modern 善" },
    OracleBoneEntry { modern: '\u{60E1}', // 惡
        description: "A crouching figure with a heart distorted, wickedness",
        pictograph: "X<3",
        evolution: "Distorted heart/intention -> ugliness, evil -> modern 惡" },
    OracleBoneEntry { modern: '\u{5929}', // (already entered above as 天)
        description: "Person with a large head or emphasis on what is above the person",
        pictograph: "O/\\",
        evolution: "Person with emphasized head (what's above) -> modern 天" },
    OracleBoneEntry { modern: '\u{9748}', // 靈
        description: "Shaman performing rain ritual, drops falling while dancing",
        pictograph: "=,,=k",
        evolution: "Rain + shaman dancing -> spiritual power -> modern 靈" },
    OracleBoneEntry { modern: '\u{798F}', // 福
        description: "Hands presenting a wine vessel to an altar, heavenly blessing",
        pictograph: "T|U",
        evolution: "Altar (示) + full wine vessel -> blessing/fortune -> modern 福" },
    OracleBoneEntry { modern: '\u{7985}', // 禅 (禪)
        description: "Altar with spirits of the earth, ground-level worship",
        pictograph: "T__",
        evolution: "Altar + level ground -> earth sacrifice / meditation -> modern 禪" },
    OracleBoneEntry { modern: '\u{7948}', // 祈
        description: "Altar with an axe, cutting ritual to petition the gods",
        pictograph: "T-J",
        evolution: "Altar + axe (ritual cutting) -> prayer/petition -> modern 祈" },

    // ===== Numbers =====
    OracleBoneEntry { modern: '\u{4E00}', // 一
        description: "A single horizontal stroke, one",
        pictograph: "-",
        evolution: "Single stroke -> horizontal line -> modern 一" },
    OracleBoneEntry { modern: '\u{4E8C}', // 二
        description: "Two horizontal strokes, two",
        pictograph: "=",
        evolution: "Two parallel strokes -> double line -> modern 二" },
    OracleBoneEntry { modern: '\u{4E09}', // 三
        description: "Three horizontal strokes, three",
        pictograph: "===",
        evolution: "Three parallel strokes -> triple line -> modern 三" },
    OracleBoneEntry { modern: '\u{56DB}', // 四
        description: "Four horizontal lines inside a rectangle (later modified)",
        pictograph: "||||",
        evolution: "Four lines -> enclosed form -> modern 四" },
    OracleBoneEntry { modern: '\u{4E94}', // 五
        description: "Two lines crossed by horizontal bars, five",
        pictograph: "X-",
        evolution: "Crossed lines between bars -> modern 五" },
    OracleBoneEntry { modern: '\u{516D}', // 六
        description: "A shelter or roof shape, later associated with six",
        pictograph: "^.",
        evolution: "Roof-like shape -> abstract number -> modern 六" },
    OracleBoneEntry { modern: '\u{4E03}', // 七
        description: "A cross or cutting mark, originally meaning 'to cut'",
        pictograph: "+",
        evolution: "Cross/cut mark -> borrowed for seven -> modern 七" },
    OracleBoneEntry { modern: '\u{516B}', // 八
        description: "Two strokes diverging, meaning division or splitting",
        pictograph: "\\/",
        evolution: "Two lines splitting apart -> diverging strokes -> modern 八" },
    OracleBoneEntry { modern: '\u{4E5D}', // 九
        description: "An arm bent at the elbow, a hook shape",
        pictograph: "J",
        evolution: "Bent arm/elbow -> hook shape -> modern 九" },
    OracleBoneEntry { modern: '\u{5341}', // 十
        description: "A vertical line through a knot or tally mark for ten",
        pictograph: "|",
        evolution: "Knotted tally rope for ten -> cross shape -> modern 十" },
    OracleBoneEntry { modern: '\u{767E}', // 百
        description: "A single seed or grain above one, the start of many",
        pictograph: "o-",
        evolution: "One/single marker with emphasis -> 'all/hundred' -> modern 百" },
    OracleBoneEntry { modern: '\u{5343}', // 千
        description: "A person figure with a mark, counting a great number",
        pictograph: "/\\-",
        evolution: "Person with tally mark -> thousand -> modern 千" },
    OracleBoneEntry { modern: '\u{842C}', // 萬
        description: "A scorpion, later borrowed for the number ten thousand",
        pictograph: "m~",
        evolution: "Scorpion pictograph -> borrowed for 'myriad/ten thousand' -> modern 萬" },

    // ===== Kinship / Roles =====
    OracleBoneEntry { modern: '\u{81E3}', // 臣
        description: "An eye looking downward, a subject averting gaze before the king",
        pictograph: "(o_)",
        evolution: "Downcast eye (submission) -> vertical eye form -> modern 臣" },
    OracleBoneEntry { modern: '\u{540E}', // 后 (後)
        description: "A woman of authority, or a person facing backward",
        pictograph: "/\\<",
        evolution: "Person of authority / facing back -> queen/after -> modern 后" },
    OracleBoneEntry { modern: '\u{5E2B}', // 師
        description: "A mound or hill with people gathered, a military encampment",
        pictograph: "_/\\A",
        evolution: "Military garrison on mound -> army/teacher -> modern 師" },
    OracleBoneEntry { modern: '\u{6C11}', // 民
        description: "An eye being pierced, originally a blinded slave or subject",
        pictograph: "(o)|",
        evolution: "Eye being pierced (slave) -> common people -> modern 民" },
    OracleBoneEntry { modern: '\u{5144}', // 兄
        description: "A person with a large open mouth, the elder who speaks with authority",
        pictograph: "O/\\",
        evolution: "Person with prominent mouth -> elder brother -> modern 兄" },
    OracleBoneEntry { modern: '\u{5F1F}', // 弟
        description: "Thread wound around a spool, ordered sequence (younger in order)",
        pictograph: "Y-Y",
        evolution: "Thread on spool (sequence/order) -> younger brother -> modern 弟" },
    OracleBoneEntry { modern: '\u{592B}', // 夫
        description: "A man with a pin through his hair, an adult male",
        pictograph: "/\\-",
        evolution: "Man with hairpin (adulthood marker) -> modern 夫" },
    OracleBoneEntry { modern: '\u{59BB}', // 妻
        description: "A woman with a hand seizing her hair, taking a wife",
        pictograph: "W-k",
        evolution: "Hand grasping woman's hair (marriage by capture) -> modern 妻" },

    // ===== Spiritual / Biblical vocabulary =====
    OracleBoneEntry { modern: '\u{5929}', // 天 (duplicate - will be filtered by lookup)
        description: "Person with a large head or emphasis on what is above the person",
        pictograph: "O/\\",
        evolution: "Person with emphasized head (what's above) -> modern 天" },
    OracleBoneEntry { modern: '\u{9053}', // 道
        description: "A head/face on a path, the way one walks while thinking",
        pictograph: "o-->",
        evolution: "Head on road/path -> way/path/doctrine -> modern 道" },
    OracleBoneEntry { modern: '\u{5FB7}', // 德
        description: "An eye looking straight ahead on a path, walking uprightly",
        pictograph: "(o)->",
        evolution: "Straight gaze on the road -> virtue/morality -> modern 德" },
    OracleBoneEntry { modern: '\u{4EC1}', // 仁
        description: "A person beside the number two, the relationship between people",
        pictograph: "/\\=",
        evolution: "Person + two (relationship) -> benevolence -> modern 仁" },
    OracleBoneEntry { modern: '\u{7FA9}', // 義
        description: "A sheep (auspicious) above a weapon, righteous sacrifice",
        pictograph: "V-J",
        evolution: "Sheep + weapon (ritual sacrifice) -> righteousness -> modern 義" },
    OracleBoneEntry { modern: '\u{4FE1}', // 信
        description: "A person standing next to the word radical, keeping one's word",
        pictograph: "/\\O=",
        evolution: "Person + speech -> trustworthiness/faith -> modern 信" },
    OracleBoneEntry { modern: '\u{611B}', // 愛
        description: "A person with a heart in the center, reaching out with hands and feet",
        pictograph: "W<3L",
        evolution: "Hands + heart + walking -> love (giving with one's whole being) -> modern 愛" },
    OracleBoneEntry { modern: '\u{5149}', // 光
        description: "Fire or torch on top of a person, light shining from above",
        pictograph: "/\\*/\\",
        evolution: "Fire above person (carrying torch) -> light/brilliance -> modern 光" },
    OracleBoneEntry { modern: '\u{6C38}', // 永
        description: "A person swimming in a long river current, enduring flow",
        pictograph: "~~/\\",
        evolution: "Person in flowing water -> eternal/forever -> modern 永" },
    OracleBoneEntry { modern: '\u{547D}', // 命
        description: "A mouth issuing orders to a kneeling person, a decree",
        pictograph: "O->k",
        evolution: "Mouth giving orders to person -> fate/command -> modern 命" },
    OracleBoneEntry { modern: '\u{5723}', // 聖 (simplified 圣)
        description: "An ear and a mouth: one who hears keenly and speaks wisely",
        pictograph: "@)O",
        evolution: "Large ear + mouth + person -> sage/holy -> modern 聖" },
    OracleBoneEntry { modern: '\u{8056}', // 聖 (traditional)
        description: "An ear and a mouth with a king: one who hears and speaks with authority",
        pictograph: "@)O_T",
        evolution: "Ear + mouth + king -> sage/holy -> modern 聖" },
    OracleBoneEntry { modern: '\u{5E73}', // 平
        description: "Balanced water surface, level and even",
        pictograph: "~-~",
        evolution: "Level water surface -> flat/peaceful -> modern 平" },
    OracleBoneEntry { modern: '\u{6B63}', // 正
        description: "A foot marching toward a target line, straight and correct",
        pictograph: "...-",
        evolution: "Foot heading to goal line -> correct/straight -> modern 正" },
    OracleBoneEntry { modern: '\u{5168}', // 全
        description: "Jade (a complete, unbroken gem) under a cover, wholeness",
        pictograph: "^O",
        evolution: "Roof/cover + jade (perfection) -> complete/whole -> modern 全" },
    OracleBoneEntry { modern: '\u{5929}', // duplicate 天
        description: "Person with a large head or emphasis on what is above the person",
        pictograph: "O/\\",
        evolution: "Person with emphasized head -> sky/heaven -> modern 天" },

    // ===== More Biblical / Common Characters =====
    OracleBoneEntry { modern: '\u{5730}', // duplicate 地, but different context
        description: "Earth mound with a snake-like form, the ground",
        pictograph: "_+~",
        evolution: "Earth altar + ground form -> earth/ground -> modern 地" },
    OracleBoneEntry { modern: '\u{4E16}', // 世
        description: "Three tens linked together, three decades, a generation",
        pictograph: "+-+-+",
        evolution: "Three connected tens -> thirty years -> generation/world -> modern 世" },
    OracleBoneEntry { modern: '\u{5E74}', // 年
        description: "A person carrying grain/harvest, the completion of a harvest cycle",
        pictograph: "/\\*",
        evolution: "Person bearing grain harvest -> year/harvest cycle -> modern 年" },
    OracleBoneEntry { modern: '\u{6642}', // 時
        description: "Sun alongside a foot moving forward, the sun's progress = time",
        pictograph: "(O)...",
        evolution: "Sun + advancing foot -> time/hour -> modern 時" },
    OracleBoneEntry { modern: '\u{65E5}', // duplicate 日
        description: "Circle with a dot in the center, depicting the sun",
        pictograph: "(O)",
        evolution: "Picture of sun with center mark -> square with line -> modern 日" },
    OracleBoneEntry { modern: '\u{591C}', // 夜
        description: "Person under the moon with a dot, the night time",
        pictograph: "/\\C)",
        evolution: "Person under crescent moon -> nighttime -> modern 夜" },
    OracleBoneEntry { modern: '\u{660E}', // 明
        description: "Sun and moon together, brightness and clarity",
        pictograph: "(O)C)",
        evolution: "Sun + moon -> brightness/understanding -> modern 明" },
    OracleBoneEntry { modern: '\u{767D}', // 白
        description: "The sun just emerging, a sliver of light, whiteness",
        pictograph: "(O>",
        evolution: "Sun beginning to shine -> white/bright -> modern 白" },
    OracleBoneEntry { modern: '\u{9ED1}', // 黑
        description: "Fire beneath a chimney or window, soot and darkness",
        pictograph: "[/\\,]",
        evolution: "Soot from fire in a vent -> black/dark -> modern 黑" },
    OracleBoneEntry { modern: '\u{8D64}', // 赤
        description: "Fire above a person, the color of glowing flames, red",
        pictograph: "/\\,/\\",
        evolution: "Person + fire above -> red/bare -> modern 赤" },
    OracleBoneEntry { modern: '\u{9752}', // 青
        description: "A sprouting plant above a well or cinnabar deposit, blue-green",
        pictograph: "+#",
        evolution: "Plant growth + mineral source -> blue-green color -> modern 青" },

    // ===== Containers and Structures =====
    OracleBoneEntry { modern: '\u{56FD}', // 國 (simplified 国)
        description: "A walled territory with a weapon inside, a defended domain",
        pictograph: "[]-J",
        evolution: "Weapon inside walls -> defended territory -> modern 國" },
    OracleBoneEntry { modern: '\u{57CE}', // 城
        description: "Earth wall with a weapon, a fortified city",
        pictograph: "_+-J",
        evolution: "Earth rampart + weapon (defense) -> city wall -> modern 城" },
    OracleBoneEntry { modern: '\u{5BAE}', // 宮
        description: "Multiple rooms under a roof, a palace or temple",
        pictograph: "^[][]",
        evolution: "Roof over connected rooms -> palace/temple -> modern 宮" },
    OracleBoneEntry { modern: '\u{5EAB}', // 庫
        description: "A building sheltering a chariot, an armory or storehouse",
        pictograph: "^--O--",
        evolution: "Shelter with chariot inside -> storehouse -> modern 庫" },

    // ===== Weather and Time =====
    OracleBoneEntry { modern: '\u{96F7}', // 雷
        description: "Rain above rolling drums in a field, thunder",
        pictograph: "=,,=[#]",
        evolution: "Rain + drum sounds in field -> thunder -> modern 雷" },
    OracleBoneEntry { modern: '\u{96FB}', // 電
        description: "Rain with a lightning bolt zigzagging down",
        pictograph: "=,,=Z",
        evolution: "Rain + zigzag bolt -> lightning/electricity -> modern 電" },
    OracleBoneEntry { modern: '\u{6625}', // 春
        description: "Plants sprouting under the sun, the season of growth",
        pictograph: "+++(O)",
        evolution: "Sprouting plants + sun -> spring season -> modern 春" },
    OracleBoneEntry { modern: '\u{79CB}', // 秋
        description: "Grain with fire, harvesting and burning fields in autumn",
        pictograph: "*~/\\,",
        evolution: "Grain + fire (burning stubble) -> autumn -> modern 秋" },

    // ===== Warfare and Government =====
    OracleBoneEntry { modern: '\u{6208}', // 戈
        description: "A dagger-axe, a pole weapon with a blade",
        pictograph: "J-|",
        evolution: "Pole weapon with blade -> simplified weapon -> modern 戈" },
    OracleBoneEntry { modern: '\u{76FE}', // 盾
        description: "A shield with an eye peeking over it",
        pictograph: "[(o)]",
        evolution: "Shield with eye looking over -> shield -> modern 盾" },
    OracleBoneEntry { modern: '\u{5175}', // 兵
        description: "Two hands holding a weapon, a soldier",
        pictograph: "WW-J",
        evolution: "Hands grasping axe/weapon -> soldier/weapon -> modern 兵" },
    OracleBoneEntry { modern: '\u{5C06}', // 將
        description: "A hand offering meat on a table, a general making offerings before battle",
        pictograph: "W.T_",
        evolution: "Hand + meat + altar -> general/about to -> modern 將" },

    // ===== Food and Agriculture =====
    OracleBoneEntry { modern: '\u{7C73}', // 米
        description: "Grains of rice scattered on a surface, a cross with dots",
        pictograph: ".*.",
        evolution: "Scattered rice grains -> dotted cross -> modern 米" },
    OracleBoneEntry { modern: '\u{9EA5}', // 麥
        description: "Wheat plant with roots and a foot (coming/arriving grain)",
        pictograph: "*|L",
        evolution: "Wheat stalk + foot (arriving crop) -> wheat -> modern 麥" },
    OracleBoneEntry { modern: '\u{9152}', // 酒
        description: "Water beside a wine vessel, fermented drink",
        pictograph: "~U.",
        evolution: "Water + wine jar with ferment -> wine/alcohol -> modern 酒" },
    OracleBoneEntry { modern: '\u{8089}', // 肉
        description: "A strip of dried meat with visible fibers",
        pictograph: "===|",
        evolution: "Strips of dried meat -> layered form -> modern 肉" },

    // ===== Miscellaneous Common =====
    OracleBoneEntry { modern: '\u{5B57}', // 字
        description: "A child under a roof, a character 'born' in a house of learning",
        pictograph: "^o-|-",
        evolution: "Child under roof -> give birth to / written character -> modern 字" },
    OracleBoneEntry { modern: '\u{66F8}', // 書
        description: "A hand holding a brush writing on a tablet",
        pictograph: "W|=",
        evolution: "Hand with brush on writing surface -> book/writing -> modern 書" },
    OracleBoneEntry { modern: '\u{5B66}', // 學
        description: "Two hands above a child under a roof, teaching",
        pictograph: "WW^o-|-",
        evolution: "Hands guiding child under roof -> study/learn -> modern 學" },
    OracleBoneEntry { modern: '\u{529B}', // 力
        description: "A plow or sinew, the tool that requires strength",
        pictograph: "J_",
        evolution: "Plow shape / flexed tendon -> strength/power -> modern 力" },
    OracleBoneEntry { modern: '\u{7537}', // 男
        description: "A field with a plow, the one who plows the field",
        pictograph: "[#]J_",
        evolution: "Field + plow/strength -> man/male -> modern 男" },
    OracleBoneEntry { modern: '\u{53CB}', // 友
        description: "Two right hands clasped together, friendship",
        pictograph: "WW",
        evolution: "Two hands reaching together -> friend -> modern 友" },
    OracleBoneEntry { modern: '\u{6587}', // 文
        description: "A person with a tattoo or pattern on the chest",
        pictograph: "/\\X",
        evolution: "Person with chest markings -> pattern/writing/culture -> modern 文" },
    OracleBoneEntry { modern: '\u{6B66}', // 武
        description: "A foot carrying a weapon (dagger-axe), marching to war",
        pictograph: "...J-|",
        evolution: "Foot + dagger-axe (marching with weapon) -> martial -> modern 武" },
    // ===== Common Chinese Bible Characters =====
    OracleBoneEntry { modern: '\u{7684}', // 的
        description: "A bright/white form — a clear target or aim",
        pictograph: "🎯",
        evolution: "Arrow hitting target (clear/bright) → possessive particle → modern 的" },
    OracleBoneEntry { modern: '\u{4ED6}', // 他
        description: "Person standing beside another",
        pictograph: "👤",
        evolution: "Person radical + 'other' → he/him → modern 他" },
    OracleBoneEntry { modern: '\u{6211}', // 我
        description: "Hand gripping a halberd — 'I' as one who fights",
        pictograph: "⚔️",
        evolution: "Hand holding halberd (weapon/self-defense) → first person → modern 我" },
    OracleBoneEntry { modern: '\u{4F60}', // 你
        description: "Person standing respectfully before another",
        pictograph: "🙋",
        evolution: "Person radical + phonetic → you (respectful) → modern 你" },
    OracleBoneEntry { modern: '\u{5728}', // 在
        description: "Sprout growing from the ground — being present",
        pictograph: "🌱",
        evolution: "Seedling in earth (existing/present) → at/in → modern 在" },
    OracleBoneEntry { modern: '\u{8AAA}', // 說
        description: "Mouth speaking joyfully — words flowing",
        pictograph: "💬",
        evolution: "Speech radical + 'pleased' → to speak/say → modern 說" },
    OracleBoneEntry { modern: '\u{662F}', // 是
        description: "Sun overhead at noon — upright, correct",
        pictograph: "☀️",
        evolution: "Sun at zenith (correct/straight) → to be/is → modern 是" },
    OracleBoneEntry { modern: '\u{6709}', // 有
        description: "Hand holding a piece of meat — to possess",
        pictograph: "🤲",
        evolution: "Hand grasping meat (possession) → to have → modern 有" },
    OracleBoneEntry { modern: '\u{4E0D}', // 不
        description: "Root or calyx of a flower — refusing to bloom",
        pictograph: "✕",
        evolution: "Flower calyx pointing down (refusal) → negation → modern 不" },
    OracleBoneEntry { modern: '\u{548C}', // 和
        description: "Grain stalks and a mouth — harmony through sharing food",
        pictograph: "🌾",
        evolution: "Grain + mouth (eating together) → peace/harmony → modern 和" },
    OracleBoneEntry { modern: '\u{4E86}', // 了
        description: "A child with arms but no legs yet — something just finished",
        pictograph: "✓",
        evolution: "Wrapped infant (completion) → aspect particle → modern 了" },
    OracleBoneEntry { modern: '\u{5C31}', // 就
        description: "A person arriving at a high capital city",
        pictograph: "🏛️",
        evolution: "Capital city + person approaching → to go to/then → modern 就" },
    OracleBoneEntry { modern: '\u{4EE5}', // 以
        description: "Person carrying or using a tool",
        pictograph: "🔧",
        evolution: "Person with tool → to use/by means of → modern 以" },
    OracleBoneEntry { modern: '\u{4F86}', // 來
        description: "Wheat plant — grain that 'comes' from heaven",
        pictograph: "🌾",
        evolution: "Wheat stalk (arriving grain) → to come → modern 來" },
    OracleBoneEntry { modern: '\u{9019}', // 這
        description: "Foot approaching something nearby",
        pictograph: "👆",
        evolution: "Walking radical + meet → this/these → modern 這" },
    OracleBoneEntry { modern: '\u{90A3}', // 那
        description: "A distant walled settlement",
        pictograph: "🏘️",
        evolution: "City/place radical → that/there → modern 那" },
    OracleBoneEntry { modern: '\u{7232}', // 為
        description: "Hand guiding an elephant — to do/act on behalf of",
        pictograph: "🐘",
        evolution: "Hand leading elephant (doing/making) → to do/for → modern 為" },
    OracleBoneEntry { modern: '\u{7D66}', // 給
        description: "Threads joined together — to give/supply",
        pictograph: "🧵",
        evolution: "Silk threads joined → to give → modern 給" },
    OracleBoneEntry { modern: '\u{8981}', // 要
        description: "Hands at waist — grasping what is wanted",
        pictograph: "🤲",
        evolution: "Hands at waist (wanting/needing) → to want/need → modern 要" },
    OracleBoneEntry { modern: '\u{5230}', // 到
        description: "Reaching a destination with precision",
        pictograph: "📍",
        evolution: "Reaching (arrival) + knife (precision) → to arrive → modern 到" },
    OracleBoneEntry { modern: '\u{5C0D}', // 對
        description: "Carefully measuring — facing something exactly",
        pictograph: "⚖️",
        evolution: "Careful measurement → correct/facing → modern 對" },
    OracleBoneEntry { modern: '\u{53C8}', // 又
        description: "Right hand reaching — doing something again",
        pictograph: "✋",
        evolution: "Right hand (repeating action) → again/also → modern 又" },
    OracleBoneEntry { modern: '\u{5F9E}', // 從
        description: "Two people walking — one following the other",
        pictograph: "👥",
        evolution: "Two persons (following) → from/to follow → modern 從" },
    OracleBoneEntry { modern: '\u{500B}', // 個
        description: "A single person standing — one unit",
        pictograph: "1️⃣",
        evolution: "Person + bamboo (counting stick) → measure word → modern 個" },
    OracleBoneEntry { modern: '\u{5404}', // 各
        description: "A foot arriving at a doorway — each visitor",
        pictograph: "🚶",
        evolution: "Foot entering dwelling → each/every → modern 各" },
    OracleBoneEntry { modern: '\u{6240}', // 所
        description: "Axe work at a doorway — a place being built",
        pictograph: "🏗️",
        evolution: "Door + axe (building place) → place/that which → modern 所" },
    OracleBoneEntry { modern: '\u{628A}', // 把
        description: "Hand gripping something firmly",
        pictograph: "✊",
        evolution: "Hand grasping (holding) → to hold/handle → modern 把" },
    OracleBoneEntry { modern: '\u{90FD}', // 都
        description: "A great capital where all gather",
        pictograph: "🏛️",
        evolution: "Walled capital (gathering place) → all/capital → modern 都" },
    OracleBoneEntry { modern: '\u{7D04}', // 約
        description: "A thread binding — making a covenant/agreement",
        pictograph: "🤝",
        evolution: "Thread binding (binding agreement) → covenant/about → modern 約" },
    OracleBoneEntry { modern: '\u{751F}', // 生
        description: "A plant sprouting from the ground — new life",
        pictograph: "🌱",
        evolution: "Sprout growing from earth → life/birth → modern 生" },
];

/// Look up Oracle Bone Script information for a given modern Chinese character.
/// Returns the first matching entry from the table.
pub fn lookup_oracle_bone(ch: char) -> Option<&'static OracleBoneInfo> {
    // We use a simple linear scan. The table is small enough that this is fine.
    // We store results as OracleBoneInfo references via a static conversion.
    // Since OracleBoneEntry and OracleBoneInfo have the same field layout,
    // we rebuild OracleBoneInfo on the fly.
    //
    // However, to return &'static OracleBoneInfo we need static storage.
    // Instead, we'll use a different approach: store OracleBoneInfo directly.
    //
    // Actually, let's just return Option<OracleBoneInfo> (owned).
    // But the signature says &'static, so let's use a lazy static map.
    ORACLE_BONE_MAP.get(&ch)
}

use std::collections::HashMap;
use std::sync::LazyLock;

static ORACLE_BONE_MAP: LazyLock<HashMap<char, OracleBoneInfo>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for entry in ORACLE_BONE_TABLE {
        // Only insert the first occurrence of each character
        map.entry(entry.modern).or_insert_with(|| OracleBoneInfo {
            modern: entry.modern,
            description: entry.description,
            pictograph: entry.pictograph,
            evolution: entry.evolution,
        });
    }
    map
});

/// Return the number of Oracle Bone Script entries available.
pub fn oracle_bone_count() -> usize {
    ORACLE_BONE_MAP.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_sun() {
        let info = lookup_oracle_bone('\u{65E5}').unwrap(); // 日
        assert_eq!(info.modern, '日');
        assert!(info.description.contains("sun"));
        assert!(!info.pictograph.is_empty());
        assert!(info.evolution.contains("modern"));
    }

    #[test]
    fn test_lookup_moon() {
        let info = lookup_oracle_bone('\u{6708}').unwrap(); // 月
        assert_eq!(info.modern, '月');
        assert!(info.description.contains("moon") || info.description.contains("crescent"));
    }

    #[test]
    fn test_lookup_person() {
        let info = lookup_oracle_bone('\u{4EBA}').unwrap(); // 人
        assert_eq!(info.modern, '人');
        assert!(info.description.contains("person") || info.description.contains("standing"));
    }

    #[test]
    fn test_lookup_heart() {
        let info = lookup_oracle_bone('\u{5FC3}').unwrap(); // 心
        assert_eq!(info.modern, '心');
        assert!(info.description.contains("heart"));
    }

    #[test]
    fn test_lookup_god() {
        let info = lookup_oracle_bone('\u{795E}').unwrap(); // 神
        assert_eq!(info.modern, '神');
        assert!(info.description.contains("altar") || info.description.contains("divine"));
    }

    #[test]
    fn test_lookup_king() {
        let info = lookup_oracle_bone('\u{738B}').unwrap(); // 王
        assert_eq!(info.modern, '王');
        assert!(info.description.contains("axe") || info.description.contains("authority"));
    }

    #[test]
    fn test_lookup_horse() {
        let info = lookup_oracle_bone('\u{99AC}').unwrap(); // 馬
        assert_eq!(info.modern, '馬');
        assert!(info.description.contains("horse") || info.description.contains("mane"));
    }

    #[test]
    fn test_lookup_dragon() {
        let info = lookup_oracle_bone('\u{9F8D}').unwrap(); // 龍
        assert_eq!(info.modern, '龍');
        assert!(info.description.contains("serpent") || info.description.contains("horn"));
    }

    #[test]
    fn test_lookup_nonexistent() {
        // Latin letter should not be in the table
        assert!(lookup_oracle_bone('A').is_none());
    }

    #[test]
    fn test_lookup_returns_first_for_duplicates() {
        // 天 appears multiple times in the table; we should get a consistent result
        let info = lookup_oracle_bone('\u{5929}').unwrap(); // 天
        assert_eq!(info.modern, '天');
    }

    #[test]
    fn test_count_at_least_150_unique() {
        // The table has duplicates for some characters; the map deduplicates.
        // We should have at least 150 unique characters.
        let count = oracle_bone_count();
        assert!(
            count >= 150,
            "Expected at least 150 unique Oracle Bone entries, got {}",
            count
        );
    }

    #[test]
    fn test_all_entries_have_nonempty_fields() {
        for (_, info) in ORACLE_BONE_MAP.iter() {
            assert!(!info.description.is_empty(), "Empty description for {}", info.modern);
            assert!(!info.pictograph.is_empty(), "Empty pictograph for {}", info.modern);
            assert!(!info.evolution.is_empty(), "Empty evolution for {}", info.modern);
        }
    }

    #[test]
    fn test_nature_characters_present() {
        let nature_chars = ['日', '月', '水', '火', '山', '木', '田', '石', '雨', '土'];
        for ch in &nature_chars {
            assert!(
                lookup_oracle_bone(*ch).is_some(),
                "Missing nature character: {}",
                ch
            );
        }
    }

    #[test]
    fn test_body_characters_present() {
        let body_chars = ['人', '女', '子', '目', '口', '耳', '手', '足', '心'];
        for ch in &body_chars {
            assert!(
                lookup_oracle_bone(*ch).is_some(),
                "Missing body character: {}",
                ch
            );
        }
    }

    #[test]
    fn test_animal_characters_present() {
        let animal_chars = ['馬', '牛', '羊', '犬', '鳥', '魚', '龍'];
        for ch in &animal_chars {
            assert!(
                lookup_oracle_bone(*ch).is_some(),
                "Missing animal character: {}",
                ch
            );
        }
    }

    #[test]
    fn test_number_characters_present() {
        let number_chars = ['一', '二', '三', '四', '五', '六', '七', '八', '九', '十'];
        for ch in &number_chars {
            assert!(
                lookup_oracle_bone(*ch).is_some(),
                "Missing number character: {}",
                ch
            );
        }
    }

    #[test]
    fn test_serialization() {
        let info = lookup_oracle_bone('日').unwrap();
        let json = serde_json::to_string(info).unwrap();
        assert!(json.contains("\"modern\""));
        assert!(json.contains("\"description\""));
        assert!(json.contains("\"pictograph\""));
        assert!(json.contains("\"evolution\""));
    }
}
