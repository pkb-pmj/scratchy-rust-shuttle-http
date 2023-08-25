use twilight_model::channel::message::embed::EmbedAuthor;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{
    locales::{ExtendLocaleEmbed, Locale, ToLocalized},
    scratch::{api, db},
};

impl ExtendLocaleEmbed for api::User {
    fn extend_locale_embed(&self, locale: Locale, mut embed: EmbedBuilder) -> EmbedBuilder {
        embed = embed
            .author(EmbedAuthor {
                name: self.username.to_string(),
                url: Some(format!("https://scratch.mit.edu/users/{}", &self.username)),
                icon_url: Some(self.profile.images.n50x50.to_string()),
                proxy_icon_url: None,
            })
            .field(EmbedFieldBuilder::new(
                locale.user_history_joined(),
                format!("<t:{}:R>", self.history.joined.unix_timestamp()),
            ));

        if let Some(country) = &self.profile.country {
            embed = embed.field(EmbedFieldBuilder::new(
                locale.user_country(),
                country.to_string(),
            ))
        }

        if !self.profile.bio.is_empty() {
            embed = embed.field(EmbedFieldBuilder::new(
                locale.user_profile_bio(),
                self.profile.bio.to_string(),
            ));
        }

        if !self.profile.status.is_empty() {
            embed = embed.field(EmbedFieldBuilder::new(
                locale.user_profile_work(),
                self.profile.status.to_string(),
            ));
        }

        embed
    }
}

impl ExtendLocaleEmbed for db::User {
    fn extend_locale_embed(&self, locale: Locale, mut embed: EmbedBuilder) -> EmbedBuilder {
        if let Some(status) = &self.status {
            let status = status.to_localized(locale);
            let description = if let Some(school) = self.school {
                let school = format!("[{school}](https://scratch.mit.edu/classes/{school}/)");
                locale.user_status_student(&school, &status)
            } else {
                status
            };
            embed = embed.description(description);
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
            Self::Scratcher => locale.user_status_scratcher(),
            Self::NewScratcher => locale.user_status_new_scratcher(),
            Self::TeacherAccount => locale.user_status_teacher(),
            Self::ScratchTeam => locale.user_status_scratch_team(),
        }
    }
}

impl ToLocalized<EmbedFieldBuilder> for db::user::Statistics {
    fn to_localized(&self, locale: Locale) -> EmbedFieldBuilder {
        let mut vec = Vec::with_capacity(6);

        if let Some(loves) = self.loves {
            vec.push(locale.user_stats_loves(&loves.to_string()));
        }
        if let Some(favorites) = self.favorites {
            vec.push(locale.user_stats_favorites(&favorites.to_string()));
        }
        if let Some(comments) = self.comments {
            vec.push(locale.user_stats_comments(&comments.to_string()));
        }
        if let Some(views) = self.views {
            vec.push(locale.user_stats_views(&views.to_string()));
        }

        vec.push(locale.user_stats_followers(&self.followers.to_string()));
        vec.push(locale.user_stats_following(&self.following.to_string()));

        let value = vec.join("\n");

        EmbedFieldBuilder::new(locale.user_stats(), value)
    }
}

impl ToLocalized<EmbedFieldBuilder> for db::user::Ranks {
    fn to_localized(&self, locale: Locale) -> EmbedFieldBuilder {
        let value = vec![
            locale.user_stats_loves(&self.loves.to_string()),
            locale.user_stats_favorites(&self.favorites.to_string()),
            locale.user_stats_comments(&self.comments.to_string()),
            locale.user_stats_views(&self.views.to_string()),
            locale.user_stats_followers(&self.followers.to_string()),
            locale.user_stats_following(&self.following.to_string()),
        ]
        .join("\n");

        EmbedFieldBuilder::new(locale.user_stats_ranks(), value)
    }
}

impl ToLocalized<EmbedFieldBuilder> for db::user::Country {
    fn to_localized(&self, locale: Locale) -> EmbedFieldBuilder {
        let value = vec![
            locale.user_stats_loves(&self.loves.to_string()),
            locale.user_stats_favorites(&self.favorites.to_string()),
            locale.user_stats_comments(&self.comments.to_string()),
            locale.user_stats_views(&self.views.to_string()),
            locale.user_stats_followers(&self.followers.to_string()),
            locale.user_stats_following(&self.following.to_string()),
        ]
        .join("\n");

        EmbedFieldBuilder::new(locale.user_stats_ranks_country(), value)
    }
}
