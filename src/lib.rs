/*
 * Copyright (c) 2019 Lucas Vasilakopoulos
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![deny(warnings)]
#![warn(clippy::all)]

//! This crate provides the capability to load desired environment variables
//! into a map, merging with any optional defaults specified by the input map.

use failure::Fail;
use std::collections::HashMap;
use std::env;
use std::env::VarError;

type EnvVars<'a, T> = &'a HashMap<&'a str, Option<&'a str>, T>;
type EnvMap = HashMap<String, String>;
type EnvErrors = HashMap<String, VarError>;

#[derive(Debug, Fail)]
#[fail(display = "error(s) occurred loading environment variables")]
pub struct LoadError {
    env_errors: EnvErrors,
}

/// Loads variables from the environment, falling back and setting defaults if
/// they are provided. It returns an error for any missing required variables.
///
/// The returned result is a map of environment variables. If a required variable
/// is missing or a unicode error is encountered, the returned error contains a
/// map of variables to the corresponding error.
///
/// Examples:
///
/// ```
/// use env_map::load;
/// use std::collections::HashMap;
///
/// let env_vars: HashMap<&str, Option<&str>> = [("REQUIRED", None), ("OPTIONAL", Some("default"))]
///     .iter()
///     .cloned()
///     .collect();
///
/// match load(&env_vars) {
///     Ok(env) => println!("{:?}", env),
///     Err(err) => println!("{:?}", err),
/// }
/// ```
pub fn load<T>(env_vars: EnvVars<T>) -> Result<EnvMap, LoadError> {
    let mut env_map: EnvMap = HashMap::new();
    let mut env_errors: EnvErrors = HashMap::new();

    for (&name, &option) in env_vars {
        let key = name.to_string();

        match env::var(&key) {
            Ok(value) => {
                env_map.insert(key, value);
            }
            Err(err) => match err {
                VarError::NotPresent => match option {
                    Some(value) => {
                        env::set_var(&key, value);
                        env_map.insert(key, value.to_string());
                    }
                    None => {
                        env_errors.insert(key, err);
                    }
                },
                _ => {
                    env_errors.insert(key, err);
                }
            },
        }
    }

    if env_errors.is_empty() {
        Ok(env_map)
    } else {
        Err(LoadError { env_errors })
    }
}
