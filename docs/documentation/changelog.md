version 0.0.0

Date: Fri Mar 10 19:17:57 2023 →0300

- Added main rust files
- Added main rust workflow
- Added common modules
- common.rs- common function for all application
- extras.rs- function for handle help texts
- Removed security module
- Fix operational system call functions
- Added error handling for system calls

version 0.0.69

Date: Wed May 10 04:24:54 2023 →0300

- Update rust dependencies
- Update main rust workflow
- Fix unformatted manual and help texts
- Added new manual samples
- Added new modules
- secman.rs- help text and manuals
- automation- sample automation function
- Update development branches

Version 0.0.98

Date: Wed Jun 14 04:21:15 PM →03 2023

- Removed unformatted manual and help texts
- Removed split files
- Removed non→standard manual pages
  (they will be reintroduced as proper man pages)
- Updated unit tests
- Updated installer for dnf package manager
- Updated Rust dependencies

Version 0.2.20

Date: Чт 29 июн 2023 20:58:11 MSK

- Added function to read shell parameters
- Implemented automation's base
- Added automation's for basic enumeration
- Added automation's for basic exploitation
- Added manual page for NetMaid
- Added new test suit
- Added new test script
- Remove unused path file (maidlists file path/list)
- Removed non→standard manual pages
- Updated unit tests
- Updated installer for dnf package manager
- Updated Rust dependencies

Version 0.2.60

Date: Пт 30 июн 2023 12:45:53 MSK

- Move all functions from automatons.rs to automation's
  module and create one file for each pentest step
- Added automation's module:
  enumeration.rs - scans, ping, dns lookup
  exploitation.rs - social, exploits, wireless
  fakedump.rs - create a fake database
  footprint.rs - not implemented yet, osint
  google/brave dorks
  harvester.rs - not implemented yet, web crawler
  keepkit.rs - not implemented yet, rootkit
    - Update code base
    - Update main version
    - Update CHANGELOG
    - Remove unused temp files
    - Remove unused imports
    - Apply cargo recommended improvements
    - Correcting misspelled words and improving grammar

Version 0.2.62

Date: Tue Jul 25 02:39:05 PM →03 2023

- Update the README.
- Fix the panic when the number of arguments is less than 2.
- Remove unused variables from the enumeration module.

Version 0.5.0 Rebuild for #maid_runner_toolkit

Date: Tue, Aug 8, 2023, 06:14:42 PM →03

- Old Code Base Revamped: The old code base has been dropped
  due to extensive experimentation and issues arising from an
  overly complex shell, which made adding new features challenging.
    - Removed outdated documentation: Discarded old documents and
      readme files.
    - Trimmed redundancies: Eliminated obsolete modules and files.
    - Preserved Files: Retained fuzzy files.
    - Kept User Assistance: Help pages are maintained, excluding
      command manuals.
- Fresh Code Base Introduced: A new code base has been added from
  scratch, addressing the limitations of the previous implementation.
- Enhanced Return Types: Custom return types have been incorporated
  to refine data output accuracy.
- Optimized Minimal Shell: A customized minimal shell has been
  developed to enhance efficiency and user→friendliness.
- Integration of hacklib: The 'hacklib' interface library has been
  included, enriching system capabilities and interactions.

Version 0.6.1

Date: Qua 23 ago 2023 16:24:22 -03

- Merge new shell commands
- Merge new commond execute methods
- Merge new features
- New shell- String handle- Scanners- Lookups- Files- Fix smal bugs at web_scanner

Version 0.6.4

Date: ter 29 ago 2023 11:12:17 -03

- Added report creation and export as JSON.
- Added lookup exif metadata
- Allow execution of external commands
- Fix bugs and crash them when shell commands are executed.
- Fix the system_time function result; change sdt::time to
  chronos.
- Remove unused imports.
- Fix conflicts.
- Fix dependencies on the git workflow
- Fix duplicate files.
- Finish the current base
- Report JSON with known bugs.

Version 0.7.0

Date: sáb 09 set 2023 09:48:03 -03

- Added curl binds (all http verbs)
- Added curl authentication (all methods)
- Added curl content type bindings (all kinds)
- Added a curl help page to the manual and readme.
- Added curl test (not all implemented yet)
- Fixed github workflow dependencies

Version 0.7.1

Date: qua 13 set 2023 17:07:56 -03

- Updated main README.md
- Updated general/readme.md
- Rename files folder to maid_lists
- Updated version
- Fix config file paths
- Merged maid_list project to MaidRunner
- Added maid_lists xss-payload-list.txt- Added maid_lists user_names.ascii- Added maid_lists maid_list_universal.ascii- Added maid_lists files_extensions.ascii- Added maid_lists malware_hash.ascii- Added maid_lists repo_keyword_scan.ascii- Added maid_lists dice_eff_words.ascii

Version 0.8.0

Date: dom 01 out 2023 16:46:59 -03

- Refactored code base with Rustfmt and LLama2
- Refactored imports for better organization and readability
- Added Maid antivirus support
- Added CSV support
- Added support for error codes
- Added support MAC lookup default file path
- Added new pattern matching backend
- Documentation updated with sample source code included
- Manual pages updated
- Unit tests updated
- Docs and images moved to sub folders for easier navigation
- Removed unused debugs
- Removed warnings about unused variables and imports
- Documentation split into separate files inside docs
- Fixed unit tests hard coded file path

Version 0.9.0

Date: seg 09 out 2023 21:18:50 -03

- Added maid_av Active antivirus scanner
- added a pattern-based AV scanner
- Added hash-based AV scanner
- Added ASCII-based entropy algorithm
- Added messages module
- Debug messages have been modified to utilize standard_messages
  rather than prints.
- Added an HTML folder inside maid_lists
- Fixed bugs while generating reports. json
- Fixed a considerable number of small bugs
- Refactored code base with rustfmt
- Refactored almost all debug
- Removed maid_reports API prototype
- Updated gitignore content

Version: 0.9.7

Date: qua 18 out 2023 05:33:38 -03

- Added codium code-workspace
- Added codium code-workspace
- Added new attack sub modules
- Added new documentation
- Added session description
- Removed old documentations and useless files
- Updated post_attack to rootkit
- Improve web scanner unit tests
- Improve report fields, add debug and session description
  and and
  change command prefix to source
- Build and install inside /bin
- Fixed empty message them using core --web_downloader
- Fixed report.jsonl bug at session description
- Fixed bug that caused panic if fewer than 3 arguments
  are given.
- Fixed sub domains and dir scanner output file path

Version 0.12.0

Date: sáb 21 out 2023 01:09:01 -03

- Transition to MONOREPO repository architecture
- Added maid_build a custom build system for MaidRunner project
- Added maid_ui single page application template for maid_visual
- Added maid_visual desktop UI with TAURI and maid_ui
- Added CODE_OF_CONDUCT.md with FreeBSD CoC
- Added SECURITY.md with security policy from aircrack.ng
- Improved logging system, reports are written in sqlite
- Removed report.json and report.html
- Refactored maid_reports to maid_api, now it's an web API only
- Refactored docs to witch_docs to keep file consistency on
  project root
- Refactored codebase, all edges were bean reviewed
- Refactored system_command_exec function to use an struct
  instead
  parameters. ProcessInit instead of parameters
- Removed write_report are replaced by logger function who
  uses Logger
  struct instead parameters and write ProcessResult inside
  maid_lists/report/archive.db
- Fixed MEOW file parser that caused panic if an variable
  have and
  string with spaces

Version 0.12.1

Date: ter 24 out 2023 02:26:27 -03

- Added maid_visual logger visualization for all categories
- Added development build script inside maid_build/local
- Added maid_api github workflow file
- Fixed logs visualization issues.
- Fixed database error on maid_visual.
- Fixed minor visual bug on maid_ui.
- Implemented features branches.
- Implemented sequential version system.
- Updated ZeroVer version system maximum number to 0.99.99

Version 0.12.2

Date: ter 20 nov 2023 12:00:00 -03

- Fixed small bugs inside core module.
- Fixed missing documentation.

Version 0.13.0

Date: seg 27 nov 2023 17:08:28 -03

- Optimized overhead and removed memory leaks.
- Utilized rustfmt for code formatting.
- Added the ISO Norms module to enhance cybersecurity practices.
- Added a CybersecurityFramework builder for increased flexibility.
- Added fully implemented the firewall module.
- Added the ability to back up and restore rule sets.
- Added a new test sample.
- Updated code style using rustfmt.
- Updated the sample template module.
- Updated gsv function to 'take_system_args' for readability.
- Updated the manual page.
- Updated outgoing port list.
- Updated the static database URL to use the URL inside the
  config file.
- Fixed clone() statements for further memory optimization.
- Fixed logger result return for improved reliability.
- Removed old cybersecurity policy.
- Removed old documentation.

Version 0.13.9

Date: sex 22 dez 2023 20:14:37 -03

- Re-implemented Debug Flag Feature
- Fix malware hash extension
- Remove unnecessary verbosity from mac lookup
- Fix scanner_automap invalid ports and delays.
- Fixed small bugs in the debugger:|, Fixed manual page.
- Update warning section
- Partial update
- Update firewall unit tests.

Version 0.14.0:

Date: sex 29 dez 2023 20:51:45 -03

- Implemented city_geo_location function
- Fixed file names from conf to config
- Added private modules configs
- Corrected country_code file name
- Added base IP and city geo location functions
- Added private modules path
- Added hosts database from StevenBlack/hosts
- Moved manual to the readme file
- Removed manpage
- Updated manual page
- Added OSINT domains hosts names (dns) names update script
- Opened link with Firefox
- Added base OSINT module and OpenStreetMap link generator
- Hardcoded debug option to force take_system_args return an
  empty string instead of an error message

Version: 0.15.0 Rebranding witchcraft

Date: sáb 06 jan 2024 13:32:33 -03

- Re-brand all projects to witchcraft.
- Add a new proper media kit inside witch_docs/media_kit.
- Change the default prefix from maid*\* to witch*\*.
- Implements all modules, even if they are empty.
- Implements private modules using a private module name
- Partially implements the blood_moon backend module inside the attack module.
- Change the data installation folder to /var/witchcraft/spellbook.
- Replace maid_visual using witch_oracle with next. js
- Replaced core.manual.rs with a manual module; the new module has its own section.
- Removed maid_api, maid_builder, and maid_workflows
- Removed manual section from README.md; all manuals are generated by helping it function itself.

Version 0.16.0 Backend rebuild from scretch

- All witchcraft backend has been rewritten from scratch.
- All old config files, documentation, and media have been dropped.
- All sample files have been dropped.
- Hosts files have been unified.
- The virus hashes database now only contains hashes with no additional info.
- All old modules have been removed, and AV features are unavailable.
- Backend now follows a meta-string based CLI architecture.

Version 0.17.1

Date: Sun Aug 11 04:23:53 PM -03 2024

- Alive2 update - back witchcraft development
- Add netrunner wiki and manual
- Change dataset from Rust struct to db.json
- Update functions documentation, run, and code cleanup
- Add DOS long passwords bind
- Remove check_install, remove setup bind, add setup with install in db.json
- Add regex and colored output
- Add db.json setup script
- Update magic documentation
- Move fancy image in chafa format to constants
- Add magic arguments list
- improve witch_fmt and magic docs

Version 0.17.3

Date: Tue Aug 13 05:13:55 PM -03 2024

- Add binds inside db.json
- Refatored shell flawless entry point
- Fixed DOS functions
- Fixed lint warnings

Version 0.17.10

Date: Tue Aug 20 08:30:19 PM -03 2024

- Rollback signature based AV with a new backend
- Add directory_lookup has AV backend
- Small overall fixes
- Updated wiki
- Updated malware list

Version 0.17.65

Date: Fri Aug 30 07:12:51 PM -03 2024

- Rebuild all backends
- Split binds into domains, like: osint, social
- Add per module api
- Add Closure type for api returns
- Move core out of modules
- Removed shell module
- Add process::exit to the main function

Version 0.17.72

Date: Mon Sep 9 11:42:09 AM -03 2024

- Removed general/wordlists
- Added defaults-credentials wordlist
- Added ladybug (compact) wordlist
- Added moth (large) wordlist
- Updated malware db
- Updated snapcraft
- Updated witchcraft version

Version 0.18.0

Date: Mon Sep 16 11:35:28 AM -03 2024

- Added Ip lookup for ANS, IP Geo-location and proxy
- Added local lookup database
- Removed wiki (wiki will be inside cosmic-zip.github.io only)

Version 0.18.2

Date: Sat Oct 5 10:37:26 AM -03 2024

- Added bind fhide based on steghide for steganography
- Update malware database
- Update build.sh
- Update showcase

Version 0.18.5

Date: Wed Oct 9 12:37:18 PM -03 2024

- Added eviltwin http server
- Added eviltwin local webpages clone
- Update dataset.json

Version: 0.19.0

Date: Fri Oct 25 02:22:50 PM -03 2024

- Added social media osint research using headless_chrome
- Added version command
- Added cinsscore bad ip address list
- Added partial web osint tool
- Added catfish phishing modules
- Added profile optimizations
- Added read log to json function
- Added log system for raw_exec
- Added json logger for external callers
- Added witchrc for readrc witchrc files
- Added default evilpages if none is given
- Updated snap build
- Updated build script
- Updated all eprintln with raise,
- Updated readme
- Fixed get_os_env returning a slash for no reason
- Fixed missing osint search option not in dataset
- Fixed panic if .witchrc or log file is missing
- Fixed snap build
- Fixed magic docs
- Removed hardcoded path in favor of WITCH_SPELLS_ROOT_DIR

Version: 0.20.0

Date: Sat Oct 26 07:09:02 PM -03 2024

- Launched LTS release 0.20.0

Revision 0.20.20

Date: Sun Nov 3 21:56:30 -03 2024

- Refactored OSINT meta search
- Added OSINT database in json format and removed rust vector database
- Added project webpage
- Small fixes for github CI/CD, Snapcraft, Dockerfile and readme

Revision 0.20.21

Date: Wed Nov 6 10:38:08 AM -03 2024

- Update revision to 0.20.21
- Fixed unnecessary error message when a command returns an error code with no stderr
- fixed DOS attack incomplete url
- Fixed chromium deps
- Fixed renamed incorrectly named function from dos.simple to dos.spam
- Updated documentation

Revision: 0.20.25

Date: Thu Nov 7 18:06:53 -03 2024

- Added chkrootkit binding
- Remove unused files
- Move archive/spellbook to spellbook/archive
- Update version
- Removed unused dependencies

Revision 0.20.56

Date: Thu Nov 7 18:06:53 -03 2024

- Added build-dist.sh to build a binary a compiled version of witchcraft
- Fixed github CI
- Fixed missing file errors
- Fixed snap builds
- Fixed docker files
- Fixed build.sh

Version 0.33.666

Date: Mon Nov 18 14:30:19 2024 -0300

- Final codebase version
- Spliced spellbook in basic and full versions
- Created a new build and build-dist script

Revision 0.33.700

Date: Fri Nov 29 10:50:24 AM -03 2024

- Added spellbook installer
- Updated spellbook
- Updated version
- Updated readme/consts/snapfile

Version 0.34.10

Date: Mon Jan 13 16:28:26 2025 -0300

- Updated meta search
- Updated the spellbook

Version 0.34.30

Date: Thu Jan 16 02:19:43 PM -03 2025

- Added and fixed dataset/db.json sections, evilpages/default/index.html
- Added new help and manual page
- Fixed evil tcp server body output
- Fixed manual FLAGS SCL to SCLF
- Split manual and help pages in to commands
- Replaced sample evilpage for a new one

Version: 0.35.10

Date: Thu Jan 30 12:07:57 PM -03 2025

- Added witchcraft tldr
- Added spell checker and they config
- Update seth imports
- Update social import to seth
- Update snapcraft
- Small improvements
- Move social engineering related modules to seth module

Version: 0.36.10

Date: Mon Feb 17 10:35:09 AM -03 2025

- Added progress indicator
- Added new build-devel script
- Added logger function for rust functions, not only sub process
- Fixed get system environment paths
- Fixed panzer banner size
- Fixed merge conflicts with seth branch
- Removed all unused files in docs/
