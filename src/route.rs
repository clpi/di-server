use std::{
    error::Error,
    fmt::{self, Formatter}, path,
    convert::{
        From, TryFrom,
    },
};

#[derive(Debug, Clone, Default)]
pub struct Router {

}

#[derive(Debug, Clone, Default)]
pub struct Routes {
    routes: Vec<Route>,
}

#[derive(Debug, Clone)]
pub enum RouteError {
    NotFound(String),
    InvalidCharacter,
    NotAuthorized,
    InternalError,
    Timeout,
}

impl Routes {

    pub fn new(mut routes: Vec<String>)
        -> Result<Self, RouteError> {
        let mut r_out: Vec<Route> = Vec::new();
        for route in routes.iter_mut() {
            match Route::try_from(route.clone()) {
                Ok(route) => r_out.push(route),
                Err(e) => return Err(e),
            };
        }
        Ok(Self { routes: r_out })
    }

    pub fn is_valid(&self, route: String) -> Result<Route, RouteError> {
        Ok(Route::try_from(route).unwrap())
    }
}

type RouteFragment = String;

#[derive(Debug, Clone)]
pub struct Route {
    path: Vec<RouteFragment>,
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for p in self.path.clone() {
            out.push('/');
            out.extend(p.chars());
        }
        f.write_str(out.as_str())
    }
}

impl Default for Route {
    fn default() -> Self {
        Self { path: vec![] }
    }
}

impl TryFrom<String> for Route {
    type Error = RouteError;
    fn try_from(input: String) -> Result<Route, Self::Error> {
        let mut path: Vec<String> = Vec::new();
        for element in input.split("/") {
            if !element.chars().all(|c| c.is_alphabetic()) {
                return Err(RouteError::InvalidCharacter);
            }
            path.push(element.to_string());
        }
        Ok(Self { path })
    }
}

// impl TryFrom<Vec<&'static str>> for Routes {}
