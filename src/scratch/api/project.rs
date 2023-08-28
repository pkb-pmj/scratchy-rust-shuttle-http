use serde::Deserialize;
use time::OffsetDateTime;

use super::user;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub visibility: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub is_published: bool,
    pub author: Author,
    pub image: String,
    pub images: Images,
    pub history: History,
    pub stats: Stats,
    pub remix: Remix,
    pub project_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Author {
    pub id: i64,
    pub username: String,
    pub scratchteam: bool,
    pub history: user::History,
    pub profile: Profile,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Profile {
    pub id: Option<i64>,
    pub images: user::Images,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Images {
    #[serde(rename = "282x218")]
    pub n282x218: String,
    #[serde(rename = "216x163")]
    pub n216x163: String,
    #[serde(rename = "200x200")]
    pub n200x200: String,
    #[serde(rename = "144x108")]
    pub n144x108: String,
    #[serde(rename = "135x102")]
    pub n135x102: String,
    #[serde(rename = "100x80")]
    pub n100x80: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct History {
    #[serde(with = "time::serde::iso8601")]
    pub created: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub modified: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub shared: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Stats {
    pub views: i64,
    pub loves: i64,
    pub favorites: i64,
    pub remixes: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Remix {
    pub parent: Option<i64>,
    pub root: Option<i64>,
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn ski_jumping() {
        let str = r#"{
            "id": 499038694,
            "title": "Skoki Narciarskie 4 | Ski Jumping",
            "description": "EN\n> Choose a jumping hill from the list, or paste a code from Jumping Hill Editor: https://scratch.mit.edu/projects/495285421/\n\n> Choose the gate.\n> Click to start, and then at the right moment to take off.\nNote: on mobile hold a little longer, it might not work instantly.\n\n> In the air and after landing steer with mouse/finger.\nup - the jumper leans forward, assumes the in-air position\ndown - the jumper straightens up and prepares for landing (at the bottom of the screen - telemark)\nNote: works only with mouse down.\n\nC - switches the camera (0 - auto TV camera)\nD - shows variables",
            "instructions": "PL\n> Wybierz skocznię z listy, albo wczytaj kod z Edytora Skoczni: https://scratch.mit.edu/projects/495285421/\n\n> Ustaw belkę startową.\n> Kliknij żeby wystartować, a potem w odpowiednim momencie żeby się wybić.\nUwaga: na telefonie przytrzymaj ułamek sekundy dłużej, czasem może nie zadziałać od razu.\n\n> W locie i po lądowaniu steruj myszką/palcem.\nwyżej - skoczek się pochyla, ustawia w pozycji do lotu\nniżej - skoczek się prostuje i przyjmuje pozycję do lądowania (na samym dole - telemark)\nUwaga: działa tylko kiedy przycisk myszy jest wciśnięty.\n\nC - przełącza kamerę (0 - automatyczna kamera telewizyjna)\nD - pokazuje zmienne\n\nJeżeli wolno działa, albo chcesz po prostu grać w lepszej jakości:\nhttps://turbowarp.org/499038694/fullscreen?hqpen\n\nSkoki Narciarskie 3 były przede wszystkim zrobione na zupełnie nowym silniku fizyki i 3D, tutaj skupiłem się na dopracowaniu tego, naprawieniu błędów, lepszym przystosowaniu do dalszego rozwoju i podstawowych funkcjach potrzebnych do grania.\n\nInne wersje Skoków Narciarskich:\n1 - https://scratch.mit.edu/projects/294866617/\n2 - https://scratch.mit.edu/projects/369357394/\n3 - https://scratch.mit.edu/projects/419973140/\n\n#pmj #gra #game #skoki #loty #narciarskie #ski #jumping #flying #skispringen #3d #physics #aerodynamics",
            "visibility": "visible",
            "public": true,
            "comments_allowed": true,
            "is_published": true,
            "author": {
                "id": 42178181,
                "username": "PMJ_Studio",
                "scratchteam": false,
                "history": {
                    "joined": "1900-01-01T00:00:00.000Z"
                },
                "profile": {
                    "id": null,
                    "images": {
                        "90x90": "https://cdn2.scratch.mit.edu/get_image/user/42178181_90x90.png?v=",
                        "60x60": "https://cdn2.scratch.mit.edu/get_image/user/42178181_60x60.png?v=",
                        "55x55": "https://cdn2.scratch.mit.edu/get_image/user/42178181_55x55.png?v=",
                        "50x50": "https://cdn2.scratch.mit.edu/get_image/user/42178181_50x50.png?v=",
                        "32x32": "https://cdn2.scratch.mit.edu/get_image/user/42178181_32x32.png?v="
                    }
                }
            },
            "image": "https://cdn2.scratch.mit.edu/get_image/project/499038694_480x360.png",
            "images": {
                "282x218": "https://cdn2.scratch.mit.edu/get_image/project/499038694_282x218.png?v=1661428412",
                "216x163": "https://cdn2.scratch.mit.edu/get_image/project/499038694_216x163.png?v=1661428412",
                "200x200": "https://cdn2.scratch.mit.edu/get_image/project/499038694_200x200.png?v=1661428412",
                "144x108": "https://cdn2.scratch.mit.edu/get_image/project/499038694_144x108.png?v=1661428412",
                "135x102": "https://cdn2.scratch.mit.edu/get_image/project/499038694_135x102.png?v=1661428412",
                "100x80": "https://cdn2.scratch.mit.edu/get_image/project/499038694_100x80.png?v=1661428412"
            },
            "history": {
                "created": "2021-03-09T21:20:14.000Z",
                "modified": "2022-08-25T11:53:32.000Z",
                "shared": "2021-03-25T08:32:24.000Z"
            },
            "stats": {
                "views": 10757,
                "loves": 503,
                "favorites": 439,
                "remixes": 26
            },
            "remix": {
                "parent": null,
                "root": null
            },
            "project_token": "1693219125_d616c4bbcdc789779229b8a2c4f89380ff309ea5479b0f5cc51aaa39256f9914b93f29e79f9a445b22a32bb023638151ddc0290771aee4c035624d8f7027a6e4"
        }"#;

        let expected = Project {
            id: 499038694,
            title: "Skoki Narciarskie 4 | Ski Jumping".into(),
            description: "EN\n> Choose a jumping hill from the list, or paste a code from Jumping Hill Editor: https://scratch.mit.edu/projects/495285421/\n\n> Choose the gate.\n> Click to start, and then at the right moment to take off.\nNote: on mobile hold a little longer, it might not work instantly.\n\n> In the air and after landing steer with mouse/finger.\nup - the jumper leans forward, assumes the in-air position\ndown - the jumper straightens up and prepares for landing (at the bottom of the screen - telemark)\nNote: works only with mouse down.\n\nC - switches the camera (0 - auto TV camera)\nD - shows variables".into(),
            instructions: "PL\n> Wybierz skocznię z listy, albo wczytaj kod z Edytora Skoczni: https://scratch.mit.edu/projects/495285421/\n\n> Ustaw belkę startową.\n> Kliknij żeby wystartować, a potem w odpowiednim momencie żeby się wybić.\nUwaga: na telefonie przytrzymaj ułamek sekundy dłużej, czasem może nie zadziałać od razu.\n\n> W locie i po lądowaniu steruj myszką/palcem.\nwyżej - skoczek się pochyla, ustawia w pozycji do lotu\nniżej - skoczek się prostuje i przyjmuje pozycję do lądowania (na samym dole - telemark)\nUwaga: działa tylko kiedy przycisk myszy jest wciśnięty.\n\nC - przełącza kamerę (0 - automatyczna kamera telewizyjna)\nD - pokazuje zmienne\n\nJeżeli wolno działa, albo chcesz po prostu grać w lepszej jakości:\nhttps://turbowarp.org/499038694/fullscreen?hqpen\n\nSkoki Narciarskie 3 były przede wszystkim zrobione na zupełnie nowym silniku fizyki i 3D, tutaj skupiłem się na dopracowaniu tego, naprawieniu błędów, lepszym przystosowaniu do dalszego rozwoju i podstawowych funkcjach potrzebnych do grania.\n\nInne wersje Skoków Narciarskich:\n1 - https://scratch.mit.edu/projects/294866617/\n2 - https://scratch.mit.edu/projects/369357394/\n3 - https://scratch.mit.edu/projects/419973140/\n\n#pmj #gra #game #skoki #loty #narciarskie #ski #jumping #flying #skispringen #3d #physics #aerodynamics".into(),
            visibility: "visible".into(),
            public: true,
            comments_allowed: true,
            is_published: true,
            author: Author {
                id: 42178181,
                username: "PMJ_Studio".into(),
                scratchteam: false,
                history: user::History {
                    joined: datetime!(1900-01-01 00:00:00.000 UTC),
                },
                profile: Profile {
                    id: None,
                    images: user::Images {
                        n90x90: "https://cdn2.scratch.mit.edu/get_image/user/42178181_90x90.png?v=".into(),
                        n60x60: "https://cdn2.scratch.mit.edu/get_image/user/42178181_60x60.png?v=".into(),
                        n55x55: "https://cdn2.scratch.mit.edu/get_image/user/42178181_55x55.png?v=".into(),
                        n50x50: "https://cdn2.scratch.mit.edu/get_image/user/42178181_50x50.png?v=".into(),
                        n32x32: "https://cdn2.scratch.mit.edu/get_image/user/42178181_32x32.png?v=".into(),
                    },
                },
            },
            image: "https://cdn2.scratch.mit.edu/get_image/project/499038694_480x360.png".into(),
            images: Images {
                n282x218: "https://cdn2.scratch.mit.edu/get_image/project/499038694_282x218.png?v=1661428412".into(),
                n216x163: "https://cdn2.scratch.mit.edu/get_image/project/499038694_216x163.png?v=1661428412".into(),
                n200x200: "https://cdn2.scratch.mit.edu/get_image/project/499038694_200x200.png?v=1661428412".into(),
                n144x108: "https://cdn2.scratch.mit.edu/get_image/project/499038694_144x108.png?v=1661428412".into(),
                n135x102: "https://cdn2.scratch.mit.edu/get_image/project/499038694_135x102.png?v=1661428412".into(),
                n100x80: "https://cdn2.scratch.mit.edu/get_image/project/499038694_100x80.png?v=1661428412".into()
            },
            history: History {
                created: datetime!(2021-03-09 21:20:14.000 UTC),
                modified: datetime!(2022-08-25 11:53:32.000 UTC),
                shared: datetime!(2021-03-25 08:32:24.000 UTC),
            },
            stats: Stats {
                views: 10757,
                loves: 503,
                favorites: 439,
                remixes: 26,
            },
            remix: Remix {
                parent: None,
                root: None,
            },
            project_token: "1693219125_d616c4bbcdc789779229b8a2c4f89380ff309ea5479b0f5cc51aaa39256f9914b93f29e79f9a445b22a32bb023638151ddc0290771aee4c035624d8f7027a6e4".into(),
        };

        let actual: Project = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }
}
