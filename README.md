# Weaver Example

## Description

This project acts as an example to showcase the features of weaver - the OpenTelemetry project that provides tooling for semantic conventions.

### Items

- Semantic convention model - an example of a model that inherits from the Otel semantic conventions.

  - Check the model for errors

    `weaver registry check -r model`

  - Get a resolved schema

    `weaver registry resolve -r model`

  - Generate documentation for the model

    `weaver registry generate -r model --templates "https://github.com/open-telemetry/semantic-conventions/archive/refs/tags/v1.34.0.zip[templates]" markdown docs`

  - Generate code for the model

    `weaver registry generate -r model --templates templates rust src`

  - Use the model to emit example telemetry

    `otel-desktop-viewer --browser 8001`

    `weaver registry emit -r model`

- Example application - an application which gets CPU information and sends appropriate spans and metrics

  - Use live-check to see if the program's telemetry is compliant with the model

    `weaver registry live-check -r model --inactivity-timeout 20`

- CI - a github workflow to build and test the application
  - A live-check test which runs the application in a test mode and will pass/fail based on live-check exit code

## Installation

```bash

```

## Usage

```bash

```
