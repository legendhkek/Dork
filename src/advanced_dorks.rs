// Advanced Google Dork Database
// Made by @LEGEND_BL

pub const ADMIN_PANEL_DORKS: &[&str] = &[
    "site:{} inurl:admin",
    "site:{} inurl:administrator",
    "site:{} inurl:admin/login",
    "site:{} inurl:admin/index",
    "site:{} inurl:admin.php",
    "site:{} inurl:admin_area",
    "site:{} inurl:panel",
    "site:{} inurl:controlpanel",
    "site:{} inurl:admincp",
    "site:{} inurl:cpanel",
    "site:{} inurl:adminpanel",
    "site:{} inurl:admin1",
    "site:{} inurl:admin2",
    "site:{} inurl:yonetim",
    "site:{} inurl:yonetici",
    "site:{} inurl:administracion",
    "site:{} inurl:moderator",
    "site:{} inurl:webadmin",
    "site:{} inurl:adminarea",
    "site:{} inurl:bb-admin",
];

pub const DATABASE_DORKS: &[&str] = &[
    "site:{} ext:sql",
    "site:{} ext:db",
    "site:{} ext:dbf",
    "site:{} ext:mdb",
    "site:{} filetype:sql",
    "site:{} filetype:sql intext:password",
    "site:{} filetype:sql intext:username",
    "site:{} ext:sql intext:\"INSERT INTO\"",
    "site:{} ext:sql intext:\"CREATE TABLE\"",
    "site:{} inurl:backup ext:sql",
    "site:{} inurl:database ext:sql",
    "site:{} inurl:dump ext:sql",
    "site:{} \"phpMyAdmin\" inurl:main.php",
    "site:{} \"phpMyAdmin SQL Dump\"",
    "site:{} \"MySQL dump\"",
];

pub const BACKUP_FILES_DORKS: &[&str] = &[
    "site:{} ext:bak",
    "site:{} ext:old",
    "site:{} ext:backup",
    "site:{} inurl:backup",
    "site:{} inurl:back",
    "site:{} filetype:bak",
    "site:{} filetype:sql",
    "site:{} filetype:zip",
    "site:{} filetype:tar.gz",
    "site:{} intext:\"backup\" ext:sql",
    "site:{} intext:\"backup\" ext:zip",
];

pub const CONFIG_FILES_DORKS: &[&str] = &[
    "site:{} ext:conf",
    "site:{} ext:config",
    "site:{} ext:cfg",
    "site:{} ext:ini",
    "site:{} filetype:conf",
    "site:{} inurl:config.php",
    "site:{} inurl:configuration.php",
    "site:{} inurl:settings.php",
    "site:{} inurl:config.inc.php",
    "site:{} ext:env",
    "site:{} intext:\"DB_PASSWORD\"",
    "site:{} intext:\"api_key\"",
];

pub const LOG_FILES_DORKS: &[&str] = &[
    "site:{} ext:log",
    "site:{} filetype:log",
    "site:{} inurl:log",
    "site:{} intext:\"error log\"",
    "site:{} inurl:error_log",
    "site:{} inurl:debug.log",
    "site:{} inurl:access.log",
    "site:{} inurl:error.log",
];

pub const DIRECTORY_LISTING_DORKS: &[&str] = &[
    "site:{} intitle:\"index of\"",
    "site:{} intitle:\"index of /admin\"",
    "site:{} intitle:\"index of /password\"",
    "site:{} intitle:\"index of /backup\"",
    "site:{} intitle:\"index of /mail\"",
    "site:{} intitle:\"index of /root\"",
    "site:{} \"Parent Directory\" \"Last Modified\"",
];

pub const LOGIN_PAGES_DORKS: &[&str] = &[
    "site:{} inurl:login",
    "site:{} inurl:signin",
    "site:{} inurl:auth",
    "site:{} inurl:authentication",
    "site:{} intitle:\"login\"",
    "site:{} intitle:\"sign in\"",
    "site:{} inurl:wp-login.php",
    "site:{} inurl:admin/login.php",
    "site:{} inurl:user/login",
];

pub const WORDPRESS_DORKS: &[&str] = &[
    "site:{} inurl:wp-content",
    "site:{} inurl:wp-admin",
    "site:{} inurl:wp-includes",
    "site:{} inurl:wp-login.php",
    "site:{} inurl:wp-config.php",
    "site:{} filetype:sql intext:wp_users",
    "site:{} intext:\"wp_users\" ext:sql",
];

pub const FILE_UPLOAD_DORKS: &[&str] = &[
    "site:{} inurl:upload",
    "site:{} inurl:uploader",
    "site:{} inurl:uploadfile",
    "site:{} inurl:file-upload",
    "site:{} inurl:upload.php",
];

pub const SQL_ERRORS_DORKS: &[&str] = &[
    "site:{} \"SQL syntax near\"",
    "site:{} \"mysql_fetch_array()\"",
    "site:{} \"You have an error in your SQL syntax\"",
    "site:{} \"Warning: mysql_\"",
    "site:{} \"ORA-00921: unexpected end of SQL\"",
    "site:{} \"Microsoft OLE DB Provider for SQL\"",
];

pub const SENSITIVE_INFO_DORKS: &[&str] = &[
    "site:{} filetype:xls password",
    "site:{} filetype:xlsx password",
    "site:{} filetype:doc password",
    "site:{} filetype:docx password",
    "site:{} filetype:pdf password",
    "site:{} filetype:txt password",
    "site:{} intext:\"confidential\"",
    "site:{} intext:\"restricted\"",
    "site:{} intext:\"not for public\"",
];

pub const GIT_DORKS: &[&str] = &[
    "site:{} inurl:.git",
    "site:{} intitle:\"index of\" .git",
    "site:{} inurl:\"/.git/config\"",
    "site:{} inurl:\".git/HEAD\"",
];

pub const API_DORKS: &[&str] = &[
    "site:{} inurl:api",
    "site:{} inurl:/api/v1",
    "site:{} inurl:/api/v2",
    "site:{} inurl:swagger",
    "site:{} inurl:graphql",
    "site:{} intext:\"api_key\"",
    "site:{} intext:\"apikey\"",
];

pub const SHELL_DORKS: &[&str] = &[
    "site:{} inurl:shell",
    "site:{} inurl:cmd",
    "site:{} inurl:webshell",
    "site:{} intext:\"c99shell\"",
    "site:{} intext:\"r57shell\"",
];

pub fn get_all_dorks() -> Vec<&'static str> {
    let mut all_dorks = Vec::new();
    all_dorks.extend_from_slice(ADMIN_PANEL_DORKS);
    all_dorks.extend_from_slice(DATABASE_DORKS);
    all_dorks.extend_from_slice(BACKUP_FILES_DORKS);
    all_dorks.extend_from_slice(CONFIG_FILES_DORKS);
    all_dorks.extend_from_slice(LOG_FILES_DORKS);
    all_dorks.extend_from_slice(DIRECTORY_LISTING_DORKS);
    all_dorks.extend_from_slice(LOGIN_PAGES_DORKS);
    all_dorks.extend_from_slice(WORDPRESS_DORKS);
    all_dorks.extend_from_slice(FILE_UPLOAD_DORKS);
    all_dorks.extend_from_slice(SQL_ERRORS_DORKS);
    all_dorks.extend_from_slice(SENSITIVE_INFO_DORKS);
    all_dorks.extend_from_slice(GIT_DORKS);
    all_dorks.extend_from_slice(API_DORKS);
    all_dorks.extend_from_slice(SHELL_DORKS);
    all_dorks
}

pub fn get_dorks_by_category(category: &str) -> Vec<&'static str> {
    match category {
        "admin" => ADMIN_PANEL_DORKS.to_vec(),
        "database" => DATABASE_DORKS.to_vec(),
        "backup" => BACKUP_FILES_DORKS.to_vec(),
        "config" => CONFIG_FILES_DORKS.to_vec(),
        "logs" => LOG_FILES_DORKS.to_vec(),
        "directory" => DIRECTORY_LISTING_DORKS.to_vec(),
        "login" => LOGIN_PAGES_DORKS.to_vec(),
        "wordpress" => WORDPRESS_DORKS.to_vec(),
        "upload" => FILE_UPLOAD_DORKS.to_vec(),
        "sql_errors" => SQL_ERRORS_DORKS.to_vec(),
        "sensitive" => SENSITIVE_INFO_DORKS.to_vec(),
        "git" => GIT_DORKS.to_vec(),
        "api" => API_DORKS.to_vec(),
        "shell" => SHELL_DORKS.to_vec(),
        _ => get_all_dorks(),
    }
}

pub fn get_categories() -> Vec<(&'static str, &'static str)> {
    vec![
        ("admin", "Admin Panel Discovery"),
        ("database", "Database Files"),
        ("backup", "Backup Files"),
        ("config", "Configuration Files"),
        ("logs", "Log Files"),
        ("directory", "Directory Listings"),
        ("login", "Login Pages"),
        ("wordpress", "WordPress Vulnerabilities"),
        ("upload", "File Upload Pages"),
        ("sql_errors", "SQL Error Messages"),
        ("sensitive", "Sensitive Information"),
        ("git", "Git Repositories"),
        ("api", "API Endpoints"),
        ("shell", "Web Shells"),
    ]
}
