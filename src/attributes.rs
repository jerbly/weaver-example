// DO NOT EDIT, this is an auto-generated file

/// A simple message.
///
/// ## Notes
///
/// This attribute is used to demonstrate a simple string attribute.
///
/// # Examples
///
/// - `"Hello, World!"`
pub const EXAMPLE_MESSAGE: &str = "example.message";

/// The CPU architecture the host system is running on
pub const HOST_ARCH: &str = "host.arch";

/// Name of the host. On Unix systems, it may contain what the hostname command returns, or the fully qualified hostname, or another name specified by the user.
///
/// # Examples
///
/// - `"opentelemetry-test"`
pub const HOST_NAME: &str = "host.name";

/// Logical name of the service.
///
/// ## Notes
///
/// MUST be the same for all instances of horizontally scaled services. If the value was not specified, SDKs MUST fallback to `unknown_service:` concatenated with [`process.executable.name`](process.md), e.g. `unknown_service:bash`. If `process.executable.name` is not available, the value MUST be set to `unknown_service`.
///
/// # Examples
///
/// - `"shoppingcart"`
pub const SERVICE_NAME: &str = "service.name";

/// The language of the telemetry SDK
pub const TELEMETRY_SDK_LANGUAGE: &str = "telemetry.sdk.language";

/// The name of the telemetry SDK as defined above.
///
/// ## Notes
///
/// The OpenTelemetry SDK MUST set the `telemetry.sdk.name` attribute to `opentelemetry`.
/// If another SDK, like a fork or a vendor-provided implementation, is used, this SDK MUST set the
/// `telemetry.sdk.name` attribute to the fully-qualified class or module name of this SDK's main entry point
/// or another suitable identifier depending on the language.
/// The identifier `opentelemetry` is reserved and MUST NOT be used in this case.
/// All custom identifiers SHOULD be stable across different versions of an implementation.
///
/// # Examples
///
/// - `"opentelemetry"`
pub const TELEMETRY_SDK_NAME: &str = "telemetry.sdk.name";

/// The version string of the telemetry SDK.
///
/// ## Notes
///
/// The OpenTelemetry SDK MUST set the `telemetry.sdk.name` attribute to `opentelemetry`.
/// If another SDK, like a fork or a vendor-provided implementation, is used, this SDK MUST set the
/// `telemetry.sdk.name` attribute to the fully-qualified class or module name of this SDK's main entry point
/// or another suitable identifier depending on the language.
/// The identifier `opentelemetry` is reserved and MUST NOT be used in this case.
/// All custom identifiers SHOULD be stable across different versions of an implementation.
///
/// # Examples
///
/// - `"1.2.3"`
pub const TELEMETRY_SDK_VERSION: &str = "telemetry.sdk.version";

