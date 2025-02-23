/*
 Navicat Premium Data Transfer

 Source Server         : localhost_3306
 Source Server Type    : MySQL
 Source Server Version : 80031
 Source Host           : localhost:3306
 Source Schema         : rust-blogs

 Target Server Type    : MySQL
 Target Server Version : 80031
 File Encoding         : 65001

 Date: 23/02/2025 17:46:49
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for article_categories
-- ----------------------------
DROP TABLE IF EXISTS `article_categories`;
CREATE TABLE `article_categories`  (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '分类ID',
  `name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '分类名称',
  `icon` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '分类图标路径',
  `count` int NULL DEFAULT 0 COMMENT '文章数量统计',
  `created_time` datetime NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `name`(`name` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 44 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of article_categories
-- ----------------------------
INSERT INTO `article_categories` VALUES (9, 'GPT', '', 0, '2025-02-21 17:05:52');
INSERT INTO `article_categories` VALUES (10, 'JavaScript', '', 0, '2025-02-21 17:05:52');
INSERT INTO `article_categories` VALUES (12, 'Vue', '', 0, '2025-02-21 17:05:52');
INSERT INTO `article_categories` VALUES (35, 'Nuxt', '', 0, '2025-02-21 17:05:52');
INSERT INTO `article_categories` VALUES (36, '浏览器', '', 0, '2025-02-21 17:05:52');
INSERT INTO `article_categories` VALUES (40, 'MongoDB', '', 0, '2025-02-21 17:05:52');
INSERT INTO `article_categories` VALUES (42, 'Node.js', '', 0, '2025-02-21 17:05:52');

-- ----------------------------
-- Table structure for articles
-- ----------------------------
DROP TABLE IF EXISTS `articles`;
CREATE TABLE `articles`  (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '文章ID',
  `category_id` int NOT NULL COMMENT '分类ID',
  `title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '文章标题',
  `html_content` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'HTML内容',
  `home_img` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '封面图路径',
  `brief` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '文章简介',
  `view_count` int NULL DEFAULT 0 COMMENT '浏览量',
  `is_visible` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否可见（0=不可见 1=可见）',
  `is_top` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否置顶（0=不置顶 1=置顶）',
  `created_time` datetime NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_time` datetime NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `idx_category`(`category_id` ASC) USING BTREE,
  INDEX `idx_created`(`created_time` ASC) USING BTREE,
  INDEX `idx_title`(`title`(50) ASC) USING BTREE,
  INDEX `idx_visible`(`is_visible` ASC) USING BTREE,
  INDEX `idx_top`(`is_top` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 454 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of articles
-- ----------------------------
INSERT INTO `articles` VALUES (453, 10, '《道德经》全文', '<p><br></p><p style=\"text-align: start;\">【第一章】道可道，非常道；名可名，非常名。无名天地之始，有名万物之母。故常无欲，以观其妙；常</p><p style=\"text-align: start;\">有欲，以观其徼（jiào）。此两者同出而异名，同谓之玄，玄之又玄，众妙之门。〖<a href=\"https://www.daodejing.org/1.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二章】天下皆知美之为美，斯恶（è）已；皆知善之为善，斯不善已。故有无相生，难易相成，长短相</p><p style=\"text-align: start;\">较，高下相倾，音声相和（hè），前后相随。是以圣人处无为之事，行不言之教，万物作焉而不辞，生而</p><p style=\"text-align: start;\">不有，为而不恃，功成而弗居。夫（fú）唯弗居，是以不去。 〖<a href=\"https://www.daodejing.org/2.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三章】不尚贤，使民不争；不贵难得之货，使民不为盗；不见（xiàn）可欲，使民心不乱。是以圣人</p><p style=\"text-align: start;\">之治，虚其心，实其腹；弱其志，强其骨。常使民无知无欲，使夫（fú）智者不敢为也。为无为，则无不</p><p style=\"text-align: start;\">治。 〖<a href=\"https://www.daodejing.org/3.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四章】道冲而用之或不盈，渊兮似万物之宗。挫其锐，解其纷，和其光，同其尘。湛兮似或存，吾不</p><p style=\"text-align: start;\">知谁之子，象帝之先。 〖<a href=\"https://www.daodejing.org/4.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五章】天地不仁，以万物为刍（chú）狗；圣人不仁，以百姓为刍狗。天地之间，其犹橐龠（tuó</p><p style=\"text-align: start;\">yuè）乎？虚而不屈，动而愈出。多言数（shuò）穷，不如守中。 〖<a href=\"https://www.daodejing.org/5.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六章】谷神不死，是谓玄牝（pìn），玄牝之门，是谓天地根。绵绵若存，用之不勤。 〖<a href=\"https://www.daodejing.org/6.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七章】天长地久。天地所以能长且久者，以其不自生，故能长生。是以圣人后其身而身先，外其身而</p><p style=\"text-align: start;\">身存。非以其无私邪（yé）？故能成其私。 〖<a href=\"https://www.daodejing.org/7.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第八章】上善若水。水善利万物而不争，处众人之所恶（wù），故几（jī）于道。居善地，心善渊，与</p><p style=\"text-align: start;\">善仁，言善信，正善治，事善能，动善时。夫唯不争，故无尤。 〖<a href=\"https://www.daodejing.org/8.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第九章】持而盈之，不如其已。揣(chuǎi)而锐之，不可长保。金玉满堂，莫之能守。富贵而骄，自遗（</p><p style=\"text-align: start;\">yí）其咎。功成身退，天之道。 〖<a href=\"https://www.daodejing.org/9.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十章】载（zài）营魄抱一，能无离乎？专气致柔，能婴儿乎？涤除玄览，能无疵乎？爱民治国，能无</p><p style=\"text-align: start;\">知（zhì）乎？天门开阖（hé），能无雌乎？明白四达，能无为乎？生之、畜（xù）之，生而不有，为而不</p><p style=\"text-align: start;\">恃，长（zhǎng）而不宰，是谓玄德。 〖<a href=\"https://www.daodejing.org/10.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十一章】三十辐共一毂（gǔ），当其无，有车之用。埏埴（shān zhí）以为器，当其无，有器之用。</p><p style=\"text-align: start;\">凿户牖（yǒu）以为室，当其无，有室之用。故有之以为利，无之以为用。 〖<a href=\"https://www.daodejing.org/11.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十二章】 五色令人目盲，五音令人耳聋，五味令人口爽，驰骋畋（tián）猎令人心发狂，难得之货令</p><p style=\"text-align: start;\">人行妨。是以圣人为腹不为目，故去彼取此。 〖<a href=\"https://www.daodejing.org/12.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十三章】宠辱若惊，贵大患若身。何谓宠辱若惊？宠为下，得之若惊，失之若惊，是谓宠辱若惊。何</p><p style=\"text-align: start;\">谓贵大患若身？吾所以有大患者，为吾有身，及吾无身，吾有何患！故贵以身为天下，若可寄天下；爱以</p><p style=\"text-align: start;\">身为天下，若可托天下。 〖<a href=\"https://www.daodejing.org/13.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十四章】视之不见名曰夷，听之不闻名曰希，搏之不得名曰微。此三者不可致诘（jié），故混（</p><p style=\"text-align: start;\">hùn）而为一。其上不皦（jiǎo皎），其下不昧。绳绳(mǐn mǐn )不可名，复归于无物，是谓无状之状，无</p><p style=\"text-align: start;\">物之象。是谓惚恍。迎之不见其首，随之不见其后。执古之道，以御今之有，能知古始，是谓道纪。 〖<a href=\"https://www.daodejing.org/14.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十五章】古之善为士者，微妙玄通，深不可识。夫唯不可识，故强(qiǎng)为之容。豫焉若冬涉川，犹</p><p style=\"text-align: start;\">兮若畏四邻，俨兮其若容，涣兮若冰之将释，敦兮其若朴，旷兮其若谷，混兮其若浊。孰能浊以静之徐清</p><p style=\"text-align: start;\">？孰能安以久动之徐生？保此道者不欲盈，夫唯不盈，故能蔽不新成。 〖<a href=\"https://www.daodejing.org/15.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十六章】致虚极，守静笃（dǔ），万物并作，吾以观复。夫物芸芸，各复归其根。归根曰静，是谓复</p><p style=\"text-align: start;\">命。复命曰常，知常曰明，不知常，妄作，凶。知常容，容乃公，公乃王（wàng），王（wàng）乃天，天</p><p style=\"text-align: start;\">乃道，道乃久，没（mò）身不殆。 〖<a href=\"https://www.daodejing.org/16.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十七章】太上，下知有之。其次，亲而誉之。其次，畏之。其次，侮之。信不足焉，有不信焉。悠兮</p><p style=\"text-align: start;\">其贵言。功成事遂，百姓皆谓我自然。 〖<a href=\"https://www.daodejing.org/17.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十八章】大道废，有仁义；慧智出，有大伪；六亲不和，有孝慈；国家昏乱，有忠臣。 〖<a href=\"https://www.daodejing.org/18.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第十九章】绝圣弃智，民利百倍；绝仁弃义，民复孝慈；绝巧弃利，盗贼无有。此三者，以为文不足，</p><p style=\"text-align: start;\">故令有所属，见（xiàn）素抱朴，少私寡欲。 〖<a href=\"https://www.daodejing.org/19.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十章】绝学无忧。唯之与阿（ē），相去几何？善之与恶，相去若何？人之所畏，不可不畏。荒兮其</p><p style=\"text-align: start;\">未央哉！众人熙熙，如享太牢，如春登台。我独泊兮其未兆，如婴儿之未孩。傫傫（lěi）兮若无所归。众</p><p style=\"text-align: start;\">人皆有余，而我独若遗。我愚人之心也哉！沌沌兮！俗人昭昭，我独昏昏；俗人察察，我独闷闷。澹（</p><p style=\"text-align: start;\">dàn）兮其若海，飂（liù）兮若无止。众人皆有以，而我独顽似鄙。我独异于人，而贵食(sì)母。 〖<a href=\"https://www.daodejing.org/20.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十一章】孔德之容，惟道是从。道之为物，惟恍惟惚。惚兮恍兮，其中有象；恍兮惚兮，其中有物</p><p style=\"text-align: start;\">。窈（yǎo）兮冥兮，其中有精；其精甚真，其中有信。自古及今，其名不去，以阅众甫。吾何以知众甫之</p><p style=\"text-align: start;\">状哉？以此。 〖<a href=\"https://www.daodejing.org/21.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十二章】曲则全，枉则直，洼则盈，敝则新，少则得，多则惑。是以圣人抱一，为天下式。不自见</p><p style=\"text-align: start;\">（xiàn）故明，不自是故彰，不自伐故有功，不自矜故长。夫唯不争，故天下莫能与之争。古之所谓曲则</p><p style=\"text-align: start;\">全者，岂虚言哉！诚全而归之。 〖<a href=\"https://www.daodejing.org/22.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十三章】希言自然。故飘风不终朝（zhāo），骤雨不终日。孰为此者？天地。天地尚不能久，而况</p><p style=\"text-align: start;\">于人乎？故从事于道者，道者同于道，德者同于德，失者同于失。同于道者，道亦乐得之；同于德者，德</p><p style=\"text-align: start;\">亦乐得之；同于失者，失亦乐得之。信不足焉，有不信焉。 〖<a href=\"https://www.daodejing.org/23.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十四章】企者不立，跨者不行，自见（xiàn）者不明，自是者不彰，自伐者无功，自矜者不长。其</p><p style=\"text-align: start;\">在道也，曰余食赘（zhuì）行。物或恶（wù）之，故有道者不处（chǔ）。 〖<a href=\"https://www.daodejing.org/24.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十五章】有物混（hùn）成，先天地生。寂兮寥兮，独立不改，周行而不殆，可以为天下母。吾不知</p><p style=\"text-align: start;\">其名，字之曰道，强(qiǎng)为之名曰大。大曰逝，逝曰远，远曰反。故道大，天大，地大，王亦大。域中</p><p style=\"text-align: start;\">有四大，而王居其一焉。人法地，地法天，天法道，道法自然。 〖<a href=\"https://www.daodejing.org/25.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十六章】重为轻根，静为躁君。是以圣人终日行不离辎（zī）重。虽有荣观（guàn），燕处超然，</p><p style=\"text-align: start;\">奈何万乘（shèng）之主，而以身轻天下？轻则失本，躁则失君。 〖<a href=\"https://www.daodejing.org/26.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十七章】善行无辙迹，善言无瑕谪(xiá zhé)，善数（shǔ）不用筹策，善闭无关楗（jiàn）而不可</p><p style=\"text-align: start;\">开，善结无绳约而不可解。是以圣人常善救人，故无弃人；常善救物，故无弃物，是谓袭明。故善人者，</p><p style=\"text-align: start;\">不善人之师；不善人者，善人之资。不贵其师，不爱其资，虽智大迷，是谓要妙。 〖<a href=\"https://www.daodejing.org/27.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十八章】知其雄，守其雌，为天下溪。为天下溪，常德不离，复归于婴儿。知其白，守其黑，为天</p><p style=\"text-align: start;\">下式。为天下式，常德不忒（tè），复归于无极。知其荣，守其辱，为天下谷。为天下谷，常德乃足，复</p><p style=\"text-align: start;\">归于朴。朴散则为器，圣人用之则为官长（zhǎng）。故大制不割。 〖<a href=\"https://www.daodejing.org/28.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第二十九章】将欲取天下而为之，吾见其不得已。天下神器，不可为也。为者败之，执者失之。故物或</p><p style=\"text-align: start;\">行或随，或歔（xū）或吹，或强或羸（léi），或挫或隳（huī）。是以圣人去甚，去奢，去泰。 〖<a href=\"https://www.daodejing.org/29.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十章】以道佐人主者，不以兵强天下，其事好（hào）还。师之所处，荆棘生焉。大军之后，必有凶</p><p style=\"text-align: start;\">年。善有果而已，不敢以取强。果而勿矜，果而勿伐，果而勿骄，果而不得已，果而勿强。物壮则老，是</p><p style=\"text-align: start;\">谓不道，不道早已。 〖<a href=\"https://www.daodejing.org/30.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十一章】夫佳兵者，不祥之器。物或恶（wù）之，故有道者不处（chǔ）。君子居则贵左，用兵则贵</p><p style=\"text-align: start;\">右。兵者，不祥之器，非君子之器。不得已而用之，恬淡为上，胜而不美。而美之者，是乐(yào)杀人。夫</p><p style=\"text-align: start;\">乐(yào)杀人者，则不可以得志于天下矣。吉事尚左，凶事尚右。偏将军居左，上将军居右，言以丧（</p><p style=\"text-align: start;\">sāng）礼处之。杀人之众，以哀悲泣之，战胜，以丧礼处之。 〖<a href=\"https://www.daodejing.org/31.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十二章】道常无名，朴虽小，天下莫能臣也。侯王若能守之，万物将自宾。天地相合以降甘露，民</p><p style=\"text-align: start;\">莫之令而自均。始制有名，名亦既有，夫亦将知止。知止可以不殆。譬道之在天下，犹川谷之于江海。 〖<a href=\"https://www.daodejing.org/32.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十三章】知人者智，自知者明。胜人者有力，自胜者强。知足者富，强行者有志，不失其所者久，</p><p style=\"text-align: start;\">死而不亡者寿。 〖<a href=\"https://www.daodejing.org/33.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十四章】大道泛兮，其可左右。万物恃之而生而不辞，功成不名有，衣养万物而不为主，常无欲，</p><p style=\"text-align: start;\">可名于小；万物归焉而不为主，可名为大。以其终不自为大，故能成其大。 〖<a href=\"https://www.daodejing.org/34.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十五章】执大象，天下往；往而不害，安平太。乐（yuè）与饵，过客止。道之出口，淡乎其无味，</p><p style=\"text-align: start;\">视之不足见（jiàn），听之不足闻，用之不足既。 〖<a href=\"https://www.daodejing.org/35.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十六章】将欲歙（xī）之，必固张之；将欲弱之，必固强之；将欲废之，必固兴之；将欲夺之，必</p><p style=\"text-align: start;\">固与之，是谓微明。柔弱胜刚强。鱼不可脱于渊，国之利器不可以示人。 〖<a href=\"https://www.daodejing.org/36.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十七章】道常无为而无不为，侯王若能守之，万物将自化。化而欲作，吾将镇之以无名之朴。无名</p><p style=\"text-align: start;\">之朴，夫亦将无欲。不欲以静，天下将自定。 〖<a href=\"https://www.daodejing.org/37.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十八章】上德不德，是以有德；下德不失德，是以无德。上德无为而无以为，下德为之而有以为。</p><p style=\"text-align: start;\">上仁为之而无以为，上义为之而有以为，上礼为之而莫之应，则攘(rǎng)臂而扔之。故失道而后德，失德</p><p style=\"text-align: start;\">而后仁，失仁而后义，失义而后礼。夫礼者，忠信之薄(bó)而乱之首。前识者，道之华而愚之始。是以大</p><p style=\"text-align: start;\">丈夫处其厚，不居其薄(bó)；处其实，不居其华。故去彼取此。 〖<a href=\"https://www.daodejing.org/38.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第三十九章】昔之得一者，天得一以清，地得一以宁，神得一以灵，谷得一以盈，万物得一以生，侯王</p><p style=\"text-align: start;\">得一以为天下贞。其致之。天无以清将恐裂，地无以宁将恐发（fèi，“发”通“废”），神无以灵将恐歇</p><p style=\"text-align: start;\">，谷无以盈将恐竭，万物无以生将恐灭，侯王无以贵高将恐蹶（jué）。故贵以贱为本，高以下为基。是以</p><p style=\"text-align: start;\">侯王自谓孤寡不穀（谷gǔ）。此非以贱为本邪（yé）？非乎？故致数（shuò）舆（yù）无舆。不欲琭（</p><p style=\"text-align: start;\">lù）琭如玉，珞（luò）珞如石。 〖<a href=\"https://www.daodejing.org/39.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十章】反者，道之动；弱者，道之用。天下万物生于有，有生于无。 〖<a href=\"https://www.daodejing.org/40.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十一章】 上士闻道，勤而行之；中士闻道，若存若亡；下士闻道，大笑之，不笑不足以为道。故建</p><p style=\"text-align: start;\">言有之：明道若昧，进道若退，夷道若颣（lèi）。上德若谷，大白若辱，广德若不足，建德若偷，质真若</p><p style=\"text-align: start;\">渝（yú）。大方无隅（yú），大器晚成，大音希声，大象无形。道隐无名，夫唯道善贷且成。 〖<a href=\"https://www.daodejing.org/41.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十二章】道生一，一生二，二生三，三生万物。万物负阴而抱阳，冲气以为和。人之所恶（wù），</p><p style=\"text-align: start;\">唯孤寡不穀（谷gǔ），而王公以为称（chēng）。故物，或损之而益，或益之而损。人之所教（jiào），我</p><p style=\"text-align: start;\">亦教之。强梁者不得其死，吾将以为教父。 〖<a href=\"https://www.daodejing.org/42.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十三章】天下之至柔，驰骋天下之至坚，无有入无间，吾是以知无为之有益。不言之教，无为之益</p><p style=\"text-align: start;\">，天下希及之。 〖<a href=\"https://www.daodejing.org/43.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十四章】名与身孰亲？身与货孰多？得与亡孰病？ 是故甚爱必大费，多藏必厚亡。知足不辱，知止</p><p style=\"text-align: start;\">不殆，可以长久。 〖<a href=\"https://www.daodejing.org/44.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十五章】大成若缺，其用不弊。大盈若冲，其用不穷。大直若屈，大巧若拙，大辩若讷。躁胜寒，</p><p style=\"text-align: start;\">静胜热。清静为天下正。 〖<a href=\"https://www.daodejing.org/45.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十六章】天下有道，却走马以粪；天下无道，戎马生于郊。祸莫大于不知足，咎莫大于欲得，故知</p><p style=\"text-align: start;\">足之足，常足矣。 〖<a href=\"https://www.daodejing.org/46.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十七章】不出户，知天下；不窥牖，见天道。其出弥远，其知弥少。是以圣人不行而知，不见而名</p><p style=\"text-align: start;\">，不为而成。 〖<a href=\"https://www.daodejing.org/47.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十八章】为学日益，为道日损。损之又损，以至于无为，无为而无不为。取天下常以无事，及其有</p><p style=\"text-align: start;\">事，不足以取天下。 〖<a href=\"https://www.daodejing.org/48.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第四十九章】圣人无常心，以百姓心为心。善者，吾善之；不善者，吾亦善之，德善。信者，吾信之；</p><p style=\"text-align: start;\">不信者，吾亦信之，德信。圣人在天下歙歙（xīxī），为天下浑其心。（百姓皆注其耳目），圣人皆孩之。 〖<a href=\"https://www.daodejing.org/49.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十章】出生入死。生之徒十有三，死之徒十有三。人之生动之死地，亦十有三。夫何故？以其生生</p><p style=\"text-align: start;\">之厚。盖闻善摄生者，陆行不遇兕（sì）虎，入军不被（pī）甲兵，兕无所投其角，虎无所措其爪（</p><p style=\"text-align: start;\">zhǎo），兵无所容其刃。夫何故？以其无死地。 〖<a href=\"https://www.daodejing.org/50.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十一章】道生之，德畜（xù）之，物形之，势成之。是以万物莫不尊道而贵德。道之尊，德之贵，</p><p style=\"text-align: start;\">夫莫之命而常自然。故道生之，德畜之。长之、育之、亭之、毒之、养之、覆之。生而不有，为而不恃，</p><p style=\"text-align: start;\">长（zhǎng）而不宰，是谓玄德。 〖<a href=\"https://www.daodejing.org/51.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十二章】天下有始，以为天下母。既得其母，以知其子；既知其子，复守其母，没（mò）身不殆。</p><p style=\"text-align: start;\">塞（sè）其兑，闭其门，终身不勤。开其兑，济其事，终身不救。见（jiàn）小曰明，守柔曰强。用其光</p><p style=\"text-align: start;\">，复归其明，无遗身殃，是为习常。 〖<a href=\"https://www.daodejing.org/52.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十三章】使我介然有知，行于大道，唯施（迤yí）是畏。大道甚夷，而民好径。朝（cháo）甚除，</p><p style=\"text-align: start;\">田甚芜，仓甚虚。服文彩，带利剑，厌饮食，财货有余，是为盗夸。非道也哉！ 〖<a href=\"https://www.daodejing.org/53.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十四章】善建者不拔，善抱者不脱，子孙以祭祀不辍。修之于身，其德乃真；修之于家，其德乃余</p><p style=\"text-align: start;\">；修之于乡，其德乃长（zhǎng）；修之于国，其德乃丰；修之于天下，其德乃普。故以身观身，以家观家</p><p style=\"text-align: start;\">，以乡观乡，以国观国，以天下观天下。吾何以知天下然哉？以此。 〖<a href=\"https://www.daodejing.org/54.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十五章】 含德之厚，比于赤子。蜂虿（chài）虺（huǐ）蛇不螫(shì)，猛兽不据，攫(jué)鸟不搏</p><p style=\"text-align: start;\">。骨弱筋柔而握固。未知牝牡之合而全作，精之至也。终日号而不嗄（shà），和之至也。知和曰常，知常</p><p style=\"text-align: start;\">曰明，益生曰祥，心使气曰强。物壮则老，谓之不道，不道早已。 〖<a href=\"https://www.daodejing.org/55.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十六章】知（zhì）者不言，言者不知（zhì）。塞（sè）其兑，闭其门，挫其锐；解其纷，和其光</p><p style=\"text-align: start;\">，同其尘，是谓玄同。故不可得而亲，不可得而疏；不可得而利，不可得而害；不可得而贵，不可得而贱</p><p style=\"text-align: start;\">，故为天下贵。 〖<a href=\"https://www.daodejing.org/56.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十七章】以正治国，以奇用兵，以无事取天下。吾何以知其然哉？以此。天下多忌讳，而民弥贫；</p><p style=\"text-align: start;\">民多利器，国家滋昏；人多伎（jì）巧，奇物滋起；法令滋彰，盗贼多有。故圣人云：“我无为而民自化</p><p style=\"text-align: start;\">，我好静而民自正，我无事而民自富，我无欲而民自朴。” 〖<a href=\"https://www.daodejing.org/57.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十八章】其政闷闷，其民淳淳；其政察察，其民缺缺。祸兮福之所倚，福兮祸之所伏。孰知其极？</p><p style=\"text-align: start;\">其无正。正复为奇，善复为妖，人之迷，其日固久。是以圣人方而不割，廉而不刿（guì），直而不肆，光</p><p style=\"text-align: start;\">而不耀。 〖<a href=\"https://www.daodejing.org/58.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第五十九章】治人事天莫若啬（sè）。夫唯啬，是谓早服。早服谓之重(chóng)积德，重(chóng)积德则</p><p style=\"text-align: start;\">无不克，无不克则莫知其极，莫知其极，可以有国。有国之母，可以长久。是谓深根固柢（dǐ），长生久</p><p style=\"text-align: start;\">视之道。 〖<a href=\"https://www.daodejing.org/59.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十章】治大国若烹小鲜。以道莅（lì）天下，其鬼不神。非其鬼不神，其神不伤人；非其神不伤人</p><p style=\"text-align: start;\">，圣人亦不伤人。夫两不相伤，故德交归焉。 〖<a href=\"https://www.daodejing.org/60.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十一章】大国者下流。天下之交，天下之牝。牝常以静胜牡，以静为下。故大国以下小国，则取小</p><p style=\"text-align: start;\">国；小国以下大国，则取大国。故或下以取，或下而取。大国不过欲兼畜（xù）人，小国不过欲入事人，</p><p style=\"text-align: start;\">夫两者各得其所欲，大者宜为下。 〖<a href=\"https://www.daodejing.org/61.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十二章】道者万物之奥，善人之宝，不善人之所保。美言可以市，尊行可以加人。人之不善，何弃</p><p style=\"text-align: start;\">之有！故立天子，置三公，虽有拱璧以先驷马，不如坐进此道。古之所以贵此道者何？不曰以求得，有罪</p><p style=\"text-align: start;\">以免邪（yé）？故为天下贵。 〖<a href=\"https://www.daodejing.org/62.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十三章】为无为，事无事，味无味。大小多少，报怨以德。图难于其易，为大于其细。天下难事必</p><p style=\"text-align: start;\">作于易，天下大事必作于细，是以圣人终不为大，故能成其大。夫轻诺必寡信，多易必多难，是以圣人犹</p><p style=\"text-align: start;\">难之。故终无难矣。 〖<a href=\"https://www.daodejing.org/63.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十四章】其安易持，其未兆易谋，其脆易泮（pàn），其微易散。为之于未有，治之于未乱。合抱之</p><p style=\"text-align: start;\">木，生于毫末；九层之台，起于累土；千里之行，始于足下。为者败之，执者失之。是以圣人无为，故无</p><p style=\"text-align: start;\">败；无执，故无失。民之从事，常于几成而败之。慎终如始，则无败事。是以圣人欲不欲，不贵难得之货</p><p style=\"text-align: start;\">。学不学，复众人之所过。以辅万物之自然，而不敢为。 〖<a href=\"https://www.daodejing.org/64.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十五章】古之善为道者，非以明民，将以愚之。民之难治，以其智多。故以智治国，国之贼；不以</p><p style=\"text-align: start;\">智治国，国之福。知此两者，亦稽（jī）式。常知稽式，是谓玄德。玄德深矣，远矣，与物反矣，然后乃</p><p style=\"text-align: start;\">至大顺。 〖<a href=\"https://www.daodejing.org/65.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十六章】江海所以能为百谷王者，以其善下之，故能为百谷王。是以欲上民，必以言下之；欲先民</p><p style=\"text-align: start;\">，必以身后之。是以圣人处上而民不重，处前而民不害，是以天下乐推而不厌。以其不争，故天下莫能与</p><p style=\"text-align: start;\">之争。 〖<a href=\"https://www.daodejing.org/66.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十七章】天下皆谓我道大，似不肖（xiào）。夫唯大，故似不肖。若肖，久矣其细也夫。我有三宝</p><p style=\"text-align: start;\">，持而保之。一曰慈，二曰俭，三曰不敢为天下先。慈，故能勇；俭，故能广；不敢为天下先，故能成器</p><p style=\"text-align: start;\">长（zhǎng）。今舍慈且勇，舍俭且广，舍后且先，死矣！夫慈，以战则胜，以守则固，天将救之，以慈卫</p><p style=\"text-align: start;\">之。 〖<a href=\"https://www.daodejing.org/67.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十八章】善为士者不武，善战者不怒，善胜敌者不与，善用人者为之下。是谓不争之德，是谓用人</p><p style=\"text-align: start;\">之力，是谓配天古之极。 〖<a href=\"https://www.daodejing.org/68.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第六十九章】用兵有言，吾不敢为主而为客，不敢进寸而退尺。是谓行（xíng）无行（háng），攘</p><p style=\"text-align: start;\">(rǎng)无臂，扔无敌，执无兵。祸莫大于轻敌，轻敌几丧吾宝。故抗兵相加，哀者胜矣。 〖<a href=\"https://www.daodejing.org/69.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十章】吾言甚易知，甚易行，天下莫能知，莫能行。言有宗，事有君。夫唯无知，是以不我知。知</p><p style=\"text-align: start;\">我者希，则我者贵，是以圣人被（pī，“被”同“披”）褐怀玉。 〖<a href=\"https://www.daodejing.org/70.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十一章】知不知，上；不知知，病。夫唯病病，是以不病。圣人不病，以其病病，是以不病。 〖<a href=\"https://www.daodejing.org/71.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十二章】民不畏威，则大威至。无狎（xiá）其所居，无厌（yà，“厌”同“压”）其所生。夫唯不</p><p style=\"text-align: start;\">厌（yà，“厌”同“压”），是以不厌(yàn)。是以圣人自知，不自见（xiàn）；自爱，不自贵。故去彼取</p><p style=\"text-align: start;\">此。 〖<a href=\"https://www.daodejing.org/72.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十三章】勇于敢则杀，勇于不敢则活。此两者，或利或害。天之所恶（wù），孰知其故？是以圣人</p><p style=\"text-align: start;\">犹难之。天之道，不争而善胜，不言而善应，不召而自来，繟（chǎn）然而善谋。天网恢恢，疏而不失。 〖<a href=\"https://www.daodejing.org/73.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十四章】民不畏死，奈何以死惧之！若使民常畏死，而为奇者，吾得执而杀之，孰敢？常有司杀者</p><p style=\"text-align: start;\">杀，夫代司杀者杀，是谓代大匠斫（zhuó）。夫代大匠斫者，希有不伤其手矣。 〖<a href=\"https://www.daodejing.org/74.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十五章】民之饥，以其上食税之多，是以饥。民之难治，以其上之有为，是以难治。民之轻死，以</p><p style=\"text-align: start;\">其求生之厚，是以轻死。夫唯无以生为者，是贤于贵生。 〖<a href=\"https://www.daodejing.org/75.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十六章】人之生也柔弱，其死也坚强。万物草木之生也柔脆，其死也枯槁。故坚强者死之徒，柔弱</p><p style=\"text-align: start;\">者生之徒。是以兵强则不胜，木强则兵。强大处下，柔弱处上。 〖<a href=\"https://www.daodejing.org/76.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十七章】天之道，其犹张弓与！高者抑之，下者举之；有余者损之，不足者补之。天之道，损有余</p><p style=\"text-align: start;\">而补不足。人之道则不然，损不足以奉有余。孰能有余以奉天下？唯有道者。是以圣人为而不恃，功成而</p><p style=\"text-align: start;\">不处，其不欲见（xiàn）贤。 〖<a href=\"https://www.daodejing.org/77.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十八章】天下莫柔弱于水，而攻坚强者莫之能胜，其无以易之。弱之胜强，柔之胜刚，天下莫不知</p><p style=\"text-align: start;\">，莫能行。是以圣人云，受国之垢，是谓社稷主；受国不祥，是为天下王。正言若反。 〖<a href=\"https://www.daodejing.org/78.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第七十九章】和大怨，必有余怨，安可以为善？是以圣人执左契，而不责于人。有德司契，无德司彻。</p><p style=\"text-align: start;\">天道无亲，常与善人。 〖<a href=\"https://www.daodejing.org/79.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第八十章】小国寡民，使有什伯（bǎi）之器而不用，使民重（zhòng）死而不远徙(xí)。虽有舟舆，无</p><p style=\"text-align: start;\">所乘之；虽有甲兵，无所陈之；使人复结绳而用之。甘其食，美其服，安其居，乐其俗。邻国相望，鸡犬</p><p style=\"text-align: start;\">之声相闻，民至老死不相往来。 〖<a href=\"https://www.daodejing.org/80.html\" target=\"\">译文</a>〗</p><p style=\"text-align: start;\"> </p><p style=\"text-align: start;\">【第八十一章】信言不美，美言不信；善者不辩，辩者不善；知（zhì）者不博，博者不知（zhì）。圣人</p><p style=\"text-align: start;\">不积，既以为人，己愈有；既以与人，己愈多。天之道，利而不害。圣人之道，为而不争。〖<a href=\"https://www.daodejing.org/81.html\" target=\"\">译文</a>〗</p>', 'https://rust-blogs.oss-cn-guangzhou.aliyuncs.com/image/2025/02/21/e9722495-d4a4-465c-af32-e349a7309bc2.jpg', '<p><br></p><p style=\"text-align: start;\">【第一章】道可道，非常道；名可名，非常名。无名天地之始，有名万物之母。故常无欲，以观其妙；常</p><p style=\"text-align: start;\">有欲，以观其徼（jiào）。此两者同出而异名，同谓之玄，玄之又玄，众妙之门。〖<a href=\"https://www.daodejing.org/1.htm', 0, 1, 0, '2025-02-21 22:06:57', '2025-02-22 22:05:04');
INSERT INTO `articles` VALUES (454, 10, '测试', '<p>测试</p>', 'https://rust-blogs.oss-cn-guangzhou.aliyuncs.com/image/2025/02/22/1bbdabe2-c569-4647-b453-8ebba965e74d.jpeg', '<p>测试</p>', 0, 1, 0, '2025-02-22 19:00:20', '2025-02-22 19:04:25');

-- ----------------------------
-- Table structure for blog_comments
-- ----------------------------
DROP TABLE IF EXISTS `blog_comments`;
CREATE TABLE `blog_comments`  (
  `comment_id` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '评论ID',
  `post_id` bigint UNSIGNED NOT NULL COMMENT '关联文章ID',
  `user_id` bigint UNSIGNED NOT NULL COMMENT '评论用户ID',
  `content` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '评论内容',
  `parent_id` bigint UNSIGNED NOT NULL DEFAULT 0 COMMENT '父评论ID（0表示顶级评论）',
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `level` smallint UNSIGNED NOT NULL DEFAULT 1 COMMENT '评论层级',
  `path` varchar(1000) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '0' COMMENT '层级路径',
  PRIMARY KEY (`comment_id`) USING BTREE,
  INDEX `idx_post`(`post_id` ASC) USING BTREE,
  INDEX `idx_parent`(`parent_id` ASC) USING BTREE,
  INDEX `idx_path`(`path`(255) ASC) USING BTREE,
  INDEX `idx_created`(`created_at` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 44 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '博客评论表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of blog_comments
-- ----------------------------
INSERT INTO `blog_comments` VALUES (35, 453, 1, '1', 0, '2025-02-23 15:09:22', 1, '0/');
INSERT INTO `blog_comments` VALUES (37, 453, 1, '`12', 35, '2025-02-23 15:21:17', 2, '0/35/');
INSERT INTO `blog_comments` VALUES (38, 453, 1, '2', 37, '2025-02-23 15:21:57', 3, '0/35/37/');
INSERT INTO `blog_comments` VALUES (39, 454, 1, 'c', 0, '2025-02-23 15:33:05', 1, '0/');
INSERT INTO `blog_comments` VALUES (40, 454, 1, 'a', 0, '2025-02-23 15:33:07', 1, '0/');
INSERT INTO `blog_comments` VALUES (41, 454, 1, 'a', 39, '2025-02-23 15:33:11', 2, '0/39/');
INSERT INTO `blog_comments` VALUES (42, 454, 1, 'a', 41, '2025-02-23 15:33:14', 3, '0/39/41/');
INSERT INTO `blog_comments` VALUES (43, 454, 1, '09\n', 0, '2025-02-23 15:33:25', 1, '0/');

-- ----------------------------
-- Table structure for blog_user
-- ----------------------------
DROP TABLE IF EXISTS `blog_user`;
CREATE TABLE `blog_user`  (
  `user_id` bigint UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户ID',
  `auth_type` tinyint NOT NULL COMMENT '认证方式(0=邮箱 1=GitHub ...)',
  `auth_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '认证标识(邮箱/第三方UID)',
  `password_hash` char(60) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '密码哈希（仅本地注册用户需要）',
  `nickname` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '昵称',
  `avatar_url` varchar(512) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '/static/default_avatar.png' COMMENT '头像',
  `registration_ip` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '注册IP',
  `last_login` datetime NULL DEFAULT NULL COMMENT '最后登录时间',
  `user_status` enum('active','locked') CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT 'active' COMMENT '状态',
  `create_time` datetime NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` datetime NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `provider_uid` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '第三方平台用户ID',
  PRIMARY KEY (`user_id`) USING BTREE,
  UNIQUE INDEX `uniq_auth`(`auth_type` ASC, `auth_id` ASC) USING BTREE,
  INDEX `idx_provider`(`provider_uid` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of blog_user
-- ----------------------------

-- ----------------------------
-- Table structure for sys_menu
-- ----------------------------
DROP TABLE IF EXISTS `sys_menu`;
CREATE TABLE `sys_menu`  (
  `menu_id` int NOT NULL AUTO_INCREMENT COMMENT '菜单ID',
  `menu_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单名称',
  `title_en` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '英文标题',
  `parent_id` bigint NULL DEFAULT 0 COMMENT '父菜单ID',
  `order_num` int NULL DEFAULT 0 COMMENT '显示顺序',
  `path` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '路由地址',
  `route_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '路由名称',
  `keep_alive` int NOT NULL DEFAULT 0 COMMENT '是否缓存（0缓存 1不缓存）',
  `is_frame` int NULL DEFAULT 1 COMMENT '是否为外链（0是 1否）',
  `menu_type` char(1) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '菜单类型（M目录 C菜单 F按钮）',
  `is_hide` tinyint NULL DEFAULT NULL COMMENT '是否隐藏 (0 是 1 否)',
  `icon` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '#' COMMENT '菜单图标',
  `remark` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '备注',
  `show_badge` int NULL DEFAULT NULL COMMENT '是否显示小红点(0 是 1否)',
  `show_text_badge` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '是否显示文本小红点(0 是 1否)',
  `is_hide_tab` int NULL DEFAULT NULL COMMENT '是否隐藏tab (0 是 1 否)',
  `link` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '外链接',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '创建者',
  `create_time` datetime NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT '' COMMENT '更新者',
  `update_time` datetime NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `permission` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '权限',
  `is_enable` int NULL DEFAULT NULL COMMENT '是否启用(0 启用 1禁用)',
  PRIMARY KEY (`menu_id`, `keep_alive`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 202 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '菜单权限表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_menu
-- ----------------------------
INSERT INTO `sys_menu` VALUES (24, '仪表盘', 'Dashboard', 0, 0, '/dashboard', 'Dashboard', 0, 1, 'M', 1, '&#xe721;', '仪表盘', 1, NULL, 1, '', 'admin', '2025-02-17 21:24:33', '1', '2025-02-21 23:45:13', 'sys:dashboard:*', 0);
INSERT INTO `sys_menu` VALUES (25, '工作台', 'Workbench', 24, 0, '/dashboard/console', '', 0, 1, 'C', 1, '', '工作台', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:25:17', 'admin', '2025-02-20 23:22:07', 'sys:dashboard:console', 0);
INSERT INTO `sys_menu` VALUES (26, '分析页', 'Analysis', 24, 1, '/dashboard/analysis', '', 0, 1, 'C', 1, '', '分析页', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:26:24', 'admin', '2025-02-20 23:22:07', 'sys:dashboard:analysis', 0);
INSERT INTO `sys_menu` VALUES (27, '组件中心', 'Components', 0, 1, '/widgets', '/widgets', 0, 1, 'M', 1, '&#xe81a;', '组件中心', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:29:53', 'admin', '2025-02-20 23:22:07', 'sys:widgets:*', 0);
INSERT INTO `sys_menu` VALUES (28, 'Icon 图标', 'Icon', 27, 0, '/widgets/icon-list', '', 0, 1, 'C', 1, '', 'Icon 图标', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:31:44', 'admin', '2025-02-20 23:22:07', 'sys:widgets:icon-list', 0);
INSERT INTO `sys_menu` VALUES (29, '图标选择器', 'Icon selector', 27, 1, '/widgets/icon-selector', '', 0, 1, 'C', 1, '', '图标选择器', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:33:18', 'admin', '2025-02-20 23:22:07', 'sys:widgets/icon-selector', 0);
INSERT INTO `sys_menu` VALUES (30, '图像裁剪', 'Image crop', 27, 2, '/widgets/image-crop', '', 0, 1, 'C', 1, '', '图像裁剪', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:34:29', 'admin', '2025-02-20 23:22:07', 'sys:widgets/image-crop', 0);
INSERT INTO `sys_menu` VALUES (31, 'Excel 导入导出', 'Excel import and export', 27, 3, '/widgets/excel', '', 0, 1, 'C', 1, '', 'Excel 导入导出', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:36:59', 'admin', '2025-02-20 23:22:07', 'sys:widgets/excel', 0);
INSERT INTO `sys_menu` VALUES (32, '视频播放器', 'Video', 27, 4, '/widgets/video', '', 0, 1, 'C', 1, '', '视频播放器', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:38:20', 'admin', '2025-02-20 23:22:07', 'sys:widgets/video', 0);
INSERT INTO `sys_menu` VALUES (33, '数字滚动', 'Count to', 27, 5, '/widgets/count-to', '', 0, 1, 'C', 1, '', '数字滚动', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:39:02', 'admin', '2025-02-20 23:22:07', 'sys:widgets/count-to', 0);
INSERT INTO `sys_menu` VALUES (34, '富文本编辑器', 'Rich text editor', 27, 6, '/widgets/wang-editor', '', 0, 1, 'C', 1, '', '富文本编辑器', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:40:12', 'admin', '2025-02-20 23:22:07', 'sys:widgets/wang-editor', 0);
INSERT INTO `sys_menu` VALUES (35, '水印', 'Watermark', 27, 7, '/widgets/watermark', '', 0, 1, 'C', 1, '', '水印', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:40:53', 'admin', '2025-02-20 23:22:07', 'sys:widgets/watermark', 0);
INSERT INTO `sys_menu` VALUES (36, '右键菜单', 'Context menu', 27, 8, '/widgets/context-menu', '', 0, 1, 'C', 1, '', '右键菜单', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:41:30', 'admin', '2025-02-20 23:22:07', 'sys:widgets/context-menu', 0);
INSERT INTO `sys_menu` VALUES (37, '二维码', 'QR code', 27, 9, '/widgets/qrcode', '', 0, 1, 'C', 1, '', '二维码', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:42:12', 'admin', '2025-02-20 23:22:07', 'sys:widgets/qrcode', 0);
INSERT INTO `sys_menu` VALUES (38, '拖拽', 'Drag', 27, 10, '/widgets/drag', '', 0, 1, 'C', 1, '', '拖拽', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:42:41', 'admin', '2025-02-20 23:22:07', 'sys:widgets/drag', 0);
INSERT INTO `sys_menu` VALUES (39, '文字滚动', 'Text scroll', 27, 11, '/widgets/text-scroll', '', 0, 1, 'C', 1, '', '文字滚动', 1, '', 1, NULL, 'admin', '2025-02-17 21:43:34', 'admin', '2025-02-20 23:22:07', 'sys:widgets/text-scroll', 0);
INSERT INTO `sys_menu` VALUES (40, '礼花', 'Fireworks', 27, 12, '/widgets/fireworks', '', 0, 1, 'C', 1, '', '礼花', 0, 'Hot', 1, NULL, 'admin', '2025-02-17 21:44:21', 'admin', '2025-02-20 23:22:07', 'sys:widgets/fireworks', 0);
INSERT INTO `sys_menu` VALUES (41, '组件总览', 'Element UI', 27, 13, '', '', 0, 0, 'C', 1, '', '组件总览', 0, NULL, 1, 'https://element-plus.org/zh-CN/component/overview.html', 'admin', '2025-02-17 21:44:55', 'admin', '2025-02-20 23:22:07', '', 0);
INSERT INTO `sys_menu` VALUES (42, '模板中心', 'Template', 0, 2, '/template', 'Template', 0, 1, 'M', 1, '&#xe860;', '模板中心', 1, NULL, 1, '', 'admin', '2025-02-17 21:51:35', '1', '2025-02-21 18:33:06', 'sys:template:*', 0);
INSERT INTO `sys_menu` VALUES (43, '聊天', 'Chat', 42, 0, '/template/chat', '', 0, 1, 'C', 1, '', '聊天', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:52:58', 'admin', '2025-02-20 23:22:07', 'sys:template:chat', 0);
INSERT INTO `sys_menu` VALUES (44, '卡片', 'Cards', 42, 1, '/template/cards', '', 0, 1, 'C', 1, '', '卡片', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:54:44', 'admin', '2025-02-20 23:22:07', 'sys:template:cards', 0);
INSERT INTO `sys_menu` VALUES (45, '横幅', 'Banners', 42, 2, '/template/banners', '', 0, 1, 'C', 1, '', '横幅', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:55:40', 'admin', '2025-02-20 23:22:07', 'sys:template:banners', 0);
INSERT INTO `sys_menu` VALUES (46, '图表', 'Charts', 42, 3, '/template/charts', '', 0, 1, 'C', 1, '', '图表', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:57:43', 'admin', '2025-02-20 23:22:07', 'sys:template:charts', 0);
INSERT INTO `sys_menu` VALUES (47, '日历', 'Calendar', 42, 4, '/template/calendar', '', 0, 1, 'C', 1, '', '日历', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:58:54', 'admin', '2025-02-20 23:22:07', 'sys:template:calendar', 0);
INSERT INTO `sys_menu` VALUES (48, '定价', 'Pricing', 42, 5, '/template/pricing', '', 0, 1, 'C', 1, '', '定价', 1, NULL, 1, NULL, 'admin', '2025-02-17 21:59:29', 'admin', '2025-02-20 23:22:07', 'sys:template:pricing', 0);
INSERT INTO `sys_menu` VALUES (49, '文章管理', 'Article manguage', 0, 3, '/article', 'Article', 1, 1, 'M', 1, '&#xe7ae;', '文章管理', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:01:50', '1', '2025-02-23 15:34:49', 'sys:article:*', 0);
INSERT INTO `sys_menu` VALUES (50, '文章列表', 'Article list', 49, 0, '/article/article-list', '', 1, 1, 'M', 1, '', '文章列表', 1, '', 1, NULL, 'admin', '2025-02-17 22:02:28', '1', '2025-02-21 23:33:03', 'sys:article:article-list', 0);
INSERT INTO `sys_menu` VALUES (51, '文章详情', 'Article category', 49, 1, '/article/detail', '', 1, 1, 'M', 1, '', '文章详情', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:04:53', '1', '2025-02-21 23:15:03', 'sys:article:detail', 0);
INSERT INTO `sys_menu` VALUES (52, '留言管理', 'Comment', 49, 2, '/article/comment', '', 0, 1, 'M', 1, '', '留言管理', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:05:38', 'admin', '2025-02-23 17:29:59', 'sys:article:comment', 0);
INSERT INTO `sys_menu` VALUES (53, '文章发布', 'Article publish', 49, 3, '/article/article-publish', '', 1, 1, 'M', 1, '', '文章发布', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:07:01', '1', '2025-02-21 21:18:24', 'sys:article:article-publish', 0);
INSERT INTO `sys_menu` VALUES (54, '用户管理', 'User manguage', 0, 4, '/user', 'User', 0, 1, 'M', 1, '&#xe86e;', '用户管理', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:08:49', 'admin', '2025-02-20 23:22:07', 'sys:user:*', 0);
INSERT INTO `sys_menu` VALUES (55, '账号管理', 'Account manguage', 54, 0, '/user/account', '', 0, 1, 'C', 1, '', '账号管理', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:11:42', 'admin', '2025-02-20 23:22:07', 'sys:user:account', 0);
INSERT INTO `sys_menu` VALUES (57, '角色权限', 'Roles', 54, 2, '/user/role', '', 0, 1, 'C', 1, '', '角色权限', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:13:06', 'admin', '2025-02-20 23:22:07', 'sys:role:*', 0);
INSERT INTO `sys_menu` VALUES (58, '个人中心', 'User center', 54, 3, '/user/user', '', 0, 1, 'C', 0, '', '个人中心', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:13:55', 'admin', '2025-02-20 23:22:07', 'sys:user:user', 0);
INSERT INTO `sys_menu` VALUES (59, '菜单管理', 'Menu manguage', 0, 5, '/menu', 'Menu', 0, 1, 'M', 1, '&#xe8a4;', '菜单管理', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:15:22', 'admin', '2025-02-20 23:22:07', 'sys:menu:*', 0);
INSERT INTO `sys_menu` VALUES (60, '菜单权限', 'Menu permissions', 59, 0, '/menu/menu', '', 0, 1, 'C', 1, '&#xe8a4;', '菜单权限', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:17:53', 'admin', '2025-02-20 23:22:07', 'sys:menu:menu', 0);
INSERT INTO `sys_menu` VALUES (61, '权限控制', 'Permission control', 59, 1, '/menu/permission', '', 0, 1, 'C', 1, '&#xe831;', '权限控制', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:20:48', 'admin', '2025-02-20 23:22:07', 'sys:menu:permission', 0);
INSERT INTO `sys_menu` VALUES (62, '嵌套菜单', 'Nested menu', 59, 2, '/menu/nested', '', 0, 1, 'C', 1, '&#xe676;', '嵌套菜单', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:21:40', 'admin', '2025-02-20 23:22:07', 'sys:menu:nested', 0);
INSERT INTO `sys_menu` VALUES (63, '菜单1', 'menu1', 62, 0, '/menu/nested/menu1', '', 0, 1, 'C', 1, '&#xe676;', '菜单1', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:22:17', 'admin', '2025-02-20 23:22:07', NULL, 0);
INSERT INTO `sys_menu` VALUES (64, '菜单2', 'menu2', 62, 1, '/menu/nested/menu2', '', 0, 1, 'C', 1, '&#xe676;', '菜单2', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:22:53', 'admin', '2025-02-20 23:22:07', NULL, 0);
INSERT INTO `sys_menu` VALUES (65, '菜单2-1', 'menu2-1', 64, 0, '/menu/nested/menu2/menu2-1', '', 0, 1, 'C', 1, '&#xe676;', '菜单2-1', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:25:23', 'admin', '2025-02-20 23:22:07', NULL, 0);
INSERT INTO `sys_menu` VALUES (66, '菜单3', 'menu3', 62, 2, '/menu/nested/menu3', '', 0, 1, 'C', 1, '&#xe676;', '菜单3', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:26:09', 'admin', '2025-02-20 23:22:07', NULL, 0);
INSERT INTO `sys_menu` VALUES (67, '菜单3-1', 'menu3-1', 66, 0, '/menu/nested/menu3/menu3-1', '', 0, 1, 'C', 1, '&#xe676', '菜单3-1', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:32:12', 'admin', '2025-02-20 23:22:07', NULL, 0);
INSERT INTO `sys_menu` VALUES (68, '菜单3-2', 'menu3-2', 66, 1, '/menu/nested/menu3/menu3-2', '', 0, 1, 'C', 1, '&#xe676', '菜单3-2', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:35:06', 'admin', '2025-02-20 23:22:07', NULL, 0);
INSERT INTO `sys_menu` VALUES (69, '菜单3-2-1', 'menu3-2-1', 68, 0, '/menu/nested/menu3/menu3-2/menu3-2-1', '', 0, 1, 'C', 1, '&#xe676', '菜单3-2-1', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:35:42', 'admin', '2025-02-20 23:22:07', NULL, 0);
INSERT INTO `sys_menu` VALUES (70, '结果页面', 'Result page', 0, 6, '/result', 'Result', 0, 1, 'M', 1, '&#xe715', '结果页面', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:37:22', 'admin', '2025-02-20 23:22:07', 'sys:result:*', 0);
INSERT INTO `sys_menu` VALUES (71, '成功页', 'Success page', 70, 0, '/result/success', '', 0, 1, 'C', 1, '', '成功页', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:37:57', 'admin', '2025-02-20 23:22:07', 'sys:result:success', 0);
INSERT INTO `sys_menu` VALUES (72, '失败页', 'Fail page', 70, 1, '/result/fail', '', 0, 1, 'C', 1, '', '失败页', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:38:25', 'admin', '2025-02-20 23:22:07', 'sys:result:fail', 0);
INSERT INTO `sys_menu` VALUES (73, '异常页面', 'Exception', 0, 7, '/exception', 'Exception', 0, 1, 'M', 1, '&#xe820;', '异常页面', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:40:54', 'admin', '2025-02-20 23:22:07', 'sys:exception:*', 0);
INSERT INTO `sys_menu` VALUES (74, '403', '403', 73, 0, '/exception/403', '', 0, 1, 'C', 1, '', '403', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:41:43', 'admin', '2025-02-20 23:22:07', 'sys:exception:403', 0);
INSERT INTO `sys_menu` VALUES (75, '404', '404', 73, 1, '/exception/404', '', 0, 1, 'C', 1, '', '404', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:42:52', 'admin', '2025-02-20 23:22:07', 'sys:exception:404', 0);
INSERT INTO `sys_menu` VALUES (76, '500', '500', 73, 2, '/exception/500', '', 0, 1, 'C', 1, '', '500', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:43:42', 'admin', '2025-02-20 23:22:07', 'sys:exception:500', 0);
INSERT INTO `sys_menu` VALUES (77, '系统设置', 'System setting', 0, 8, '/system', 'System', 0, 1, 'M', 1, '&#xe7b9', '系统设置', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:44:22', 'admin', '2025-02-20 23:22:07', 'sys:system:*', 0);
INSERT INTO `sys_menu` VALUES (78, '系统设置', 'System setting', 77, 0, '/system/setting', '', 0, 1, 'C', 1, '', '系统设置', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:44:37', 'admin', '2025-02-20 23:22:07', 'sys:system:setting', 0);
INSERT INTO `sys_menu` VALUES (79, 'API管理', 'API manguage', 77, 1, '/system/api', '', 0, 1, 'C', 1, '', 'API管理', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:45:32', 'admin', '2025-02-20 23:22:07', 'sys:system:api', 0);
INSERT INTO `sys_menu` VALUES (80, '系统日志', 'System log', 77, 2, '/system/log', '', 0, 1, 'C', 1, '', '系统日志', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:46:03', 'admin', '2025-02-20 23:22:07', 'sys:system:log', 0);
INSERT INTO `sys_menu` VALUES (81, '运维管理', 'Safeguard', 0, 9, '/safeguard', 'Safeguard', 0, 1, 'M', 1, '&#xe816', '运维管理', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:47:43', 'admin', '2025-02-20 23:22:07', 'sys:safeguard:*', 0);
INSERT INTO `sys_menu` VALUES (82, '服务器管理', 'Server manguage', 81, 0, '/safeguard/server', '', 0, 1, 'C', 1, '', '服务器管理', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:48:26', 'admin', '2025-02-20 23:22:07', 'sys:safeguard:server', 0);
INSERT INTO `sys_menu` VALUES (83, '版本计划', 'Version Plan', 0, 10, '/plan', 'Plan', 0, 1, 'M', 1, '&#xe712;', '版本计划', 1, NULL, 1, NULL, 'admin', '2025-02-17 22:48:49', 'admin', '2025-02-20 23:22:07', 'sys:splan:*', 0);
INSERT INTO `sys_menu` VALUES (84, '帮助中心', 'Update Plan', 0, 11, '', '', 1, 0, 'M', 1, '&#xe6c8;', '更新日志', 1, NULL, 1, '', 'admin', '2025-02-17 22:49:27', '1', '2025-02-21 00:31:19', '', 0);
INSERT INTO `sys_menu` VALUES (86, '新增账号', 'add account', 55, 0, '', '', 0, 1, 'F', 1, '', '新增账号', 1, NULL, 1, NULL, 'admin', '2025-02-18 15:21:56', 'admin', '2025-02-20 23:22:07', 'sys:user:add', 0);
INSERT INTO `sys_menu` VALUES (87, '修改账号', 'edit account', 55, 0, '', '', 0, 1, 'F', 1, '', '修改账号', 1, NULL, 1, NULL, 'admin', '2025-02-18 15:21:56', 'admin', '2025-02-20 23:22:07', 'sys:user:edit', 0);
INSERT INTO `sys_menu` VALUES (88, '删除账号', 'del account', 55, 0, '', '', 0, 1, 'F', 1, '', '删除账号', 1, NULL, 1, NULL, 'admin', '2025-02-18 15:21:56', 'admin', '2025-02-20 23:22:07', 'sys:user:del', 0);
INSERT INTO `sys_menu` VALUES (90, '查看角色', 'view role', 57, 0, '', '', 0, 1, 'F', 1, '', '查看账号', 1, NULL, 1, NULL, 'admin', '2025-02-18 15:21:56', 'admin', '2025-02-20 23:22:07', 'sys:role:view', 0);
INSERT INTO `sys_menu` VALUES (91, '菜单权限-添加', 'menu prem', 60, 0, '', '', 0, 1, 'F', 1, '', '菜单权限-添加', 1, NULL, 1, NULL, 'admin', '2025-02-19 00:44:07', 'admin', '2025-02-20 23:22:07', 'add', 0);
INSERT INTO `sys_menu` VALUES (92, '菜单权限-修改', 'menu prem', 60, 0, '', '', 0, 1, 'F', 1, '', '菜单权限-修改', 1, NULL, 1, NULL, 'admin', '2025-02-19 00:44:07', 'admin', '2025-02-20 23:22:07', 'edit', 0);
INSERT INTO `sys_menu` VALUES (93, '菜单权限-删除', 'menu prem', 60, 0, '', '', 0, 1, 'F', 1, '', '菜单权限-删除', 1, NULL, 1, NULL, 'admin', '2025-02-19 00:44:07', 'admin', '2025-02-20 23:22:07', 'delete', 0);
INSERT INTO `sys_menu` VALUES (107, '查看用户', NULL, 55, 1, '', '', 1, 1, 'F', 1, '', '', NULL, NULL, NULL, '', '', '2025-02-19 19:29:07', '', '2025-02-21 00:31:22', 'sys:user:view', 0);
INSERT INTO `sys_menu` VALUES (152, '官方文档', NULL, 84, 0, '', '', 1, 1, 'C', 1, '', '', NULL, NULL, NULL, 'https://www.lingchen.kim/art-design-pro/docs/', '', '2025-02-20 19:07:50', '', '2025-02-21 00:31:22', '', 0);
INSERT INTO `sys_menu` VALUES (153, '更新日志', NULL, 83, 0, '/plan/log', '', 1, 1, 'C', 1, '', '', NULL, NULL, NULL, '', '', '2025-02-20 19:08:37', '', '2025-02-21 00:31:22', '', 0);
INSERT INTO `sys_menu` VALUES (200, '修改', NULL, 50, 0, '', '', 1, 1, 'F', 0, '', '', NULL, NULL, NULL, '', '', '2025-02-21 18:38:22', '', '2025-02-22 20:38:23', 'edit', 0);
INSERT INTO `sys_menu` VALUES (201, '文章分类', NULL, 49, 5, '/article/article-category', '', 1, 1, 'M', 1, '', '', NULL, NULL, NULL, '', '', '2025-02-21 23:44:23', '', '2025-02-23 17:31:37', 'sys:article:*', 0);
INSERT INTO `sys_menu` VALUES (202, '删除文章', NULL, 50, 1, '', '', 1, 1, 'F', 0, '', '', NULL, NULL, NULL, '', '', '2025-02-21 18:38:22', '', '2025-02-22 20:38:31', 'delete', 0);
INSERT INTO `sys_menu` VALUES (203, '评论', NULL, 51, 0, '', '', 1, 1, 'F', 0, '#', '评论', NULL, NULL, NULL, NULL, '', '2025-02-22 22:09:05', '', '2025-02-22 22:10:13', 'sys:comment:*', NULL);
INSERT INTO `sys_menu` VALUES (205, '修改评论', NULL, 51, 1, '', '', 1, 1, 'F', 0, '', '', NULL, NULL, NULL, '', '', '2025-02-22 22:18:42', '', '2025-02-22 22:18:42', 'edit', 0);
INSERT INTO `sys_menu` VALUES (206, '删除评论', NULL, 51, 1, '', '', 1, 1, 'F', 0, '', '', NULL, NULL, NULL, '', '', '2025-02-22 22:18:58', '', '2025-02-22 22:18:58', 'delete', 0);
INSERT INTO `sys_menu` VALUES (209, '文章分类', NULL, 201, 1, '', '', 1, 1, 'F', 0, '', '', NULL, NULL, NULL, '', '', '2025-02-23 17:33:12', '', '2025-02-23 17:33:12', 'sys:category:*', 0);

-- ----------------------------
-- Table structure for sys_role
-- ----------------------------
DROP TABLE IF EXISTS `sys_role`;
CREATE TABLE `sys_role`  (
  `role_id` bigint NOT NULL AUTO_INCREMENT COMMENT '角色唯一标识',
  `role_code` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色编码（英文唯一键）',
  `role_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '角色显示名称',
  `role_desc` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '角色描述',
  `order_num` int NULL DEFAULT 0 COMMENT '显示排序（升序）',
  `status` tinyint NULL DEFAULT 1 COMMENT '状态: 0=禁用, 1=启用',
  `create_time` datetime NULL DEFAULT CURRENT_TIMESTAMP COMMENT '记录创建时间',
  `update_time` datetime NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后更新时间',
  PRIMARY KEY (`role_id`) USING BTREE,
  UNIQUE INDEX `role_code`(`role_code` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 7 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '系统角色定义表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_role
-- ----------------------------
INSERT INTO `sys_role` VALUES (1, 'admin', '超级管理员', '拥有系统全部权限', 0, 1, '2025-02-14 18:46:45', '2025-02-17 13:47:09');
INSERT INTO `sys_role` VALUES (2, 'user', '普通用户', '基础功能使用权限', 0, 1, '2025-02-14 18:46:45', '2025-02-17 13:47:14');
INSERT INTO `sys_role` VALUES (3, 'guest', '访客', '只读权限', 0, 1, '2025-02-14 18:46:45', '2025-02-14 18:46:45');

-- ----------------------------
-- Table structure for sys_role_menu
-- ----------------------------
DROP TABLE IF EXISTS `sys_role_menu`;
CREATE TABLE `sys_role_menu`  (
  `role_id` bigint NOT NULL COMMENT '角色ID',
  `menu_id` bigint NOT NULL COMMENT '菜单ID',
  PRIMARY KEY (`role_id`, `menu_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '角色和菜单关联表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_role_menu
-- ----------------------------
INSERT INTO `sys_role_menu` VALUES (1, 24);
INSERT INTO `sys_role_menu` VALUES (1, 25);
INSERT INTO `sys_role_menu` VALUES (1, 26);
INSERT INTO `sys_role_menu` VALUES (1, 27);
INSERT INTO `sys_role_menu` VALUES (1, 28);
INSERT INTO `sys_role_menu` VALUES (1, 29);
INSERT INTO `sys_role_menu` VALUES (1, 30);
INSERT INTO `sys_role_menu` VALUES (1, 31);
INSERT INTO `sys_role_menu` VALUES (1, 32);
INSERT INTO `sys_role_menu` VALUES (1, 33);
INSERT INTO `sys_role_menu` VALUES (1, 34);
INSERT INTO `sys_role_menu` VALUES (1, 35);
INSERT INTO `sys_role_menu` VALUES (1, 36);
INSERT INTO `sys_role_menu` VALUES (1, 37);
INSERT INTO `sys_role_menu` VALUES (1, 38);
INSERT INTO `sys_role_menu` VALUES (1, 39);
INSERT INTO `sys_role_menu` VALUES (1, 40);
INSERT INTO `sys_role_menu` VALUES (1, 41);
INSERT INTO `sys_role_menu` VALUES (1, 42);
INSERT INTO `sys_role_menu` VALUES (1, 43);
INSERT INTO `sys_role_menu` VALUES (1, 44);
INSERT INTO `sys_role_menu` VALUES (1, 45);
INSERT INTO `sys_role_menu` VALUES (1, 46);
INSERT INTO `sys_role_menu` VALUES (1, 47);
INSERT INTO `sys_role_menu` VALUES (1, 48);
INSERT INTO `sys_role_menu` VALUES (1, 49);
INSERT INTO `sys_role_menu` VALUES (1, 50);
INSERT INTO `sys_role_menu` VALUES (1, 51);
INSERT INTO `sys_role_menu` VALUES (1, 52);
INSERT INTO `sys_role_menu` VALUES (1, 53);
INSERT INTO `sys_role_menu` VALUES (1, 54);
INSERT INTO `sys_role_menu` VALUES (1, 55);
INSERT INTO `sys_role_menu` VALUES (1, 57);
INSERT INTO `sys_role_menu` VALUES (1, 58);
INSERT INTO `sys_role_menu` VALUES (1, 59);
INSERT INTO `sys_role_menu` VALUES (1, 60);
INSERT INTO `sys_role_menu` VALUES (1, 61);
INSERT INTO `sys_role_menu` VALUES (1, 62);
INSERT INTO `sys_role_menu` VALUES (1, 63);
INSERT INTO `sys_role_menu` VALUES (1, 64);
INSERT INTO `sys_role_menu` VALUES (1, 65);
INSERT INTO `sys_role_menu` VALUES (1, 66);
INSERT INTO `sys_role_menu` VALUES (1, 67);
INSERT INTO `sys_role_menu` VALUES (1, 68);
INSERT INTO `sys_role_menu` VALUES (1, 69);
INSERT INTO `sys_role_menu` VALUES (1, 70);
INSERT INTO `sys_role_menu` VALUES (1, 71);
INSERT INTO `sys_role_menu` VALUES (1, 72);
INSERT INTO `sys_role_menu` VALUES (1, 73);
INSERT INTO `sys_role_menu` VALUES (1, 74);
INSERT INTO `sys_role_menu` VALUES (1, 75);
INSERT INTO `sys_role_menu` VALUES (1, 76);
INSERT INTO `sys_role_menu` VALUES (1, 77);
INSERT INTO `sys_role_menu` VALUES (1, 78);
INSERT INTO `sys_role_menu` VALUES (1, 79);
INSERT INTO `sys_role_menu` VALUES (1, 80);
INSERT INTO `sys_role_menu` VALUES (1, 81);
INSERT INTO `sys_role_menu` VALUES (1, 82);
INSERT INTO `sys_role_menu` VALUES (1, 83);
INSERT INTO `sys_role_menu` VALUES (1, 84);
INSERT INTO `sys_role_menu` VALUES (1, 86);
INSERT INTO `sys_role_menu` VALUES (1, 87);
INSERT INTO `sys_role_menu` VALUES (1, 88);
INSERT INTO `sys_role_menu` VALUES (1, 90);
INSERT INTO `sys_role_menu` VALUES (1, 91);
INSERT INTO `sys_role_menu` VALUES (1, 92);
INSERT INTO `sys_role_menu` VALUES (1, 93);
INSERT INTO `sys_role_menu` VALUES (1, 107);
INSERT INTO `sys_role_menu` VALUES (1, 152);
INSERT INTO `sys_role_menu` VALUES (1, 153);
INSERT INTO `sys_role_menu` VALUES (1, 200);
INSERT INTO `sys_role_menu` VALUES (1, 201);
INSERT INTO `sys_role_menu` VALUES (1, 202);
INSERT INTO `sys_role_menu` VALUES (1, 203);
INSERT INTO `sys_role_menu` VALUES (1, 205);
INSERT INTO `sys_role_menu` VALUES (1, 206);
INSERT INTO `sys_role_menu` VALUES (1, 209);
INSERT INTO `sys_role_menu` VALUES (2, 24);
INSERT INTO `sys_role_menu` VALUES (2, 25);
INSERT INTO `sys_role_menu` VALUES (2, 26);
INSERT INTO `sys_role_menu` VALUES (2, 27);
INSERT INTO `sys_role_menu` VALUES (2, 28);
INSERT INTO `sys_role_menu` VALUES (2, 29);
INSERT INTO `sys_role_menu` VALUES (2, 30);
INSERT INTO `sys_role_menu` VALUES (2, 31);
INSERT INTO `sys_role_menu` VALUES (2, 32);
INSERT INTO `sys_role_menu` VALUES (2, 33);
INSERT INTO `sys_role_menu` VALUES (2, 34);
INSERT INTO `sys_role_menu` VALUES (2, 35);
INSERT INTO `sys_role_menu` VALUES (2, 36);
INSERT INTO `sys_role_menu` VALUES (2, 37);
INSERT INTO `sys_role_menu` VALUES (2, 38);
INSERT INTO `sys_role_menu` VALUES (2, 39);
INSERT INTO `sys_role_menu` VALUES (2, 40);
INSERT INTO `sys_role_menu` VALUES (2, 41);

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user`  (
  `user_id` bigint NOT NULL AUTO_INCREMENT COMMENT '用户唯一标识',
  `username` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '登录账号（需唯一）',
  `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'BCrypt加密后的密码',
  `nickname` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '用户显示名称',
  `email` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '电子邮箱',
  `phone` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '手机号码（带国际区号）',
  `avatar` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '头像URL地址',
  `status` tinyint NULL DEFAULT 1 COMMENT '状态: 0=禁用, 1=启用',
  `last_login` datetime NULL DEFAULT NULL COMMENT '最后登录时间',
  `create_time` datetime NULL DEFAULT CURRENT_TIMESTAMP COMMENT '记录创建时间',
  `update_time` datetime NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后更新时间',
  `sex` int NULL DEFAULT NULL COMMENT '性别',
  PRIMARY KEY (`user_id`) USING BTREE,
  UNIQUE INDEX `username`(`username` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 15 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '系统用户基本信息表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_user
-- ----------------------------
INSERT INTO `sys_user` VALUES (1, 'admin', '$2b$10$1TxxtcZ/28jh2LYgOd11X.KG3gYtqg9zHHHDfNTHmT9WsFKIVOygq', '系统管理员', 'admin@example.com', '+8613812345678', 'https://rust-blogs.oss-cn-guangzhou.aliyuncs.com/image/2025/02/22/e4aa1482-82b7-4073-a2ab-5120993201ab.jpg', 1, '2025-02-23 17:33:19', '2025-02-14 18:46:45', '2025-02-23 17:33:19', 1);
INSERT INTO `sys_user` VALUES (3, 'zhangsan', '$2b$10$e1o4mrl3XCFD7Xn7TNQHVutV7tRCe7pDAmKAq8T/LI0YRdaIEhyta', '张三', NULL, NULL, NULL, 1, '2025-02-21 13:40:58', '2025-02-14 18:46:45', '2025-02-21 13:40:57', 1);
INSERT INTO `sys_user` VALUES (4, 'lisi', '$2b$10$1Zo4EYox1WW3W1boy3cNIehDOUfzCpvBWoGtyd5ZQ.gjSTZ0FYgmy', '李四', NULL, NULL, NULL, 0, '2025-02-19 16:24:43', '2025-02-14 18:46:45', '2025-02-19 16:24:43', 0);
INSERT INTO `sys_user` VALUES (14, 'test', '$2b$12$u8YugH.LbfbowMVnX/2ulejVGQnhT0H2o6S.QFrYZeIYOibPjvvWC', '', '', '18999999999', '/avatar/default.png', 1, '2025-02-21 13:50:08', '2025-02-19 16:44:32', '2025-02-21 13:50:07', 1);

-- ----------------------------
-- Table structure for sys_user_role
-- ----------------------------
DROP TABLE IF EXISTS `sys_user_role`;
CREATE TABLE `sys_user_role`  (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '关联记录ID',
  `user_id` bigint NOT NULL COMMENT '用户ID（逻辑外键->sys_user.user_id）',
  `role_id` bigint NOT NULL COMMENT '角色ID（逻辑外键->sys_role.role_id）',
  `create_time` datetime NULL DEFAULT CURRENT_TIMESTAMP COMMENT '授权时间',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `uk_user_role`(`user_id` ASC, `role_id` ASC) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 10 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '用户角色授权记录表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_user_role
-- ----------------------------
INSERT INTO `sys_user_role` VALUES (1, 1, 1, '2025-02-14 18:46:45');
INSERT INTO `sys_user_role` VALUES (2, 2, 3, '2025-02-14 18:46:45');
INSERT INTO `sys_user_role` VALUES (3, 3, 2, '2025-02-14 18:46:45');
INSERT INTO `sys_user_role` VALUES (9, 14, 3, '2025-02-20 15:59:29');

SET FOREIGN_KEY_CHECKS = 1;
