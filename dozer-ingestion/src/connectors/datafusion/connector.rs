use dozer_types::ingestion_types::{LocalStorage, S3Storage};
use std::collections::HashMap;
use std::sync::Arc;

use crate::connectors::datafusion::schema_mapper::{Mapper, SchemaMapper};
use crate::connectors::datafusion::table_reader::{Reader, TableReader};
use crate::connectors::TableInfo;
use crate::errors::ConnectorError;
use crate::{connectors::Connector, errors, ingestion::Ingestor};
use dozer_types::parking_lot::RwLock;

pub struct DataFusionConnector<T: Clone> {
    pub id: u64,
    config: T,
    ingestor: Option<Arc<RwLock<Ingestor>>>,
    tables: Option<Vec<TableInfo>>,
}

impl<T: Clone> DataFusionConnector<T> {
    pub fn new(id: u64, config: T) -> Self {
        Self {
            id,
            config,
            ingestor: None,
            tables: None,
        }
    }
}

impl Connector for DataFusionConnector<S3Storage> {
    fn test_connection(&self) -> Result<(), ConnectorError> {
        todo!()
    }

    fn validate(&self, _tables: Option<Vec<TableInfo>>) -> Result<(), ConnectorError> {
        Ok(())
    }

    fn validate_schemas(
        &self,
        _tables: &[crate::connectors::TableInfo],
    ) -> Result<crate::connectors::ValidationResults, errors::ConnectorError> {
        Ok(HashMap::new())
    }

    fn get_schemas(
        &self,
        table_names: Option<Vec<TableInfo>>,
    ) -> Result<Vec<dozer_types::types::SchemaWithChangesType>, ConnectorError> {
        let tables = table_names
            .as_ref()
            .map_or_else(std::vec::Vec::new, |t| t.clone());
        let mapper = SchemaMapper::new(self.config.clone());
        mapper.get_schema(tables)
    }

    fn get_tables(&self) -> Result<Vec<crate::connectors::TableInfo>, errors::ConnectorError> {
        todo!()
    }

    fn initialize(
        &mut self,
        ingestor: Arc<RwLock<Ingestor>>,
        tables: Option<Vec<TableInfo>>,
    ) -> Result<(), ConnectorError> {
        self.ingestor = Some(ingestor);
        self.tables = tables;
        Ok(())
    }

    fn start(&self, _from_seq: Option<(u64, u64)>) -> Result<(), ConnectorError> {
        let tables = self
            .tables
            .as_ref()
            .map_or_else(std::vec::Vec::new, |t| t.clone());

        let reader = TableReader::new(self.config.clone());

        let ingestor = self
            .ingestor
            .as_ref()
            .map_or(Err(ConnectorError::InitializationError), Ok)?
            .clone();

        reader.read_tables(tables, ingestor)
    }

    fn stop(&self) {
        todo!()
    }
}

impl Connector for DataFusionConnector<LocalStorage> {
    fn test_connection(&self) -> Result<(), ConnectorError> {
        todo!()
    }

    fn validate(&self, _tables: Option<Vec<TableInfo>>) -> Result<(), ConnectorError> {
        Ok(())
    }

    fn validate_schemas(
        &self,
        _tables: &[crate::connectors::TableInfo],
    ) -> Result<crate::connectors::ValidationResults, errors::ConnectorError> {
        Ok(HashMap::new())
    }

    fn get_schemas(
        &self,
        table_names: Option<Vec<TableInfo>>,
    ) -> Result<Vec<dozer_types::types::SchemaWithChangesType>, ConnectorError> {
        let tables = table_names
            .as_ref()
            .map_or_else(std::vec::Vec::new, |t| t.clone());
        let mapper = SchemaMapper::new(self.config.clone());
        mapper.get_schema(tables)
    }

    fn get_tables(&self) -> Result<Vec<crate::connectors::TableInfo>, errors::ConnectorError> {
        todo!()
    }

    fn initialize(
        &mut self,
        ingestor: Arc<RwLock<Ingestor>>,
        tables: Option<Vec<TableInfo>>,
    ) -> Result<(), ConnectorError> {
        self.ingestor = Some(ingestor);
        self.tables = tables;
        Ok(())
    }

    fn start(&self, _from_seq: Option<(u64, u64)>) -> Result<(), ConnectorError> {
        let tables = self
            .tables
            .as_ref()
            .map_or_else(std::vec::Vec::new, |t| t.clone());

        let reader = TableReader::new(self.config.clone());

        let ingestor = self
            .ingestor
            .as_ref()
            .map_or(Err(ConnectorError::InitializationError), Ok)?
            .clone();

        reader.read_tables(tables, ingestor)
    }

    fn stop(&self) {
        todo!()
    }
}