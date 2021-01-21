# Smaug

Smaug is a tool to manage your DragonRuby Game Toolkit projects.

# Usage

```
USAGE:
    smaug [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Silence all output
    -v, --verbose    Displays more information
    -V, --version    Prints version information

SUBCOMMANDS:
    build         Builds your DragonRuby project.
    dragonruby    Manages your local DragonRuby installation.
    help          Prints this message or the help of the given subcommand(s)
    init          Initializes an existing project as a Smaug project.
    install       Installs dependencies from Smaug.toml.
    new           Start a new DragonRuby project
    package       Manages your DragonRuby package.
    publish       Publish your DragonRuby project to Itch.io
    run           Runs your DragonRuby project.
```

# Starting a new DragonRuby project

1. Download a copy of the DragonRuby Game Toolkit from either [Itch.io](https://dragonruby.itch.io/dragonruby-gtk) (standard) or [the DragonRuby website](https://dragonruby.herokuapp.com/toolkit/game) (pro).
2. Install your downloaded copy of DragonRuby: `smaug install ~/Downloads/dragonruby-linux-amd64.zip`.
3. Create a new project: `smaug new my-game` then `cd my-game`.
4. Edit your project's configuration at `Smaug.toml`.
5. Run your game: `smaug run`.
6. Build your game: `smaug build`.
    * Builds will be stored in `my-game/builds`.
7. Publish your game: `smaug publish`.

# Migrate an existing DragonRuby project

The following instructions assume your project lives at `~/projects/dragonruby-linux-amd64/mygame`.

1. Move your game's directory outside of the DragonRuby directory: `mv ~/projects/dragonruby-linux-amd64/mygame ~/projects/mygame`.
2. Install your version of DragonRuby: `smaug dragonruby install ~/projects/dragonruby-linux-amd64`.
3. Add smaug to your project: `smaug init ~/projects/mygame`.
4. `cd ~/projects/mygame`
5. Edit your project's configuration at `Smaug.toml`.
6. Run your game: `smaug run`.
7. Build your game: `smaug build`.
    * Builds will be stored in `my-game/builds`.
8. Publish your game: `smaug publish`.

# Install a package

1. Edit `Smaug.toml`:
    ```
    [dependencies]
    draco = "https://github.com/guitsaru/draco.git"
    ```
2. Run `smaug install`
3. Add `require "app/smaug.rb"` to the top of your `main.rb`.

# Creating a package

1. Run `dragonruby package init` from your package's directory.
2. Edit your package's new `Smaug.toml` file to configure the package.
3. Add each of the files that are needed for the DragonRuby project.
    ```
    # package_file = project_file
    "lib/library.rb" = "app/lib/library.rb"

    # If you don't want to `require` the file in `smaug.rb`
    "lib/library.rb" = { path = "app/lib/library.rb", require = false }
    ```
4. Publish your changes.

