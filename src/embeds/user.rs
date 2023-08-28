use time::OffsetDateTime;
use twilight_model::channel::message::embed::EmbedAuthor;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{
    locales::{Locale, ToLocalized},
    scratch::{
        api,
        db::{
            self,
            user::{Statistics, Status},
        },
    },
};

use super::{timestamp, trim_field, Extend};

#[derive(Debug, Default)]
pub struct User {
    username: Option<String>,
    image: Option<String>,
    joined: Option<OffsetDateTime>,
    country: Option<String>,
    about: Option<String>,
    work: Option<String>,
    status: Option<Status>,
    school: Option<i64>,
    statistics: Option<Statistics>,
}

impl User {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Extend<api::User> for User {
    fn extend(&mut self, data: api::User) -> &mut Self {
        self.username = Some(data.username);
        self.image = Some(data.profile.images.n50x50);
        self.joined = Some(data.history.joined);
        self.country = data.profile.country;
        self.about = Some(data.profile.bio).filter(|s| !s.is_empty());
        self.work = Some(data.profile.status).filter(|s| !s.is_empty());
        self
    }
}

impl Extend<db::User> for User {
    fn extend(&mut self, data: db::User) -> &mut Self {
        self.status = data.status;
        self.school = data.school;
        self.statistics = data.statistics;
        self
    }
}

impl ToLocalized<EmbedBuilder> for User {
    fn to_localized(&self, locale: Locale) -> EmbedBuilder {
        let mut embed = EmbedBuilder::new();

        if let Some(username) = &self.username {
            embed = embed.author(EmbedAuthor {
                name: username.to_string(),
                url: Some(format!("https://scratch.mit.edu/users/{}", &username)),
                icon_url: self.image.as_ref().map(|value| value.to_string()),
                proxy_icon_url: None,
            });
        }

        if let Some(status) = &self.status {
            let status = status.to_localized(locale);
            let description = if let Some(school) = self.school {
                let school = format!("[{school}](https://scratch.mit.edu/classes/{school}/)");
                locale.status_student(&school, &status)
            } else {
                status
            };
            embed = embed.description(description);
        }

        if let Some(joined) = self.joined {
            embed = embed.field(EmbedFieldBuilder::new(
                locale.user_joined(),
                timestamp(joined),
            ));
        }

        if let Some(country) = &self.country {
            embed = embed.field(EmbedFieldBuilder::new(
                locale.country(),
                country.to_string(),
            ))
        }

        if let Some(about) = &self.about {
            embed = embed.field(EmbedFieldBuilder::new(locale.user_bio(), trim_field(about)));
        }

        if let Some(work) = &self.work {
            embed = embed.field(EmbedFieldBuilder::new(locale.user_work(), trim_field(work)));
        }

        if let Some(stats) = &self.statistics {
            embed = embed
                .field(stats.to_localized(locale).inline())
                .field(stats.ranks.to_localized(locale).inline())
                .field(stats.ranks.country.to_localized(locale).inline());
        }

        embed
    }
}

impl ToLocalized<String> for db::user::Status {
    fn to_localized(&self, locale: Locale) -> String {
        match self {
            Self::Scratcher => locale.status_scratcher(),
            Self::NewScratcher => locale.status_new_scratcher(),
            Self::TeacherAccount => locale.status_teacher(),
            Self::ScratchTeam => locale.status_scratch_team(),
        }
    }
}

impl ToLocalized<EmbedFieldBuilder> for db::user::Statistics {
    fn to_localized(&self, locale: Locale) -> EmbedFieldBuilder {
        let mut vec = Vec::with_capacity(6);

        if let Some(loves) = self.loves {
            vec.push(locale.stats_loves(&loves.to_string()));
        }
        if let Some(favorites) = self.favorites {
            vec.push(locale.stats_favorites(&favorites.to_string()));
        }
        if let Some(comments) = self.comments {
            vec.push(locale.stats_comments(&comments.to_string()));
        }
        if let Some(views) = self.views {
            vec.push(locale.stats_views(&views.to_string()));
        }

        vec.push(locale.stats_followers(&self.followers.to_string()));
        vec.push(locale.stats_following(&self.following.to_string()));

        let value = vec.join("\n");

        EmbedFieldBuilder::new(locale.stats(), value)
    }
}

impl ToLocalized<EmbedFieldBuilder> for db::user::Ranks {
    fn to_localized(&self, locale: Locale) -> EmbedFieldBuilder {
        let value = vec![
            locale.stats_loves(&self.loves.to_string()),
            locale.stats_favorites(&self.favorites.to_string()),
            locale.stats_comments(&self.comments.to_string()),
            locale.stats_views(&self.views.to_string()),
            locale.stats_followers(&self.followers.to_string()),
            locale.stats_following(&self.following.to_string()),
        ]
        .join("\n");

        EmbedFieldBuilder::new(locale.stats_ranks(), value)
    }
}

impl ToLocalized<EmbedFieldBuilder> for db::user::Country {
    fn to_localized(&self, locale: Locale) -> EmbedFieldBuilder {
        let value = vec![
            locale.stats_loves(&self.loves.to_string()),
            locale.stats_favorites(&self.favorites.to_string()),
            locale.stats_comments(&self.comments.to_string()),
            locale.stats_views(&self.views.to_string()),
            locale.stats_followers(&self.followers.to_string()),
            locale.stats_following(&self.following.to_string()),
        ]
        .join("\n");

        EmbedFieldBuilder::new(locale.stats_ranks_country(), value)
    }
}
