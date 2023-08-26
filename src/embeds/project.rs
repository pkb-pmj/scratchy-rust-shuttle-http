use twilight_model::channel::message::embed::EmbedAuthor;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::{
    locales::{Locale, ToLocalized},
    scratch::{
        api::{self, project::History},
        db::{self, project::Ranks},
    },
};

use super::{timestamp, Extend};

#[derive(Debug, Default)]
pub struct Project {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub instructions: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub public: Option<bool>,
    pub comments_allowed: Option<bool>,
    pub is_published: Option<bool>,
    pub image: Option<String>,
    pub remix: Option<Remix>,
    pub author: Option<Author>,
    pub history: Option<History>,
    pub statistics: Option<Statistics>,
}

#[derive(Debug)]
pub struct Remix {
    pub parent: i64,
    pub root: i64,
}

#[derive(Debug)]
pub struct Author {
    pub id: Option<i64>,
    pub username: String,
    pub image: Option<String>,
}

#[derive(Debug, Default)]
pub struct Statistics {
    pub views: i64,
    pub loves: i64,
    pub favorites: i64,
    pub remixes: Option<i64>,
    pub comments: Option<i64>,
    pub ranks: Option<Ranks>,
}

impl Project {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Extend<api::Project> for Project {
    fn extend(&mut self, data: api::Project) -> &mut Self {
        self.id = Some(data.id);
        self.title = Some(data.title);
        self.instructions = Some(data.instructions).filter(|s| !s.is_empty());
        self.description = Some(data.description).filter(|s| !s.is_empty());
        self.visibility = Some(data.visibility);
        self.public = Some(data.public);
        self.comments_allowed = Some(data.comments_allowed);
        self.is_published = Some(data.is_published);

        let author = self.author.get_or_insert(Author {
            id: None,
            username: data.author.username,
            image: None,
        });
        author.id = Some(data.author.id);
        author.image = Some(data.author.profile.images.n50x50);

        self.image = Some(data.image);

        self.history = Some(data.history);

        let statistics = self.statistics.get_or_insert(Default::default());
        statistics.views = data.stats.views;
        statistics.loves = data.stats.loves;
        statistics.favorites = data.stats.favorites;
        statistics.remixes = Some(data.stats.remixes);

        if let (Some(parent), Some(root)) = (data.remix.parent, data.remix.root) {
            self.remix = Some(Remix { parent, root });
        }

        self
    }
}

impl Extend<db::Project> for Project {
    fn extend(&mut self, data: db::Project) -> &mut Self {
        self.id = Some(data.id);

        self.author.get_or_insert(Author {
            id: None,
            username: data.username,
            image: None,
        });

        self.title = Some(data.title);
        self.instructions = Some(data.instructions).filter(|s| !s.is_empty());
        self.description = Some(data.description).filter(|s| !s.is_empty());
        self.public = Some(data.public);
        self.comments_allowed = Some(data.comments_allowed);

        self.history = Some(History {
            created: data.times.created,
            modified: data.times.modified,
            shared: data.times.shared,
        });

        if let (Some(parent), Some(root)) = (data.remix.parent, data.remix.root) {
            self.remix = Some(Remix { parent, root });
        }

        let statistics = self.statistics.get_or_insert(Default::default());
        statistics.views = data.statistics.views;
        statistics.loves = data.statistics.loves;
        statistics.favorites = data.statistics.favorites;
        statistics.comments = data.statistics.comments;
        statistics.ranks = Some(data.statistics.ranks);

        self
    }
}

impl ToLocalized<EmbedBuilder> for Project {
    fn to_localized(&self, locale: Locale) -> EmbedBuilder {
        let mut embed = EmbedBuilder::new();

        if let Some(author) = &self.author {
            embed = embed.author(author.to_localized(locale));
        }

        if let Some(title) = &self.title {
            embed = embed.title(title);
        }
        if let Some(id) = self.id {
            embed = embed.url(format!("https://scratch.mit.edu/projects/{}", id))
        }
        if let Some(image) = &self.image {
            embed = embed.image(ImageSource::url(image).unwrap());
        }
        if let Some(history) = &self.history {
            embed = embed.description(history.to_localized(locale));
        }

        if let Some(instructions) = &self.instructions {
            embed = embed.field(EmbedFieldBuilder::new(
                locale.project_instructions(),
                instructions.to_string(),
            ));
        }

        if let Some(description) = &self.description {
            embed = embed.field(EmbedFieldBuilder::new(
                locale.project_description(),
                description,
            ));
        }

        if let Some(stats) = &self.statistics {
            embed = embed.field(stats.to_localized(locale).inline());
            if let Some(ranks) = &stats.ranks {
                embed = embed.field(ranks.to_localized(locale).inline())
            }
        }

        embed
    }
}

impl ToLocalized<EmbedFieldBuilder> for Remix {
    fn to_localized(&self, locale: Locale) -> EmbedFieldBuilder {
        EmbedFieldBuilder::new(
            locale.remix_of(),
            format!("https://scratch.mit.edu/projects/{}", self.parent),
        )
    }
}

impl ToLocalized<EmbedAuthor> for Author {
    fn to_localized(&self, _locale: Locale) -> EmbedAuthor {
        EmbedAuthor {
            icon_url: self.image.to_owned(),
            name: self.username.to_owned(),
            proxy_icon_url: None,
            url: Some(format!("https://scratch.mit.edu/users/{}", &self.username)),
        }
    }
}

impl ToLocalized<String> for History {
    fn to_localized(&self, locale: Locale) -> String {
        vec![
            locale.project_created(&timestamp(self.created)),
            locale.project_modified(&timestamp(self.modified)),
            locale.project_shared(&timestamp(self.shared)),
        ]
        .join("\n")
    }
}

impl ToLocalized<EmbedFieldBuilder> for Statistics {
    fn to_localized(&self, locale: Locale) -> EmbedFieldBuilder {
        let mut vec = vec![
            locale.stats_views(&self.views.to_string()),
            locale.stats_loves(&self.loves.to_string()),
            locale.stats_favorites(&self.favorites.to_string()),
        ];

        if let Some(comments) = self.comments {
            vec.push(locale.stats_comments(&comments.to_string()));
        }

        if let Some(remixes) = self.remixes {
            vec.push(locale.stats_remixes(&remixes.to_string()));
        }

        let value = vec.join("\n");

        EmbedFieldBuilder::new(locale.stats(), value)
    }
}

impl ToLocalized<EmbedFieldBuilder> for Ranks {
    fn to_localized(&self, locale: Locale) -> EmbedFieldBuilder {
        let value = vec![
            locale.stats_loves(&self.loves.to_string()),
            locale.stats_favorites(&self.favorites.to_string()),
            locale.stats_views(&self.views.to_string()),
        ]
        .join("\n");

        EmbedFieldBuilder::new(locale.stats_ranks(), value)
    }
}
