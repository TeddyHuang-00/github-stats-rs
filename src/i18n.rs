use log::warn;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupportedLanguage {
    Chinese,
    English,
    French,
    German,
    Italian,
    Japanese,
    Korean,
    Portuguese,
    Russian,
    Spanish,
}

impl<S: AsRef<str>> From<S> for SupportedLanguage {
    fn from(s: S) -> Self {
        // Normalize input locale string
        let s = s.as_ref().to_lowercase().replace('_', "-");
        match s.as_str() {
            "chinese" | "zh" | "zh-cn" | "zh-hans" => Self::Chinese,
            "french" | "fr" => Self::French,
            "german" | "de" => Self::German,
            "italian" | "it" => Self::Italian,
            "japanese" | "ja" => Self::Japanese,
            "korean" | "ko" => Self::Korean,
            "portuguese" | "pt" | "pt-br" => Self::Portuguese,
            "russian" | "ru" => Self::Russian,
            "spanish" | "es" => Self::Spanish,
            "english" | "en" => Self::English,
            _ => {
                warn!("Unrecognized language code '{s}', falling back to English");
                Self::English
            }
        }
    }
}

pub trait Internationalization {
    fn overview_title<S: AsRef<str>>(&self, name: S) -> String;
    fn language_title(&self) -> String;
    fn stars(&self) -> String;
    fn forks(&self) -> String;
    fn followers(&self) -> String;
    fn contributions(&self) -> String;
    fn views(&self) -> String;
    fn repositories(&self) -> String;
}

impl Internationalization for SupportedLanguage {
    fn overview_title<S: AsRef<str>>(&self, name: S) -> String {
        match self {
            Self::Chinese => "{{name}} 的 GitHub 统计信息",
            Self::English => "{{name}}'s GitHub Statistics",
            Self::French => "Statistiques GitHub de {{name}}",
            Self::German => "GitHub-Statistiken von {{name}}",
            Self::Italian => "Statistiche GitHub di {{name}}",
            Self::Japanese => "{{name}} の GitHub 統計情報",
            Self::Korean => "{{name}}의 GitHub 통계",
            Self::Portuguese => "Estatísticas do GitHub de {{name}}",
            Self::Russian => "Статистика GitHub пользователя {{name}}",
            Self::Spanish => "Estadísticas de GitHub de {{name}}",
        }
        .replace("{{name}}", name.as_ref())
    }

    fn language_title(&self) -> String {
        match self {
            Self::Chinese => "使用的语言（按文件大小）",
            Self::English => "Languages Used (By File Size)",
            Self::French => "Langages utilisés (par taille de fichier)",
            Self::German => "Verwendete Sprachen (nach Dateigröße)",
            Self::Italian => "Linguaggi Utilizzati (Per Dimensione del File)",
            Self::Japanese => "使用言語（ファイルサイズ別）",
            Self::Korean => "사용된 언어 (파일 크기별)",
            Self::Portuguese => "Linguagens Usadas (Por Tamanho do Arquivo)",
            Self::Russian => "Используемые языки (по размеру файла)",
            Self::Spanish => "Lenguajes utilizados (por tamaño de archivo)",
        }
        .into()
    }

    fn stars(&self) -> String {
        match self {
            Self::Chinese => "星标",
            Self::English => "Stars",
            Self::French => "Étoiles",
            Self::German => "Sterne",
            Self::Italian => "Stelle",
            Self::Japanese => "スター",
            Self::Korean => "스타",
            Self::Portuguese => "Estrelas",
            Self::Russian => "Звезды",
            Self::Spanish => "Estrellas",
        }
        .into()
    }

    fn forks(&self) -> String {
        match self {
            Self::Chinese => "分支",
            Self::English
            | Self::French
            | Self::German
            | Self::Italian
            | Self::Portuguese
            | Self::Spanish => "Forks",
            Self::Japanese => "フォーク",
            Self::Korean => "포크",
            Self::Russian => "Форки",
        }
        .into()
    }

    fn followers(&self) -> String {
        match self {
            Self::Chinese => "关注者",
            Self::English => "Followers",
            Self::French => "Abonnés",
            Self::German | Self::Italian => "Follower",
            Self::Japanese => "フォロワー",
            Self::Korean => "팔로워",
            Self::Portuguese | Self::Spanish => "Seguidores",
            Self::Russian => "Подписчики",
        }
        .into()
    }

    fn contributions(&self) -> String {
        match self {
            Self::Chinese => "累计贡献",
            Self::English => "All-time contributions",
            Self::French => "Contributions totales",
            Self::German => "Gesamtbeiträge",
            Self::Italian => "Contributi Totali",
            Self::Japanese => "累積貢献",
            Self::Korean => "누적 기여",
            Self::Portuguese => "Contribuições Totais",
            Self::Russian => "Всего вкладов",
            Self::Spanish => "Contribuciones totales",
        }
        .into()
    }

    fn views(&self) -> String {
        match self {
            Self::Chinese => "仓库浏览量（近两周）",
            Self::English => "Repository views (past two weeks)",
            Self::French => "Vues du dépôt (2 dernières semaines)",
            Self::German => "Aufrufe des Repositorys (letzte zwei Wochen)",
            Self::Italian => "Visualizzazioni del Repository (ultime due settimane)",
            Self::Japanese => "リポジトリの閲覧数（近2週間）",
            Self::Korean => "저장소 조회수 (지난 2주간)",
            Self::Portuguese => "Visualizações do Repositório (últimas duas semanas)",
            Self::Russian => "Просмотры репозитория (за последние две недели)",
            Self::Spanish => "Vistas del repositorio (últimas dos semanas)",
        }
        .into()
    }

    fn repositories(&self) -> String {
        match self {
            Self::Chinese => "有贡献的仓库数",
            Self::English => "Repositories with contributions",
            Self::French => "Dépôts avec contributions",
            Self::German => "Repositories mit Beiträgen",
            Self::Italian => "Repository con contributi",
            Self::Japanese => "貢献のあるリポジトリ数",
            Self::Korean => "기여한 저장소 수",
            Self::Portuguese => "Repositórios com contribuições",
            Self::Russian => "Репозитории с вкладом",
            Self::Spanish => "Repositorios con contribuciones",
        }
        .into()
    }
}
