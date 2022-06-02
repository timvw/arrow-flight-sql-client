use arrow::datatypes::Schema;
use arrow::datatypes::SchemaRef;
use arrow::error::Result;
use arrow::error::ArrowError;
use arrow::ipc::MessageHeader;
use arrow_flight_sql_client::arrow_flight_protocol::*;
use arrow_flight_sql_client::arrow_flight_protocol::flight_service_client::FlightServiceClient;
use clap::{Args, Parser, Subcommand};
use std::cell::RefCell;
use tonic::transport::Channel;
use tonic::Streaming;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Execute(ExecuteArgs),
    ExecuteUpdate(ExecuteUpdateArgs),
    GetCatalogs(GetCatalogsArgs),
    GetTableTypes(GetTableTypesArgs),
    GetSchemas(GetSchemasArgs),
    GetTables(GetTablesArgs),
    GetExportedKeys(GetExportedKeysArgs),
    GetImportedKeys(GetImportedKeysArgs),
    GetPrimaryKeys(GetPrimaryKeysArgs),
}

#[derive(Args, Debug)]
struct Common {
    #[clap(long, default_value_t = String::from("localhost"))]
    hostname: String,
    #[clap(short, long, default_value_t = 52358, parse(try_from_str))]
    port: usize,
}

#[derive(Args, Debug)]
struct ExecuteArgs {
    #[clap(flatten)]
    common: Common,
    #[clap(short, long)]
    query: String,
}

#[derive(Args, Debug)]
struct ExecuteUpdateArgs {
    #[clap(flatten)]
    common: Common,
    #[clap(short, long)]
    query: String,
}

#[derive(Args, Debug)]
struct GetCatalogsArgs {
    #[clap(flatten)]
    common: Common,
}

#[derive(Args, Debug)]
struct GetTableTypesArgs {
    #[clap(flatten)]
    common: Common,
}

#[derive(Args, Debug)]
struct GetSchemasArgs {
    #[clap(flatten)]
    common: Common,
    #[clap(short, long)]
    catalog: Option<String>,
    #[clap(short, long)]
    db_schema_filter_pattern: Option<String>,
}

#[derive(Args, Debug)]
struct GetTablesArgs {
    #[clap(flatten)]
    common: Common,
    #[clap(short, long)]
    catalog: Option<String>,
    #[clap(short, long)]
    db_schema_filter_pattern: Option<String>,
    #[clap(short, long)]
    table_name_filter_pattern: Option<String>,
    #[clap(short, long)]
    include_schema: bool,
}

#[derive(Args, Debug)]
struct GetExportedKeysArgs {
    #[clap(flatten)]
    common: Common,
    #[clap(short, long)]
    catalog: Option<String>,
    #[clap(short, long)]
    db_schema: Option<String>,
    #[clap(short, long)]
    table: String,
}

#[derive(Args, Debug)]
struct GetImportedKeysArgs {
    #[clap(flatten)]
    common: Common,
    #[clap(short, long)]
    catalog: Option<String>,
    #[clap(short, long)]
    db_schema: Option<String>,
    #[clap(short, long)]
    table: String,
}

#[derive(Args, Debug)]
struct GetPrimaryKeysArgs {
    #[clap(flatten)]
    common: Common,
    #[clap(short, long)]
    catalog: Option<String>,
    #[clap(short, long)]
    db_schema: Option<String>,
    #[clap(short, long)]
    table: String,
}

async fn new_client(
    hostname: &String,
    port: &usize,
) -> Result<FlightServiceClient<Channel>> {
    let client_address = format!("http://{}:{}", hostname, port);
    FlightServiceClient::connect(client_address)
        .await
        .map_err(transport_error_to_arrow_erorr)
}

async fn get_and_print(
    mut client: FlightServiceClient<Channel>,
    fi: FlightInfo,
) -> Result<()> {
    let first_endpoint = fi.endpoint.first().ok_or(ArrowError::ComputeError(
        "Failed to get first endpoint".to_string(),
    ))?;

    let first_ticket = first_endpoint
        .ticket
        .clone()
        .ok_or(ArrowError::ComputeError(
            "Failed to get first ticket".to_string(),
        ))?;

    /*
    let mut flight_data_stream = client.do_get(first_ticket).await?;

    let arrow_schema = arrow_schema_from_flight_info(&fi)?;
    let arrow_schema_ref = SchemaRef::new(arrow_schema);

    print_flight_data_stream(arrow_schema_ref, &mut flight_data_stream).await*/
    unimplemented!()
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Execute(ExecuteArgs {
                              common: Common { hostname, port },
                              query,
                          }) => {
            let mut client = new_client(hostname, port).await?;
            /*
            let fi = client.execute(query.to_string()).await?;
            get_and_print(client, fi).await*/
            unimplemented!()
        }
        Commands::ExecuteUpdate(ExecuteUpdateArgs {
                                    common: Common { hostname, port },
                                    query,
                                }) => {
            let mut client = new_client(hostname, port).await?;
            /*let record_count = client.execute_update(query.to_string()).await?;
            println!("Updated {} records.", record_count);
            Ok(())*/
            unimplemented!()
        }
        Commands::GetCatalogs(GetCatalogsArgs {
                                  common: Common { hostname, port },
                              }) => {
            let mut client = new_client(hostname, port).await?;
            /*
            let fi = client.get_catalogs().await?;
            get_and_print(client, fi).await*/
            unimplemented!()
        }
        Commands::GetTableTypes(GetTableTypesArgs {
                                    common: Common { hostname, port },
                                }) => {
            let mut client = new_client(hostname, port).await?;
            /*
            let fi = client.get_table_types().await?;
            get_and_print(client, fi).await*/
            unimplemented!()
        }
        Commands::GetSchemas(GetSchemasArgs {
                                 common: Common { hostname, port },
                                 catalog,
                                 db_schema_filter_pattern: schema,
                             }) => {
            let mut client = new_client(hostname, port).await?;
            /*
            let fi = client
                .get_db_schemas(CommandGetDbSchemas {
                    catalog: catalog.as_deref().map(|x| x.to_string()),
                    db_schema_filter_pattern: schema.as_deref().map(|x| x.to_string()),
                })
                .await?;
            get_and_print(client, fi).await*/
            unimplemented!()
        }
        Commands::GetTables(GetTablesArgs {
                                common: Common { hostname, port },
                                catalog,
                                db_schema_filter_pattern,
                                table_name_filter_pattern,
                                include_schema,
                            }) => {
            let mut client = new_client(hostname, port).await?;
            /*
            let fi = client
                .get_tables(CommandGetTables {
                    catalog: catalog.as_deref().map(|x| x.to_string()),
                    db_schema_filter_pattern: db_schema_filter_pattern
                        .as_deref()
                        .map(|x| x.to_string()),
                    table_name_filter_pattern: table_name_filter_pattern
                        .as_deref()
                        .map(|x| x.to_string()),
                    table_types: vec![],
                    include_schema: *include_schema,
                })
                .await?;
            get_and_print(client, fi).await*/
            unimplemented!()
        }
        Commands::GetExportedKeys(GetExportedKeysArgs {
                                      common: Common { hostname, port },
                                      catalog,
                                      db_schema,
                                      table,
                                  }) => {
            let mut client = new_client(hostname, port).await?;
            /*
            let fi = client
                .get_exported_keys(CommandGetExportedKeys {
                    catalog: catalog.as_deref().map(|x| x.to_string()),
                    db_schema: db_schema.as_deref().map(|x| x.to_string()),
                    table: table.to_string(),
                })
                .await?;
            get_and_print(client, fi).await*/
            unimplemented!()
        }
        Commands::GetImportedKeys(GetImportedKeysArgs {
                                      common: Common { hostname, port },
                                      catalog,
                                      db_schema,
                                      table,
                                  }) => {
            let mut client = new_client(hostname, port).await?;
            /*
            let fi = client
                .get_imported_keys(CommandGetImportedKeys {
                    catalog: catalog.as_deref().map(|x| x.to_string()),
                    db_schema: db_schema.as_deref().map(|x| x.to_string()),
                    table: table.to_string(),
                })
                .await?;
            get_and_print(client, fi).await

             */
            unimplemented!()
        }
        Commands::GetPrimaryKeys(GetPrimaryKeysArgs {
                                     common: Common { hostname, port },
                                     catalog,
                                     db_schema,
                                     table,
                                 }) => {
            let mut client = new_client(hostname, port).await?;
            /*
            let fi = client
                .get_primary_keys(CommandGetPrimaryKeys {
                    catalog: catalog.as_deref().map(|x| x.to_string()),
                    db_schema: db_schema.as_deref().map(|x| x.to_string()),
                    table: table.to_string(),
                })
                .await?;
            get_and_print(client, fi).await*/
            unimplemented!()
        }
    }
}

async fn print_flight_data_stream(
    arrow_schema_ref: SchemaRef,
    flight_data_stream: &mut Streaming<FlightData>,
) -> Result<()> {
    while let Some(flight_data) = flight_data_stream
        .message()
        .await
        .map_err(status_to_arrow_error)?
    {
        let arrow_data = arrow_data_from_flight_data(flight_data, &arrow_schema_ref)?;
        match arrow_data {
            ArrowFlightData::RecordBatch(record_batch) => {
                arrow::util::pretty::print_batches(&[record_batch])?;
            }
            _ => {} // no data to print..
        }
    }

    Ok(())
}

pub fn transport_error_to_arrow_erorr(error: tonic::transport::Error) -> ArrowError {
    //ArrowError::TonicRequestError(format!("{}", error))
    unimplemented!()
}

pub fn status_to_arrow_error(status: tonic::Status) -> ArrowError {
    //ArrowError::TonicRequestError(format!("{:?}", status))
    unimplemented!()
}

pub enum ArrowFlightData {
    RecordBatch(arrow::record_batch::RecordBatch),
    Schema(arrow::datatypes::Schema),
}

pub fn arrow_data_from_flight_data(
    flight_data: FlightData,
    arrow_schema_ref: &SchemaRef,
) -> Result<ArrowFlightData> {
    let ipc_message =
        arrow::ipc::root_as_message(&flight_data.data_header[..]).map_err(|err| {
            ArrowError::ParseError(format!("Unable to get root as message: {:?}", err))
        })?;

    match ipc_message.header_type() {
        MessageHeader::RecordBatch => {
            let ipc_record_batch =
                ipc_message
                    .header_as_record_batch()
                    .ok_or(ArrowError::ComputeError(
                        "Unable to convert flight data header to a record batch"
                            .to_string(),
                    ))?;

            /*
            let dictionaries_by_field = &[];
            let record_batch = arrow::ipc::reader::read_record_batch(
                &flight_data.data_body,
                ipc_record_batch,
                arrow_schema_ref.clone(),
                dictionaries_by_field,
                None,
            )?;
            Ok(ArrowFlightData::RecordBatch(record_batch))*/
            unimplemented!()
        }
        MessageHeader::Schema => {
            let ipc_schema =
                ipc_message
                    .header_as_schema()
                    .ok_or(ArrowError::ComputeError(
                        "Unable to convert flight data header to a schema".to_string(),
                    ))?;

            let arrow_schema = arrow::ipc::convert::fb_to_schema(ipc_schema);
            Ok(ArrowFlightData::Schema(arrow_schema))
        }
        MessageHeader::DictionaryBatch => {
            let _ = ipc_message.header_as_dictionary_batch().ok_or(
                ArrowError::ComputeError(
                    "Unable to convert flight data header to a dictionary batch"
                        .to_string(),
                ),
            )?;
            Err(ArrowError::NotYetImplemented(
                "no idea on how to convert an ipc dictionary batch to an arrow type"
                    .to_string(),
            ))
        }
        MessageHeader::Tensor => {
            let _ = ipc_message
                .header_as_tensor()
                .ok_or(ArrowError::ComputeError(
                    "Unable to convert flight data header to a tensor".to_string(),
                ))?;
            Err(ArrowError::NotYetImplemented(
                "no idea on how to convert an ipc tensor to an arrow type".to_string(),
            ))
        }
        MessageHeader::SparseTensor => {
            let _ =
                ipc_message
                    .header_as_sparse_tensor()
                    .ok_or(ArrowError::ComputeError(
                        "Unable to convert flight data header to a sparse tensor"
                            .to_string(),
                    ))?;
            Err(ArrowError::NotYetImplemented(
                "no idea on how to convert an ipc sparse tensor to an arrow type"
                    .to_string(),
            ))
        }
        _ => Err(ArrowError::ComputeError(format!(
            "Unable to convert message with header_type: '{:?}' to arrow data",
            ipc_message.header_type()
        ))),
    }
}

pub fn arrow_schema_from_flight_info(fi: &FlightInfo) -> Result<Schema> {
    let ipc_message = arrow::ipc::size_prefixed_root_as_message(&fi.schema[4..])
        .map_err(|e| ArrowError::ComputeError(format!("{:?}", e)))?;

    let ipc_schema = ipc_message
        .header_as_schema()
        .ok_or(ArrowError::ComputeError(
            "failed to get schema...".to_string(),
        ))?;

    let arrow_schema = arrow::ipc::convert::fb_to_schema(ipc_schema);

    Ok(arrow_schema)
}
