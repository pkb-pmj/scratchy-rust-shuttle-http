use std::collections::HashMap;

use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Project {
    pub id: i64,
    pub sys_id: i64,
    pub username: String,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub public: bool,
    pub comments_allowed: bool,
    pub times: Times,
    pub remix: Remix,
    pub statistics: Statistics,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Times {
    #[serde(with = "time::serde::iso8601")]
    pub created: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub modified: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub shared: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub last_check: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub last_metadata_check: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Remix {
    pub parent: Option<i64>,
    pub root: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Statistics {
    pub ranks: Ranks,
    pub views: i64,
    pub loves: i64,
    pub favorites: i64,
    pub comments: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Ranks {
    pub views: i64,
    pub loves: i64,
    pub favorites: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Metadata {
    pub version: i64,
    pub costumes: i64,
    pub blocks: i64,
    pub variables: i64,
    pub assets: i64,
    pub hash: String,
    pub user_agent: String,
    pub history: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn ski_jumping() {
        let str = r#"{
            "id": 499038694,
            "sys_id": 42938998,
            "username": "PMJ_Studio",
            "title": "Skoki Narciarskie 4 | Ski Jumping",
            "description": "EN\n> Choose a jumping hill from the list, or paste a code from Jumping Hill Editor: https://scratch.mit.edu/projects/495285421/\n\n> Choose the gate.\n> Click to start, and then at the right moment to take off.\nNote: on mobile hold a little longer, it might not work instantly.\n\n> In the air and after landing steer with mouse/finger.\nup - the jumper leans forward, assumes the in-air position\ndown - the jumper straightens up and prepares for landing (at the bottom of the screen - telemark)\nNote: works only with mouse down.\n\nC - switches the camera (0 - auto TV camera)\nD - shows variables",
            "instructions": "PL\n> Wybierz skocznię z listy, albo wczytaj kod z Edytora Skoczni: https://scratch.mit.edu/projects/495285421/\n\n> Ustaw belkę startową.\n> Kliknij żeby wystartować, a potem w odpowiednim momencie żeby się wybić.\nUwaga: na telefonie przytrzymaj ułamek sekundy dłużej, czasem może nie zadziałać od razu.\n\n> W locie i po lądowaniu steruj myszką/palcem.\nwyżej - skoczek się pochyla, ustawia w pozycji do lotu\nniżej - skoczek się prostuje i przyjmuje pozycję do lądowania (na samym dole - telemark)\nUwaga: działa tylko kiedy przycisk myszy jest wciśnięty.\n\nC - przełącza kamerę (0 - automatyczna kamera telewizyjna)\nD - pokazuje zmienne\n\nJeżeli wolno działa, albo chcesz po prostu grać w lepszej jakości:\nhttps://turbowarp.org/499038694/fullscreen?hqpen\n\nSkoki Narciarskie 3 były przede wszystkim zrobione na zupełnie nowym silniku fizyki i 3D, tutaj skupiłem się na dopracowaniu tego, naprawieniu błędów, lepszym przystosowaniu do dalszego rozwoju i podstawowych funkcjach potrzebnych do grania.\n\nInne wersje Skoków Narciarskich:\n1 - https://scratch.mit.edu/projects/294866617/\n2 - https://scratch.mit.edu/projects/369357394/\n3 - https://scratch.mit.edu/projects/419973140/\n\n#pmj #gra #game #skoki #loty #narciarskie #ski #jumping #flying #skispringen #3d #physics #aerodynamics",
            "public": true,
            "comments_allowed": true,
            "times": {
                "created": "2021-03-09T21:20:14.000Z",
                "modified": "2022-08-25T11:53:32.000Z",
                "shared": "2021-03-25T08:32:24.000Z",
                "last_check": "2023-08-09T16:57:05.000Z",
                "last_metadata_check": "2022-10-29T12:52:48.000Z"
            },
            "remix": {
                "parent": null,
                "root": null
            },
            "statistics": {
                "ranks": {
                    "views": 45526,
                    "loves": 30497,
                    "favorites": 28838
                },
                "views": 10736,
                "loves": 503,
                "favorites": 439,
                "comments": null
            },
            "metadata": {
                "version": 3,
                "costumes": 9,
                "blocks": 1930,
                "variables": 58,
                "assets": 9,
                "hash": "356c1bd1ad2bf590d87977a3eb4fae11",
                "user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:104.0) Gecko/20100101 Firefox/104.0",
                "history": {
                    "2021-04-13T10:19:54.000Z": "0a7ba70f8faf0d4f427a7ef021b3e70e",
                    "2021-03-30T20:09:05.000Z": "0f001d00cdb8176bf304a6f1469960f5",
                    "2022-08-25T11:53:32.000Z": "356c1bd1ad2bf590d87977a3eb4fae11",
                    "2021-12-29T16:51:33.000Z": "9e068484c9cf317f88a4d4a04b4fbbc4",
                    "2021-12-25T20:43:48.000Z": "a2943ccae095bbd8d3b3f6c12e9760cf",
                    "2021-03-25T10:46:47.000Z": "dca57ed06ea517a965632d31b61a3b21"
                }
            }
        }"#;

        let expected = Project {
            id: 499038694,
            sys_id: 42938998,
            username: "PMJ_Studio".into(),
            title: "Skoki Narciarskie 4 | Ski Jumping".into(),
            description: "EN\n> Choose a jumping hill from the list, or paste a code from Jumping Hill Editor: https://scratch.mit.edu/projects/495285421/\n\n> Choose the gate.\n> Click to start, and then at the right moment to take off.\nNote: on mobile hold a little longer, it might not work instantly.\n\n> In the air and after landing steer with mouse/finger.\nup - the jumper leans forward, assumes the in-air position\ndown - the jumper straightens up and prepares for landing (at the bottom of the screen - telemark)\nNote: works only with mouse down.\n\nC - switches the camera (0 - auto TV camera)\nD - shows variables".into(),
            instructions: "PL\n> Wybierz skocznię z listy, albo wczytaj kod z Edytora Skoczni: https://scratch.mit.edu/projects/495285421/\n\n> Ustaw belkę startową.\n> Kliknij żeby wystartować, a potem w odpowiednim momencie żeby się wybić.\nUwaga: na telefonie przytrzymaj ułamek sekundy dłużej, czasem może nie zadziałać od razu.\n\n> W locie i po lądowaniu steruj myszką/palcem.\nwyżej - skoczek się pochyla, ustawia w pozycji do lotu\nniżej - skoczek się prostuje i przyjmuje pozycję do lądowania (na samym dole - telemark)\nUwaga: działa tylko kiedy przycisk myszy jest wciśnięty.\n\nC - przełącza kamerę (0 - automatyczna kamera telewizyjna)\nD - pokazuje zmienne\n\nJeżeli wolno działa, albo chcesz po prostu grać w lepszej jakości:\nhttps://turbowarp.org/499038694/fullscreen?hqpen\n\nSkoki Narciarskie 3 były przede wszystkim zrobione na zupełnie nowym silniku fizyki i 3D, tutaj skupiłem się na dopracowaniu tego, naprawieniu błędów, lepszym przystosowaniu do dalszego rozwoju i podstawowych funkcjach potrzebnych do grania.\n\nInne wersje Skoków Narciarskich:\n1 - https://scratch.mit.edu/projects/294866617/\n2 - https://scratch.mit.edu/projects/369357394/\n3 - https://scratch.mit.edu/projects/419973140/\n\n#pmj #gra #game #skoki #loty #narciarskie #ski #jumping #flying #skispringen #3d #physics #aerodynamics".into(),
            public: true,
            comments_allowed: true,
            times: Times {
                created: datetime!(2021-03-09 21:20:14.000 UTC),
                modified: datetime!(2022-08-25 11:53:32.000 UTC),
                shared: datetime!(2021-03-25 08:32:24.000 UTC),
                last_check: datetime!(2023-08-09 16:57:05.000 UTC),
                last_metadata_check: datetime!(2022-10-29 12:52:48.000 UTC),
            },
            remix: Remix {
                parent: None,
                root: None,
            },
            statistics: Statistics {
                ranks: Ranks {
                    views: 45526,
                    loves: 30497,
                    favorites: 28838,
                },
                views: 10736,
                loves: 503,
                favorites: 439,
                comments: None,
            },
            metadata: Metadata {
                version: 3,
                costumes: 9,
                blocks: 1930,
                variables: 58,
                assets: 9,
                hash: "356c1bd1ad2bf590d87977a3eb4fae11".into(),
                user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:104.0) Gecko/20100101 Firefox/104.0".into(),
                history: HashMap::from([
                    (
                        "2021-04-13T10:19:54.000Z".into(),
                        "0a7ba70f8faf0d4f427a7ef021b3e70e".into(),
                    ),
                    (
                        "2021-03-30T20:09:05.000Z".into(),
                        "0f001d00cdb8176bf304a6f1469960f5".into(),
                    ),
                    (
                        "2022-08-25T11:53:32.000Z".into(),
                        "356c1bd1ad2bf590d87977a3eb4fae11".into(),
                    ),
                    (
                        "2021-12-29T16:51:33.000Z".into(),
                        "9e068484c9cf317f88a4d4a04b4fbbc4".into(),
                    ),
                    (
                        "2021-12-25T20:43:48.000Z".into(),
                        "a2943ccae095bbd8d3b3f6c12e9760cf".into(),
                    ),
                    (
                        "2021-03-25T10:46:47.000Z".into(),
                        "dca57ed06ea517a965632d31b61a3b21".into(),
                    ),
                ]),
            },
        };

        let actual: Project = serde_json::from_str(str).unwrap();

        assert_eq!(actual, expected);
    }
}
