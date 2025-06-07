# uptechstar-rs

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/uptechstar-rs.svg)](https://crates.io/crates/uptechstar-rs)
[![Build Status](https://github.com/Kazu-Kusa/uptechstar-rs/actions/workflows/publish.yml/badge.svg)](https://github.com/Kazu-Kusa/uptechstar-rs/actions)

---
A Rust library for interacting with hardware components on embedded systems, specifically designed for use with Uptech's
hardware.

## Features

This library provides access to various hardware components:

- **ADC-IO**: Provides functions for working with analog-to-digital converters (ADC) and digital input/output (IO)
  channels.
- **Display**: Provides functions for controlling an LCD display including drawing shapes, text, and managing LED
  colors.
- **MPU**: Provides functions for working with an MPU6500 6-axis motion sensor.

## Dependencies

The library requires `libuptech.so` to be present in the system. This library is loaded at runtime and provides the
low-level hardware interaction functionality.