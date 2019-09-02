use crate::{error::OptionsError, validation};

/// Options for serde support.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SerdeSupport {
    No,
    Yes,
    Mixed { serialize: bool, deserialize: bool },
}

impl SerdeSupport {
    pub(crate) fn should_derive_ser_de(self) -> Option<(bool, bool)> {
        match self {
            Self::No => None,
            Self::Yes => Some((true, true)),
            Self::Mixed {
                serialize,
                deserialize,
            } => {
                if !(serialize || deserialize) {
                    None
                } else {
                    Some((serialize, deserialize))
                }
            }
        }
    }
}

impl Default for SerdeSupport {
    fn default() -> Self {
        Self::No
    }
}

/// When to perform dynamic loading from the config file itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicLoading {
    /// Always load the config from file.
    Always,

    /// Load from file in debug mode, but use the statically-included const in
    /// release mode.
    DebugOnly,

    /// Never load dynamically. Always use the statically-included const.
    Never,
}

impl Default for DynamicLoading {
    fn default() -> Self {
        Self::DebugOnly
    }
}

/// Options for configuring the generation of a struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructOptions {
    /// The name of the resulting struct. Defaults to `"Config"`.
    pub struct_name: String,

    /// The name of the resulting const, if generated.
    /// Defaults to the value of `struct_name` in uppercase.
    pub const_name: Option<String>,

    /// Whether or not to generate a `const` instance of the struct.
    /// Defaults to `true`.
    pub generate_const: bool,

    /// A list of traits for the struct to derive. Defaults to `["Debug", "Clone",
    /// "Serialize", "Deserialize"]`
    ///
    /// **Note:** if you include `Serialize`/`Deserialize`, you must also include
    /// the `serde` and `serde_derive` crates in the crate that uses the generated
    /// config module.
    pub derived_traits: Vec<String>,

    /// Shorthand for generating the Serialize and Deserialize traits.
    /// Defaults to `No`.
    pub serde_support: SerdeSupport,

    /// The recommended way to derive Serialize and Deserialize is via the `serde`
    /// crate's `derive` feature: https://serde.rs/derive.html
    ///
    /// If you instead need to use the old method of including the `serde_derive`
    /// crate, set this flag to `true`.
    pub use_serde_derive_crate: bool,

    /// Whether or not to generate helper functions to load the struct at runtime.
    /// Defaults to `true`.
    ///
    /// **Note:** if you enable the generation of dynamic loading functions, you
    /// must also include the relevant deserialization crate in the crate that uses
    /// the generated config module.
    ///
    /// For example, if you generate a config based on "config.json", you must also
    /// include `extern crate serde_json` in your crate. Likewise with `toml`,
    /// `ron`, and `serde_yaml` for the other supported formats.
    pub generate_load_fns: bool,

    /// Whether the load functions, if generated, are dynamic, and when.
    /// Defaults to `DebugOnly`.
    pub dynamic_loading: DynamicLoading,

    /// Whether or not to create the parent directories of the output file, if
    /// they don't exist. Defaults to `true`.
    pub create_dirs: bool,

    /// Whether to check if the destination file would be changed before writing
    /// output. This is to avoid unnecessary writes from marking the destination
    /// file as changed (which could, for example, trigger a process which is
    /// watching for changes). This option only works with the `create_*` functions.
    /// Defaults to `true`.
    pub write_only_if_changed: bool,

    /// The type of floating point values in the config, where the format does not
    /// make it explicit.
    ///
    /// Defaults to `F64`.
    pub default_float_size: FloatSize,

    /// The type of integer values in the config, where the format does not
    /// make it explicit.
    ///
    /// Defaults to `I64`.
    pub default_int_size: IntSize,

    /// The maximum array size, over which array values in the config will be
    /// represented as slices instead. If set to `0`, slices will always be used.
    ///
    /// Defaults to `0`.
    pub max_array_size: usize,
}

/// Represents a floating-point type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSize {
    F32,
    F64,
}

/// Represents an integer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntSize {
    I8,
    I16,
    I32,
    I64,
    ISize,
}

impl StructOptions {
    pub fn validate(&self) -> Result<(), OptionsError> {
        if !validation::valid_identifier(&self.struct_name) {
            return Err(OptionsError::InvalidStructName(self.struct_name.clone()));
        }

        Ok(())
    }

    pub fn real_const_name(&self) -> String {
        self.const_name
            .clone()
            .unwrap_or_else(|| self.struct_name.to_uppercase())
    }
}

impl Default for StructOptions {
    /// ```rust
    /// use config_struct::*;
    ///
    /// let default_options = StructOptions {
    ///     struct_name: "Config".to_owned(),
    ///     const_name: None,
    ///     generate_const: true,
    ///     derived_traits: vec![
    ///         "Debug".to_owned(),
    ///         "Clone".to_owned(),
    ///     ],
    ///     serde_support: SerdeSupport::No,
    ///     use_serde_derive_crate: false,
    ///     generate_load_fns: true,
    ///     dynamic_loading: DynamicLoading::DebugOnly,
    ///     create_dirs: true,
    ///     write_only_if_changed: true,
    ///     default_float_size: FloatSize::F64,
    ///     default_int_size: IntSize::I64,
    ///     max_array_size: 0,
    /// };
    /// assert_eq!(default_options, StructOptions::default());
    /// ```
    fn default() -> Self {
        StructOptions {
            struct_name: "Config".to_owned(),
            const_name: None,
            generate_const: true,
            derived_traits: vec!["Debug".to_owned(), "Clone".to_owned()],
            serde_support: SerdeSupport::default(),
            use_serde_derive_crate: false,
            generate_load_fns: true,
            dynamic_loading: DynamicLoading::DebugOnly,
            create_dirs: true,
            write_only_if_changed: true,
            default_float_size: FloatSize::F64,
            default_int_size: IntSize::I64,
            max_array_size: 0,
        }
    }
}
