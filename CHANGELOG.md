# Version 0.5.2

* Fix canonical directory locations for Windows

# Version 0.5.1

* Allow ctrl-c while using `smaug run`
* Save the `smaug run` pid to a file to allow external processes to quit them
# Version 0.5.0

* Add support for DragonRuby Indie

# Version 0.4.0

* Add support for DragonRuby versions > 3
* Display identifier in `smaug dragonruby list` for easier uninstall

# Version 0.3.2

* Fix setting devid when calling `publish`

# Version 0.3.1

* Add `config` command to show your current Smaug configuration
* Respect `.smaugignore` file when building projects or installing packages
* Add `docs` command to open the configured DragonRuby version's docs in your web browser.

# Version 0.3.0

* Add `--json` argument to all commands
* Fix an issue with `smaug add` and Windows line endings

# Version 0.2.3

* Fix dragonruby install from itch.io package
* Fix issue with add with empty dependencies in config
* Fix `smaug.rb` generation on Windows
* Add `compile_ruby` flag to enable Bytecode compilation
* Automatically run `smaug install` on `smaug add`
