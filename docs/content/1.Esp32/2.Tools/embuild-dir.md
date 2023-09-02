# Embuild build Directory

  * https://github.com/esp-rs/embuild/issues/79
  * https://github.com/esp-rs/esp-idf-sys/blob/master/BUILD_OPTIONS.md#esp_idf_tools_install_dir-esp_idf_tools_install_dir

Typically when building under windows there can be issues with the created .embuild directory having paths that are too long.
One workaround is to specify an alternate path for the directory.

`.cargo/config.toml`
```
[env]
ESP_IDF_TOOLS_INSTALL_DIR = '''custom:C:\Temp\embuild'''
```
