pub struct Url {
    feed_type: FeedTypes,
    limit: LimitType,
    genre: Genre,
    extension: ExtensionType,
}
impl Default for Url {
    fn default() -> Self {
        Self {
            feed_type: FeedTypes::TopGrossingApplications,
            limit: LimitType::Ten,
            genre: Genre::Game,
            extension: ExtensionType::Json,
        }
    }
}
impl ToString for Url {
    fn to_string(&self) -> String {
        let feed = self.feed_type.to_string();
        let limit = self.limit.to_string();
        let extension = self.extension.to_string();

        let url = format!(
            "http://itunes.apple.com/jp/rss/{}/{}",
            feed, limit
        );

        match self.genre.to_string().as_ref() {
            None => format!("{}/{}", url, extension),
            Some(genre) => format!("{}/{}/{}", url, genre, extension)
        }
    }
}

pub enum FeedTypes {
    TopFreeApplications,            // トップ無料 iPhone
    TopPaidApplications,            // トップ有料 iPhone
    TopGrossingApplications,        // トップセールス iPhone
    TopFreeIpadApplications,        // トップ無料 iPad
    TopPaidIpadApplications,        // トップ有料 iPad
    TopGrossingIpadApplications,    // トップセールス iPad
    NewApplications,                // 新規アプリ
    NewFreeApplications,            // 新規無料アプリ
    NewPaidApplications,            // 新規無料アプリ
}
impl ToString for FeedTypes {
    fn to_string(&self) -> String {
        let name = match self {
            FeedTypes::TopFreeApplications => "topfreeapplications",
            FeedTypes::TopPaidApplications => "toppaidapplications",
            FeedTypes::TopGrossingApplications => "topgrossingapplications",
            FeedTypes::TopFreeIpadApplications => "topfreeipadapplications",
            FeedTypes::TopPaidIpadApplications => "toppaidipadapplications",
            FeedTypes::TopGrossingIpadApplications => "topgrossingipadapplications",
            FeedTypes::NewApplications => "newapplications",
            FeedTypes::NewFreeApplications => "newfreeapplications",
            FeedTypes::NewPaidApplications => "newpaidapplications",
        };
        name.to_string()
    }
}

pub enum LimitType {
    Ten,
    TwentyFive,
    Fifty,
    OneHundred
}
impl ToString for LimitType {
    fn to_string(&self) -> String {
        let limit = match self {
            LimitType::Ten => 10,
            LimitType::TwentyFive => 25,
            LimitType::Fifty => 50,
            LimitType::OneHundred => 100,
        };
        format!("limit={}", limit)
    }
}

pub enum Genre {
    All,
    NewsStand,
    Entertainment,
    Game,
    Sports,
    SNS,
    Navigation,
    News,
    Business,
    Finance,
    Book,
    HealthCareFitness,
    Musical,
    Medical,
    Utility,
    LifeStyle,
    Productivity,
    PhotoVideo,
    Weather,
    Education,
    Travel,
    DictionaryAndOther
}
impl Genre {
    fn to_string(&self) -> Option<String> {
        let genre_num = match self {
            Genre::All => None,
            Genre::NewsStand => Some(6021),
            Genre::Entertainment => Some(6016),
            Genre::Game => Some(6014),
            Genre::Sports => Some(6004),
            Genre::SNS => Some(6005),
            Genre::Navigation => Some(6010),
            Genre::News => Some(6009),
            Genre::Business => Some(6000),
            Genre::Finance => Some(6015),
            Genre::Book => Some(6018),
            Genre::HealthCareFitness => Some(6013),
            Genre::Musical => Some(6011),
            Genre::Medical => Some(6020),
            Genre::Utility => Some(6002),
            Genre::LifeStyle => Some(6012),
            Genre::Productivity => Some(6007),
            Genre::PhotoVideo => Some(6008),
            Genre::Weather => Some(6001),
            Genre::Education => Some(6017),
            Genre::Travel => Some(6003),
            Genre::DictionaryAndOther => Some(6006)
        };
        genre_num.and_then(|num| Some(format!("genre={}", num)))
    }
}


pub enum ExtensionType {
    Json,
    XML,
}
impl ToString for ExtensionType {
    fn to_string(&self) -> String {
        let extension_name = match self {
            ExtensionType::Json => "json",
            ExtensionType::XML => "xml",
        };
        extension_name.to_string()
    }
}

