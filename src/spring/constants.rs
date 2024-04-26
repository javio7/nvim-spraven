use super::errors::SpringtimeError;

// Spring Curl
pub const SPRING_URL: &str = "https://start.spring.io";

// Dependencies
pub const DEPENDENCIES: &str = "dependencies";

// Java version
pub const JAVA_VERSION: &str = "javaVersion";
pub const JAVA_VERSION_LUAFILE: &str = "java_version.lua";

// Spring Boot version
pub const SPRING_BOOT_VERSION: &str = "bootVersion";
pub const SPRING_BOOT_VERSION_LUAFILE: &str = "spring_boot.lua";

// Spring Boot libraries
pub const LIBRARIES_LUAFILE: &str = "libraries.lua";

// Error wrapper
pub type SpringtimeResult<T=()> = Result<T, SpringtimeError>;
