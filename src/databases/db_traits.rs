use tokio_postgres;
use async_trait::async_trait;

pub struct SqlQuery {
    pub db_type: String,
}

pub trait Query {
    fn get_tables_query(&self) -> &str;
    fn get_columns_query(&self) -> &str;
    fn get_count_query(&self,  schema: &String, table: &String) -> String;
}

pub struct Postgres {
    pub name: String,
    pub client: tokio_postgres::Client,
}

impl Postgres {
    fn get_client(&self) -> &tokio_postgres::Client {
        &self.client
    }
}

pub struct Column {
    pub catalog: Option<String>,
    pub schema: Option<String>,
    pub table_name: Option<String>,
    pub column_name: Option<String>,
    pub ordinal: Option<i32>,
    pub default: Option<String>,
    pub is_nullable: Option<String>,
    pub data_type: Option<String>,
    pub max_length: Option<i32>
}

#[async_trait]
pub trait Catalog {
    async fn query_tables(&self, schema: &String) -> Result<Vec<String>, tokio_postgres::Error>;
    async fn query_columns(&self, schema: &String, table: &String) -> Result<Vec<Column>, tokio_postgres::Error>;
    async fn query_row_count(&self, schema: &String, table: &String) -> Result<i64, tokio_postgres::Error>;
}

pub trait Database<T> {
    fn get_provider(&self) -> String;
    fn get_name(&self) -> String;
    fn get_connection(&self) -> T;
    fn get_queries(&self) -> Box<dyn Query>;
}

impl Query for Postgres {
    fn get_tables_query(&self) -> &str {
        "SELECT table_name FROM information_schema.tables where table_schema=$1"
    }
    fn get_columns_query(&self) -> &str {
        "SELECT table_catalog, table_schema, table_name, column_name, ordinal_position, column_default, is_nullable, data_type, character_maximum_length FROM information_schema.columns where table_schema =$1 and table_name=$2"
    }
    fn get_count_query(&self,  schema: &String, table: &String) -> String {
        format!("SELECT COUNT(*) from {0}.{1}", schema, table)
    }
}

impl Database<tokio_postgres::Connection<tokio_postgres::Socket, tokio_postgres::tls::NoTlsStream>>
    for Postgres
{
    fn get_provider(&self) -> std::string::String {
        todo!()
    }
    fn get_name(&self) -> std::string::String {
        todo!()
    }
    fn get_connection(
        &self,
    ) -> tokio_postgres::Connection<tokio_postgres::Socket, tokio_postgres::tls::NoTlsStream> {
        todo!()
    }
    fn get_queries(&self) -> Box<dyn Query> {
        todo!()
    }
}

#[async_trait]
impl Catalog for Postgres {
    async fn query_tables(&self, schema: &String) -> Result<Vec<String>, tokio_postgres::Error> {
        let tables_query = self.get_tables_query();

        let rows: Vec<tokio_postgres::Row> = self.get_client().query(tables_query, &[schema]).await?;
        
        let mut tables: Vec<String> = Vec::new();

        for row in rows.iter() {
            let value: String = row.get(0);
            tables.push(value);
        }

        Ok(tables)
    }
    async fn query_columns(&self, schema: &String, table: &String) -> Result<Vec<Column>, tokio_postgres::Error> {
        let column_query = self.get_columns_query();
        
        let rows: Vec<tokio_postgres::Row> = self.get_client().query(column_query, &[schema, table]).await?;
        
        let mut columns: Vec<Column> = Vec::new();

        for row in rows.iter() {
            
            let column = Column{
                catalog: row.get(0),
                schema: row.get(1),
                table_name: row.get(2),
                column_name: row.get(3),
                ordinal: row.get(4),
                default: row.get(5),
                is_nullable: row.get(6),
                data_type: row.get(7),
                max_length: row.get(8)
            };
            columns.push(column);
        }

        Ok(columns)
    }
    async fn query_row_count(&self, schema: &String, table: &String) -> Result<i64, tokio_postgres::Error> {
        let count_query = self.get_count_query(schema, table);
        let rows: Vec<tokio_postgres::Row> = self.get_client().query(count_query.as_str(), &[]).await?;

        let count: i64 = rows[0].get(0);
        Ok(count)
    }
}
