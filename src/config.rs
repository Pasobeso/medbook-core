use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub frontend: Frontend,
    pub database: Database,
    pub message_queue: MessageQueue,
    pub swagger: Swagger,
}

#[derive(Debug, Clone)]
pub struct Swagger {
    pub swagger_path: String,
    pub swagger_json_path: String,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: usize,
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct Frontend {
    pub development_url: String,
    pub production_url: String,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct MessageQueue {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct PatientsSecret {
    pub secret: String,
    pub refresh_secret: String,
}

#[derive(Debug, Clone)]
pub struct DoctorsSecret {
    pub secret: String,
    pub refresh_secret: String,
}

use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self {
            Stage::Local => "Local",
            Stage::Development => "Development",
            Stage::Production => "Production",
        };

        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn try_from(stage: &str) -> Result<Self> {
        match stage {
            "Local" => Ok(Stage::Local),
            "Development" => Ok(Stage::Development),
            "Production" => Ok(Stage::Production),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let frontend = Frontend {
        production_url: std::env::var("PRODUCTION_FRONTEND_URL")
            .expect("PRODUCTION_FRONTEND_URL is invalid"),
        development_url: std::env::var("DEVELOPMENT_FRONTEND_URL")
            .expect("DEVELOPMENT_FRONTEND_URL is invalid"),
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    let message_queue = MessageQueue {
        url: std::env::var("RMQ_URL").expect("RMQ_URL is invalid"),
    };

    let swagger = Swagger {
        swagger_path: std::env::var("SWAGGER_PATH").expect("SWAGGER_PATH is invalid"),
        swagger_json_path: std::env::var("SWAGGER_JSON_PATH")
            .expect("SWAGGER_JSON_PATH is invalid"),
    };

    Ok(DotEnvyConfig {
        server,
        frontend,
        database,
        message_queue,
        swagger,
    })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_patients_secret_env() -> Result<PatientsSecret> {
    dotenvy::dotenv().ok();

    Ok(PatientsSecret {
        secret: std::env::var("JWT_PATIENT_SECRET").expect("JWT_PATIENT_SECRET is invalid"),
        refresh_secret: std::env::var("JWT_PATIENT_REFRESH_SECRET")
            .expect("JWT_PATIENT_REFRESH_SECRET is invalid"),
    })
}

pub fn get_doctors_secret_env() -> Result<DoctorsSecret> {
    dotenvy::dotenv().ok();

    Ok(DoctorsSecret {
        secret: std::env::var("JWT_DOCTOR_SECRET").expect("JWT_DOCTOR_SECRET is invalid"),
        refresh_secret: std::env::var("JWT_DOCTOR_REFRESH_SECRET")
            .expect("JWT_DOCTOR_REFRESH_SECRET is invalid"),
    })
}
