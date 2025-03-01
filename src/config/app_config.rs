use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};
use std::env;
use crate::services::{OsrmConfig, RoutingConfig};

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Server configuration
    pub server: ServerConfig,
    
    /// OSRM configuration
    pub osrm: OsrmConfig,
    
    /// Routing configuration
    pub routing: RoutingConfig,
}

/// Server configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,
    
    /// Port to listen on
    pub port: u16,
    
    /// Number of worker threads
    pub workers: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: num_cpus::get(),
        }
    }
}

/// Internal configuration structure for deserialization
#[derive(Debug, Deserialize)]
struct ConfigFile {
    server: Option<ServerConfig>,
    osrm: Option<OsrmConfigFile>,
    routing: Option<RoutingConfigFile>,
}

#[derive(Debug, Deserialize)]
struct OsrmConfigFile {
    base_url: Option<String>,
    default_profile: Option<String>,
    timeout_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct RoutingConfigFile {
    default_max_time: Option<u32>,
    default_threads: Option<u8>,
}

impl AppConfig {
    /// Load configuration from file and environment variables
    pub fn load() -> Result<Self, ConfigError> {
        // Determine the run mode
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string());
        
        // Build configuration
        let builder = Config::builder()
            // Add configuration from files
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add configuration from environment variables
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .ignore_empty(true)
            );
            
        // Build and convert the config
        let config: ConfigFile = builder.build()?.try_deserialize()?;
        
        // Create server config
        let server = config.server.unwrap_or_default();
        
        // Create OSRM config
        let osrm_file = config.osrm.unwrap_or(OsrmConfigFile {
            base_url: None,
            default_profile: None,
            timeout_seconds: None,
        });
        
        let osrm = OsrmConfig {
            base_url: osrm_file.base_url.unwrap_or_else(|| "http://localhost:5000".to_string()),
            default_profile: osrm_file.default_profile.unwrap_or_else(|| "car".to_string()),
            timeout_seconds: osrm_file.timeout_seconds.unwrap_or(30),
        };
        
        // Create routing config
        let routing_file = config.routing.unwrap_or(RoutingConfigFile {
            default_max_time: None,
            default_threads: None,
        });
        
        let routing = RoutingConfig {
            osrm: osrm.clone(),
            default_max_time: routing_file.default_max_time.unwrap_or(30),
            default_threads: routing_file.default_threads.unwrap_or(4),
        };
        
        Ok(AppConfig {
            server,
            osrm,
            routing,
        })
    }
} 