-- ingredients
UPDATE terms SET reading='ブール', genre='dairy',
  notes='牛乳を撹拌して得られる乳脂肪分80%以上の固体。フランス料理の基礎的な脂肪素材。ノルマンディー産が名高い。塩分の有無でbeurre salé（有塩）とbeurre doux（無塩）に分かれる。'
WHERE french='beurre';

UPDATE terms SET reading='クレーム・フレッシュ', genre='dairy',
  notes='乳脂肪分30〜40%の発酵クリーム。乳酸菌による穏やかな酸味が特徴。加熱しても分離しにくく、ソース・スープ・デザートに多用される。イギリスのサワークリームより脂肪分が高い。'
WHERE french='crème fraîche';

UPDATE terms SET reading='ファリーヌ', genre='grain',
  notes='小麦を製粉した粉。フランスではType 45〜150の分類があり、数値が低いほど精白度が高い。T45はパティスリー向け、T55はバゲット向け。灰分量（ミネラル残留量）で型番が決まる。'
WHERE french='farine';

UPDATE terms SET reading='セル', genre='spice',
  notes='料理の基本調味料。フランスではゲランドの粗塩（gros sel de Guérande）やフルール・ド・セル（fleur de sel）が珍重される。ミネラルを豊富に含む天日塩が高品質とされる。'
WHERE french='sel';

UPDATE terms SET reading='ポワーヴル', genre='spice',
  notes='コショウ科植物の果実を乾燥させた香辛料。黒・白・緑・赤の種類がある。フランス料理では鴨・牛肉との相性が良い。ステーク・オ・ポワーヴル（コショウステーキ）が著名。'
WHERE french='poivre';

UPDATE terms SET reading='アイ', genre='vegetable',
  notes='ユリ科の球根野菜。南仏料理（プロヴァンス・ラングドック）に特に多用される。生・炒め・コンフィ・ロースト等、調理法により風味が大きく変わる。アリシンが独特の香りの元。'
WHERE french='ail';

UPDATE terms SET reading='エシャロット', genre='vegetable',
  notes='玉ねぎとニンニクの中間的な風味を持つ球根野菜。フランス料理において最重要の香味野菜の一つ。ソース・ベアルネーズやソース・ボルドレーズなどの古典ソースに不可欠。'
WHERE french='échalote';

UPDATE terms SET reading='タン', genre='herb',
  notes='シソ科の多年草ハーブ。地中海沿岸原産。ブーケ・ガルニの必須素材で、殺菌・防腐作用がある。煮込み料理・マリネ・フォン（出汁）に広く使われる。乾燥させても香りが持続する。'
WHERE french='thym';

UPDATE terms SET reading='ローリエ', genre='herb',
  notes='クスノキ科の常緑樹の葉（月桂樹）。ブーケ・ガルニの主要素材。生葉より乾燥葉の方が香りが安定する。フォン・ド・ヴォー（仔牛出汁）など全ての出汁に欠かせない。'
WHERE french='laurier';

UPDATE terms SET reading='ペルシ', genre='herb',
  notes='フランスパセリ（平葉・イタリアンパセリ）が主流。フィーヌ・エルブ（細かく刻んだハーブの混合）の一つ。クロロフィルが豊富で、風味付け・緑色の付与・飾りの三役をこなす。'
WHERE french='persil';

UPDATE terms SET reading='シャンピニョン', genre='mushroom',
  notes='食用キノコの総称。フランスではパリ産マッシュルーム（champignon de Paris / Agaricus bisporus）が最も一般的。デュクセル（みじん切り炒め）の主材料。水分が多いため高温で素早く炒める。'
WHERE french='champignon';

UPDATE terms SET reading='トリュフ', genre='mushroom',
  notes='地中に生育する高級食用菌。黒トリュフ（Tuber melanosporum、ペリゴール産）と白トリュフ（Tuber magnatum、アルバ産）が二大品種。土・腐葉土・ナッツを思わせる複雑な香り。オーク等の樹木と共生するため栽培が極めて困難。'
WHERE french='truffe';

UPDATE terms SET reading='フォワ・グラ', genre='protein',
  notes='ガチョウまたはアヒルの肥大した肝臓。強制給餌（ガヴァージュ）により生産。脂肪分が非常に高く（40〜60%）、濃厚な風味と滑らかな食感が特徴。フランス法により国家文化・美食遺産に指定。'
WHERE french='foie gras';

UPDATE terms SET reading='エスカルゴ', genre='protein',
  notes='食用カタツムリ。ブルゴーニュ種（Helix pomatia）が最高品種とされる。下処理後にニンニクバター（beurre d''escargot）とパセリを詰めて専用皿（escargotière）で焼く「エスカルゴ・ブルギニョン」が定番。'
WHERE french='escargot';

-- techniques
UPDATE terms SET reading='ソテ',
  notes='少量の油脂を高温に熱したフライパンで食材を素早く動かしながら炒める技法。「跳ばす」を意味するsauterに由来。短時間で表面にメイラード反応による焼き色をつけ、うま味を封じ込める。'
WHERE french='sauté';

UPDATE terms SET reading='ブレゼ',
  notes='食材を少量の液体（ワイン・フォン・ミルポワ）とともに密閉した鍋でオーブン蒸し煮する技法。長時間・低温（160〜180℃）でコラーゲンがゼラチン化し、柔らかく濃厚な風味になる。硬い肉（すね・頬）に最適。'
WHERE french='braisé';

UPDATE terms SET reading='ポシェ',
  notes='食材を沸騰させない湯（70〜85℃）でゆっくり加熱する技法。水分の多い液体（フォン・クール・ブイヨン）で卵・魚・鶏胸肉など繊細な食材を調理する。タンパク質が変性しすぎず、しっとりした食感を保てる。'
WHERE french='poché';

UPDATE terms SET reading='グリエ',
  notes='炭火または鋳物グリルで直火加熱する技法。高温（250〜300℃）で食材表面にメイラード反応と炭化による複雑な香ばしさと格子模様（quadrillage）を付ける。脂の多い肉・魚に適す。'
WHERE french='grillé';

UPDATE terms SET reading='ロティ',
  notes='オーブン内の対流・輻射熱で食材全体を均一に加熱する技法。調理中に肉汁・バターをかけながら（アロゼ）焼くことで表面は香ばしく内部はジューシーに仕上がる。温度管理と休ませ（repos）が重要。'
WHERE french='rôti';

UPDATE terms SET reading='フランベ',
  notes='調理中または仕上げにコニャック・ラム酒・カルバドスなどのアルコールを加えて点火する技法。アルコール臭を飛ばし、燃焼による高温でカラメル化が促進され、料理に独特の香りとコクを加える。'
WHERE french='flambé';

UPDATE terms SET reading='ジュリエンヌ',
  notes='食材を長さ5〜6cm、幅1〜2mmの細い棒状に切る技法。均一な形状により加熱ムラを防ぐ。スープのガルニチュール（具材）や前菜の付け合わせとして使われる。野菜・ハム・トリュフなど幅広く応用。'
WHERE french='julienne';

UPDATE terms SET reading='ブリュノワーズ',
  notes='食材を一辺2〜3mmの極小サイコロ状に切る技法。ジュリエンヌをさらに細断した形状。スープ・ソースへの風味付け、テリーヌの具材、料理の飾り（デコール）として使われる。精密な包丁さばきが必要。'
WHERE french='brunoise';

UPDATE terms SET reading='シフォナード',
  notes='バジル・ホウレン草・スイバ（oseille）などの葉物を重ねてロール状に巻き、細く千切りにする技法。サラダの飾り、スープやソースの仕上げに用いる。バジルはカットすると黒変するため、提供直前に行う。'
WHERE french='chiffonade';

UPDATE terms SET reading='デグラッセ',
  notes='食材を焼いた後の鍋底に残る焦げつき（スック、sucs）をワイン・コニャック・酢・フォンなどの液体で溶かし取る技法。旨味成分（グルタミン酸・メイラード生成物）をソースに取り込む上で最重要の工程。'
WHERE french='déglacer';

-- dishes
UPDATE terms SET reading='スープ・ア・ロニョン', genre='soup',
  notes='薄切り玉ねぎを長時間炒めてカラメル化させ（30〜60分）、ビーフフォンで煮たスープ。上にクルートン（焼いたバゲット）を浮かべグリュイエールチーズを乗せてオーブンで焼く（グラティネ）。パリのレ・アール（中央市場）の夜明け市に由来する庶民料理。'
WHERE french='soupe à l''oignon';

UPDATE terms SET reading='クレープ', genre='pastry',
  notes='小麦粉・卵・牛乳・溶かしバターで作る薄焼き生地。ブルターニュ地方発祥。甘いものはデザート用、そば粉（farine de sarrasin）で作るものはガレット（galette）と呼ばれ惣菜向け。クレープ・シュゼット（オレンジバターソース+フランベ）は著名なバリエーション。'
WHERE french='crêpe';

UPDATE terms SET reading='ラタトゥイユ', genre='stew',
  notes='プロヴァンス地方の代表的な野菜煮込み。ズッキーニ・ナス・パプリカ・トマト・玉ねぎ・ニンニクをオリーブオイルで炒め合わせる。各野菜を別々に炒めてから合わせる本式の方法が食感を保つ。温冷どちらでも供される。'
WHERE french='ratatouille';

UPDATE terms SET reading='ポトフ', genre='stew',
  notes='牛すね肉・骨付き肉と根菜（人参・かぶ・ポワロー・セロリ）をフォンで長時間ゆっくり煮込むフランス家庭料理の象徴。スープを先に供し、次に肉・野菜を芥子・コルニション（ピクルス）とともに盛る。「火の上の鍋」という意味。'
WHERE french='pot-au-feu';

UPDATE terms SET reading='カスレ', genre='stew',
  notes='白インゲン豆・鴨のコンフィ・トゥールーズ産ソーセージ・豚皮を長時間煮込んだラングドック地方の料理。カスソール（素焼きの土鍋）で仕上げにパン粉の焼き皮（クルート）を何度も作るのが伝統。カステルノーダリ・カルカソンヌ・トゥールーズが起源を争う。'
WHERE french='cassoulet';

UPDATE terms SET reading='コック・オー・ヴァン', genre='stew',
  notes='鶏肉を赤ワイン・マッシュルーム・パールオニオン・ラルドン（角切り塩漬け豚肉）とともに煮込む料理。ブルゴーニュ産ピノ・ノワールを使うのが本式。もともとは固い雄鶏（coq）を柔らかくするための農家料理。事前のマリネが風味の鍵。'
WHERE french='coq au vin';

UPDATE terms SET reading='タルト・タタン', genre='dessert',
  notes='リンゴを砂糖とバターでカラメリゼしてから生地（パート・ブリゼ）を被せてオーブンで焼き、型ごと逆さにして供するタルト。1880年代にロワール地方のラモット=ブーヴロンでタタン姉妹が偶然発明したとされる。リンゴはゴールデンやグラニースミスが適す。'
WHERE french='tarte tatin';

UPDATE terms SET reading='クレームブリュレ', genre='dessert',
  notes='卵黄・生クリーム・砂糖・バニラで作るカスタードプディング。湯煎でゆっくり火を入れた後、冷やし固め、表面にグラニュー糖を振ってバーナーで焦がし（ブリュレ）キャラメル層を作る。17世紀のフランス料理書に登場する古典菓子。'
WHERE french='crème brûlée';

UPDATE terms SET reading='キッシュ', genre='main',
  notes='パート・ブリゼ（ショートクラスト生地）にアパレイユ（卵・クリーム・塩・胡椒の混合液）を流し込んで焼くタルト。ロレーヌ地方発祥のキッシュ・ロレーヌ（ラルドン・グリュイエール入り）が原型。生地を盲目焼き（fonçage）してから具を入れるとサクサクに仕上がる。'
WHERE french='quiche';

-- utensils
UPDATE terms SET reading='ココット',
  notes='鋳鉄製の厚手鍋（ダッチオーブン）。密閉性が高く均一に蓄熱するため、ブレゼ・ロースト・スープ・パンの調理に最適。フランスではル・クルーゼ（Le Creuset）とストウブ（Staub）が二大ブランド。卵料理の小皿を指すこともある。'
WHERE french='cocotte';

UPDATE terms SET reading='マンドリン',
  notes='食材を均一な薄さにスライスする調理器具。刃の高さを調節して厚みを変えられる。グラタン・ダルフィノワのジャガイモスライスやフェンネルのラペなどに使用。刃が非常に鋭利なため切創防止グローブの着用が必須。'
WHERE french='mandoline';

UPDATE terms SET reading='タミ',
  notes='金属メッシュまたは馬毛を張った裏ごし器（太鼓状）。スープ・ピューレ・ソース・菓子生地を通すことで滑らかな食感を得る。動詞形は「タミゼ（tamiser）」。コニカル型のシノワ（chinois）は粗め用途に使い分ける。'
WHERE french='tamis';

UPDATE terms SET reading='フュエ',
  notes='ワイヤーを放射状に束ねた泡立て器。生クリームのホイップ、ソースへのバター混入（モンテ・オ・ブール）、卵白の撹拌に使用。バルーン型（球形、卵白向け）とソース型（細め、ソース用）の2種が主流。'
WHERE french='fouet';

UPDATE terms SET reading='スパチュール',
  notes='シリコン・ゴム製のへら。ボウルや鍋の残りを無駄なくすくう（マレルダー）用途や、生地を均すのに使う。熱耐性のあるシリコン製はソースの仕上げにも使用可。金属製オフセットスパチュラは展延・仕上げに使う別用途。'
WHERE french='spatule';
