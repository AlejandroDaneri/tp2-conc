//! Este es el modulo principal para la busqueda de sinonimos mediante el uso de herramientas de concurrencia
pub mod balancer;
pub mod finder_executor;
pub mod merriamwebster;
pub mod searcher;
pub mod thesaurus;
pub mod yourdictionary;

use crate::{logger, requester::Requester};

#[derive(Debug)]
pub struct QueryResponse {
    pub word: String,
    pub synonyms: Vec<String>,
}

/// Error que ocurre durante la ejecucion de la busqueda
#[derive(Debug)]
pub struct FinderError;

impl From<reqwest::Error> for FinderError {
    fn from(_error: reqwest::Error) -> Self {
        FinderError
    }
}

impl std::fmt::Display for FinderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error en el buscador de sinónimos")
    }
}

impl std::error::Error for FinderError {}
/// Trait a implementar en todas las paginas en las cual se quiere hacer una busqueda

pub trait Finder {
    /// Genera la nueva busqueda
    fn new_query(word: &str) -> Self
    where
        Self: Sized;

    fn get_id() -> String;

    /// Arma la url a utilizar
    fn url(&self) -> String;

    /// Hace el parseo del contenido de la pagina
    fn parse_body(&self, body: &str) -> QueryResponse;

    /// Encuentra los sinonimos en esta pagina
    fn find_synonyms(&self) -> Result<QueryResponse, FinderError> {
        let log = logger::Logger::new(logger::Level::Debug);

        let url = self.url();

        log.info(format!("Making request to {:?}", url));

        let body = match Requester::make_request(url) {
            Ok(request) => request,
            Err(_err) => unreachable!(), //TODO:
        };

        log.info(format!("Finish request to {:?}", self.url()));
        Ok(self.parse_body(body.as_str()))
    }
}
