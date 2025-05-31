# Weaver Example

## Description

This project demonstrate practical usage of weaver - the OpenTelemetry project that provides tooling for semantic conventions.

## Prerequisites

Install weaver by following the instructions from the [release](https://github.com/open-telemetry/weaver/releases) that matches your OS. Be sure to have weaver on your path for the commands below to work. Also, an internal test requires weaver on the path if you run that.

## Semantic convention model

In the `model` directory is a model that defines attributes, spans and metrics. The OpenTelemetry Semantic Conventions project already defines a large set of attributes and signals which you can reference by declaring a dependency in `registry_manifest.yaml`.

To assist with authoring these yaml files, most modern IDEs support json schemas that provide inline feedback, hints and completion. This project contains an example for vscode. Weaver will also report any schema errors when you run any of its `registry` commands.

### Check the model for errors

The schema can only express fundamental structural requirements for the model. `check` digs deeper to ensure the model is valid. (Note that `check` is run implicitly for all `registry` commands too).

`weaver registry check -r model`

### Get a resolved schema

It's often useful to see how you model has been resolved to a registry with this command. This is useful when you're using references, more so if those references are to a dependency.

`weaver registry resolve -r model`

### Generate documentation

Weaver can pass the resolved schema to jinja templates to render documentation. You can write your own, or reference templates defined elsewhere. This example uses the templates defined in the OpenTelemetry Semantic Conventions project to generate the docs directory and content.

`weaver registry generate -r model --templates "https://github.com/open-telemetry/semantic-conventions/archive/refs/tags/v1.34.0.zip[templates]" markdown docs`

### Generate code

Like the documentation, Weaver can use jinja templates to create code. In this case, custom templates are defined in this project. The command below will create the `attributes.rs` and `metrics.rs` files in the `src` directory right alongside the application code.

`weaver registry generate -r model --templates templates rust src`

### Emit example telemetry

It's useful to see what your telemetry, as defined in your model, will look like in observability tools. `emit` generates OTLP with example data for each signal defined in your model. (Only spans are supported by weaver 0.15.2 - the latest as of 31-May-2025)

`otel-desktop-viewer --browser 8001`

In another shell:

`weaver registry emit -r model`

## Example application

The rust application sends spans and metrics to demonstrate the live-check feature of weaver. `live-check` acts like an OpenTelemetry Collector - it receives telemetry in OTLP and analyzes its compatibility with a model. We can use this in tests locally and in CI to regression test our application's instrumentation.

The application has been contrived so you can provoke a violation in `live-check`. The `example.message` attribute is defined with type `string` but you can produce an `int` or `float` by providing these in the command line parameters: `cargo run -- 42` will send an `int`. `live-check` will return a violation for this type mismatch and exit with code 1.

The `live-check.rs` unit test checks for pass and fail by providing strings and ints as above.

### Use live-check on the command line

`weaver registry live-check -r model --inactivity-timeout 20`

In another shell:

`cargo run -- foo 42`

### CI

A github workflow to build and test the application is included. The application is live-checked in the workflow.

see [release-and-test.yml](https://github.com/jerbly/weaver-example/blob/main/.github/workflows/release-and-test.yml)
