# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.2] - 2020-05-08
### Added
- #26 command line parameter `--rotctld-interval` and config option
  `rotctld_interval` to control the polling interval of the rotator position
- #26 increase default `rotctld_interval` from 1 to 5 seconds
- #20 command line parameter `--job-update-interval` and config option
  `job_update_interval` to control how often we ask the network for new jobs

## [0.4.1] - 2020-04-29
### Fixed
- Update `satnogs-network-client` to work with new station api

## [0.4.0] - 2020-04-13
### Added
- #18 config options `rotator_warn` and `rotator_error` to override the warning
  and error level for the rotator position display
  
### Changed
- #22 switch to new waterfall file format (SatNOGS Client >= 1.1)
- default rotator warning level is now 5.0 deg
- default rotator error level is now  15.0 deg

### Fixed
- Support rotor performing a reset where no position is returned
- Fix #15 dB -> color mapping for waterfall plot
- Fix #21 Fixes panic when waterfall data but no waterfall frequency information
  is available

## [0.3.1] - 2019-12-06
### Added
- command line parameter `--db-min` and `--db-max` to set the dB range of the
  spectrum and waterfall plot
- ui config options `db_min` and `db_max` to set the dB range of the spectrum
  and waterfall plot
- display current azimuth and elevation of the current observations vessel
- display rotator position if rotctld address is set
- command line parameter `--rotctld-address` to enable rotator monitoring
- config option `rotctld_address` to enable rotator monitoring

## [0.3.0] - 2019-12-02
### Added
- spectrum plot `--spectrum`
- waterfall widget `--waterfall` (requires a true color terminal emulator)
- Command line parameter `--data-path` to specify the satnogs-client data path

## [0.2.0] - 2019-10-07
### Added
- Command line parameter `-a|--api` to specify the SatNOGS network api endpoint.
- Polar plot
- Show satellite footprint on map.
- toggle satellite footprint with `f` key
- config option `sat_footprint` to enable/disable satellite footprint on start

### Changed
- Display as many future jobs as possible.
- Compiles on stable Rust.

### Fixed
- Typos
- Defaults for non essential config file entries.
- Background of the log window is cleared on display.

## [0.1.1] - 2018-11-18
### Added
- Command line parameter `-o|--orbit` which specifies how many orbits of the
  current satellite are plotted on the map.
 
### Changed
- Reduced SatNOGS API calls.
- Reduced CPU load and update satellite ground tracks only when orbit number
  has changed.
- Reduced used colors to base16 until themes have landed (to support hopefully
  all terminal emulators).

### Fixed
- Fix parse error when the station hasn't been seen by the network.
