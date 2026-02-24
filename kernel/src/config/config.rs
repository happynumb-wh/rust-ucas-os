

pub mod config {
    //! Platform configuration module.
    //!
    //! If the `UC_CONFIG_PATH` environment variable is set, it will load the configuration from the specified path.
    //! Otherwise, it will fall back to the `ucconfig.toml` file in the current directory and generate the default configuration.
    //!
    //! If the `PACKAGE` field in the configuration does not match the package name, it will panic with an error message.
    axconfig_macros::include_configs!(path_env = "UCCONFIG_PATH", fallback = "ucconfig.toml");
}