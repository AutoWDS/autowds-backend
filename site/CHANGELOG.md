# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Data export to Google Sheet
- Data export to database

## [0.4.0] - 2026-04-25

### Added
- Add a Stop Collection button to the data collection task page.
- Add a toolbox with XPath selector and regex tools.

### Changed
- Improved user interface with modern design elements
- Optimize the issue where styles cannot be injected across multiple Shadow DOMs
- Optimize the single-element XPath generation algorithm
- Optimize the intelligent pagination recognition algorithm.

## [0.2.0] - 2024-12-01

### Added
- Brand new visual rule editor with drag-and-drop operations
- AI intelligent recognition to automatically identify page structures

### Changed
- Refactored core scraping engine, significantly improving stability and performance
- Updated user interface for a more intuitive user experience

### Removed
- Removed legacy compatibility mode, focusing on modern browser support

## [0.1.1] - 2024-11-15

### Fixed
- Fixed Chrome extension compatibility issues with the latest browser version
- Resolved bugs where scheduled tasks failed to execute properly in certain situations
- Fixed Chinese character encoding issues when exporting Excel files

### Changed
- Optimized memory usage, reducing memory consumption during long-running sessions

## [0.1.0] - 2024-10-30

### Added
- New scheduled task system for automated data collection
- Enhanced error reporting and debugging tools

### Changed
- Improved pagination detection algorithm
- Updated extension manifest to v3 for better security

### Fixed
- Fixed issues with AJAX content detection
- Resolved cookie handling problems on certain websites

## [0.0.2] - 2024-10-15

### Fixed
- Critical security fix for data export functionality
- Fixed memory leaks in long-running scraping sessions
- Resolved compatibility issues with React-based websites

### Changed
- Improved error messages for better user guidance
- Redesigned settings panel for better usability
- Improved element selector accuracy

## [0.0.1] - 2025-09-10

### Added
- Initial release of the browser extension
- Basic web scraping functionality
- Support for Chrome, Firefox, and Edge browsers
- CSV and JSON export formats
- Visual element selection tool

[Unreleased]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.0.2...v0.1.0
[0.0.2]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/octopus-crawler/octopus-crawler/releases/tag/v0.0.1