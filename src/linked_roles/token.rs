use oauth2::{
    basic::BasicTokenType, AccessToken, EmptyExtraTokenFields, RefreshToken, StandardTokenResponse,
    TokenResponse,
};
use time::OffsetDateTime;

pub type OAuthToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: OffsetDateTime,
}

impl Into<OAuthToken> for Token {
    fn into(self) -> OAuthToken {
        let access_token = AccessToken::new(self.access_token);
        let refresh_token = RefreshToken::new(self.refresh_token);
        let token_type = BasicTokenType::Bearer;
        let extra_fields = EmptyExtraTokenFields {};

        let mut token = OAuthToken::new(access_token, token_type, extra_fields);

        token.set_refresh_token(Some(refresh_token));

        let expires_in = (self.expires_at - OffsetDateTime::now_utc())
            .whole_seconds()
            .try_into()
            .unwrap_or(0);
        token.set_expires_in(Some(&std::time::Duration::from_secs(expires_in)));

        token
    }
}

impl TryFrom<OAuthToken> for Token {
    type Error = ();

    fn try_from(value: OAuthToken) -> Result<Self, Self::Error> {
        let expires_at = OffsetDateTime::now_utc() + value.expires_in().ok_or(())?;

        Ok(Self {
            access_token: value.access_token().secret().to_string(),
            refresh_token: value.refresh_token().ok_or(())?.secret().to_string(),
            expires_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use time::Duration;

    use super::*;

    #[test]
    fn into_from() {
        let original = Token {
            access_token: "access_token".into(),
            refresh_token: "refresh_token".into(),
            expires_at: OffsetDateTime::now_utc() + Duration::seconds(10),
        };

        let expected = original.clone();

        let intermediate: OAuthToken = original.into();

        let actual: Token = intermediate.try_into().unwrap();

        assert_eq!(actual.access_token, expected.access_token);
        assert_eq!(actual.refresh_token, expected.refresh_token);
        assert!(actual.expires_at.unix_timestamp() - expected.expires_at.unix_timestamp() <= 1);
    }
}
