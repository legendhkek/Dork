use rusqlite::{Connection, Result as SqlResult};
use anyhow::Result;
use chrono::Utc;

pub struct Database {
    conn: Connection,
}

#[derive(Debug)]
pub struct Statistics {
    pub total_scans: usize,
    pub vulnerabilities: usize,
    pub dorks_checked: usize,
    pub subdomains: usize,
    pub db_size: String,
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS dork_results (
                id INTEGER PRIMARY KEY,
                target TEXT NOT NULL,
                dork TEXT NOT NULL,
                result TEXT,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sql_scans (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL,
                method TEXT NOT NULL,
                payload TEXT,
                vulnerable INTEGER NOT NULL,
                response TEXT,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS subdomains (
                id INTEGER PRIMARY KEY,
                domain TEXT NOT NULL,
                subdomain TEXT NOT NULL,
                ip_address TEXT,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS port_scans (
                id INTEGER PRIMARY KEY,
                target TEXT NOT NULL,
                port INTEGER NOT NULL,
                status TEXT NOT NULL,
                service TEXT,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS hash_results (
                id INTEGER PRIMARY KEY,
                hash TEXT NOT NULL,
                plain TEXT,
                algorithm TEXT,
                method TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Database { conn })
    }
    
    pub fn save_dork_result(&self, target: &str, dork: &str, result: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO dork_results (target, dork, result, timestamp) VALUES (?1, ?2, ?3, ?4)",
            [target, dork, result, &Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }
    
    pub fn save_sql_scan(&self, url: &str, method: &str, payload: &str, vulnerable: bool, response: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO sql_scans (url, method, payload, vulnerable, response, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![url, method, payload, vulnerable as i32, response, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }
    
    pub fn save_subdomain(&self, domain: &str, subdomain: &str, ip: Option<&str>) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO subdomains (domain, subdomain, ip_address, timestamp) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![domain, subdomain, ip, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }
    
    pub fn save_port_scan(&self, target: &str, port: u16, status: &str, service: Option<&str>) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO port_scans (target, port, status, service, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![target, port as i32, status, service, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }
    
    pub fn save_hash_result(&self, hash: &str, plain: Option<&str>, algo: &str, method: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO hash_results (hash, plain, algorithm, method, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![hash, plain, algo, method, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }
    
    pub async fn get_statistics(&self) -> Result<Statistics> {
        let total_scans: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM sql_scans",
            [],
            |row| row.get(0),
        )?;
        
        let vulnerabilities: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM sql_scans WHERE vulnerable = 1",
            [],
            |row| row.get(0),
        )?;
        
        let dorks_checked: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM dork_results",
            [],
            |row| row.get(0),
        )?;
        
        let subdomains: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM subdomains",
            [],
            |row| row.get(0),
        )?;
        
        let db_size = std::fs::metadata("swiss_army.db")
            .map(|m| format!("{:.2} MB", m.len() as f64 / 1024.0 / 1024.0))
            .unwrap_or_else(|_| "Unknown".to_string());
        
        Ok(Statistics {
            total_scans,
            vulnerabilities,
            dorks_checked,
            subdomains,
            db_size,
        })
    }
}
