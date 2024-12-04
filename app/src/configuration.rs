use secrecy::{ExposeSecret as _, SecretBox};

/// An enum that enumerates possible runtime environments for our application.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    /// Converts the 'Environment' value to a str.
    ///
    /// # Arguments
    ///
    /// * `self` - The self reference.
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        match text.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
                Use either `local` or `production`.",
                other
            )),
        }
    }
}

/// A struct that represents our settings.
#[derive(serde::Deserialize)]
pub struct Settings {
    /// Database settings.
    pub database: DatabaseSettings,
    /// Application settings.
    pub application: ApplicationSettings,
}

/// A struct that represents our database's settings.
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    /// A username.
    pub username: String,
    /// A password.
    pub password: SecretBox<String>,
    /// A port.
    pub port: u16,
    /// A host.
    pub host: String,
    /// A database name.
    pub database_name: String,
}

impl DatabaseSettings {
    /// Gets a connection string.
    ///
    /// # Arguments
    ///
    /// * `self` - The self reference.
    pub fn connection_string(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )))
    }

    /// Gets a connection string without the database name part.
    ///
    /// # Arguments
    ///
    /// * `self` - The self reference.
    pub fn connection_string_without_db(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )))
    }
}

/// A struct that represents our application's settings.
#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    /// A port.
    pub port: u16,
    /// A host.
    pub host: String,
}

/// Gets configuration.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}
