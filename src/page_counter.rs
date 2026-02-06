use crate::text_data;

//Copyの内部処理にCloneが自動で付与されるようになっているが
//deriveのルール上Copy単体で書いてもCloneは使えない
//Cloneを覚えていないとCopyが使えない上位魔法だと思えばいい。

//ノベルリスト
#[derive(Copy, Clone)]
pub enum Novel {
    Novel1,
    Novel2,
    Novel3,
}

//挿絵リスト
#[derive(Copy, Clone)]
pub enum NovelImg {
    Novel1,
    Novel2,
    Novel3,
}

impl Novel {
    pub fn novel_page(&self) -> &'static [&'static str] {
        match self {
            Novel::Novel1 => &text_data::NOVEL1,
            Novel::Novel2 => &text_data::NOVEL2,
            Novel::Novel3 => &text_data::NOVEL3,
        }
    }
}

pub fn get_message(novel: Novel, count: usize) -> String {
    novel
        .novel_page()
        .get(count)
        .unwrap_or(&"Not found")
        .to_string()
}

impl NovelImg {
    pub fn nimgpath(&self, count: usize) -> Option<&'static str> {
        match self {
            NovelImg::Novel1 => match count {
                1 => Some("images/illusts/ouch.webp"),
                2 => Some("images/illusts/doup.webp"),
                5 => Some("images/illusts/ikuzo.webp"),
                _ => None,
            },
            NovelImg::Novel2 => match count {
                0 => Some("images/illusts/zenbuyaku.webp"),
                1 => Some("images/illusts/nattsuya-de.webp"),
                2 => Some("images/illusts/hakkou.webp"),
                3 => Some("images/illusts/otosan.webp"),
                4 => Some("images/illusts/neji.webp"),
                5 => Some("images/illusts/takeuma.webp"),
                _ => None,
            },
            NovelImg::Novel3 => match count {
                0 => Some("images/illusts/aderi-dayo.webp"),
                1 => Some("images/illusts/yattimatta.webp"),
                2 => Some("images/illusts/kaikyuu.webp"),
                3 => Some("images/illusts/doba-.webp"),
                4 => Some("images/illusts/wristband.webp"),
                5 => Some("images/illusts/ending.webp"),
                _ => None,
            },
        }
    }
}
