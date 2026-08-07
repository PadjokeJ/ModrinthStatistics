# Modrinth Statistics

Get a detailed breakdown of licenses, modloaders, and minecraft versions uploaded to modrinth.

## Running the aggregator

First off, get yourself data from the modrinth api. **REMINDER: SCRAPING MODRINTH IS STRICTLY PROHIBITED, ONLY GATHER THE DATA YOU NEED.**

The rust version is structured specifically for this request: `https://api.modrinth.com/v2/search?query=&limit=100`

You need to put the data inside of a `results` folder at the root of the project

You can then execute the rust code with `cargo run [loader|version|license]` (provided you have a toolchain installed, see <https://rustup.rs>)

You should then find a file named `data.json` at the root of the project.

Per example, running `cargo run loader` will yield:
```
Fabric (56637)  :  ###########################################################################################
Forge (37070)   :  ###########################################################
Neoforge (29027):  ###############################################
Quilt (10603)   :  #################
```

Other interesting datasets can be found in theses files:
- [Popular mod Licenses](/licenses.md)
- [Popular mod versions](/versions.md)