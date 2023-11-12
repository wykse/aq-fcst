use clap::Parser;
use polars::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about="Get NOAA's Air Quality Forecast Guidance as a CSV", long_about = None)]
pub struct Args {
    /// Path to CSV containing input points; header must have following field names: point_id, lat, long
    pub input: String,

    /// Path to output CSV
    pub output: String,

    /// Url for ArcGIS image service identify endpoint
    #[arg(short, long, default_value_t = String::from("https://mapservices.weather.noaa.gov/raster/rest/services/air_quality/ndgd_apm25_hr01_bc/ImageServer/identify"))]
    pub url: String,

    /// Output an additional CSV that is processed, e.g., your_output_processed.csv. File is filtered for latest issuance and sorted ascending
    #[arg(short, long)]
    pub process: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Point {
    pub point_id: String,
    pub lat: f64,
    pub long: f64,
}

impl Point {
    pub fn to_json_geometry(&self) -> Result<String, Box<dyn Error>> {
        // Serialize long and lat into a json formatted string for ArcGIS REST API.
        // Reference: https://mapservices.weather.noaa.gov/raster/sdk/rest/02ss/02ss0000008m000000.htm#POINT

        let geo_string = format!(
            "{{'x': {}, 'y': {}, 'spatialReference': {{'wkid': 4326}}}}",
            self.long, self.lat
        );

        Ok(geo_string)
    }
}

// Data about the identify response, includes the point requested,
// when it was requested, request url, and response body
#[derive(Debug, Serialize)]
pub struct IdentifyResponse {
    pub point: Point,
    pub requested_on: String,
    pub url: String,
    pub body: ResponseBody,
}

impl IdentifyResponse {
    /// Processes the response body and outputs a CSV with a few additional columns.
    /// TODO: Separate the processing and CSV writing into separate methods.
    pub fn process_body(&self, output_file: &str) -> Result<(), Box<dyn Error>> {
        // Pixel values for the point
        let values = &self.body.properties.values;

        // Source of the pixel values
        let rasters = &self.body.catalog_items.features;

        // Open a file for writing
        let file = File::create(output_file)?;

        let mut wtr = csv::Writer::from_writer(file);

        // Struct for the other fields to include in the record
        #[derive(Debug, Serialize)]
        struct OtherFields<'a> {
            idp_issueddate_iso: Option<String>,
            idp_validtime_iso: Option<String>,
            value: &'a String,
            requested_on: &'a String,
            url: &'a String,
        }

        // Timezone for local time
        let los_angeles_tz = chrono_tz::America::Los_Angeles;

        // Hold each processed record
        let mut records: Vec<(&Point, OtherFields, &Attributes)> = Vec::new();

        for i in 0..values.len() {
            let idp_validtime_iso: Option<String> =
                if let Some(validtime) = &rasters[i].attributes.idp_validtime {
                    let datetime =
                        chrono::DateTime::from_timestamp((validtime / 1000).try_into().unwrap(), 0)
                            .unwrap()
                            .with_timezone(&los_angeles_tz);
                    Some(datetime.to_rfc3339())
                } else {
                    None
                };

            let idp_issueddate_iso: Option<String> = if let Some(issueddate) =
                &rasters[i].attributes.idp_issueddate
            {
                let datetime =
                    chrono::DateTime::from_timestamp((issueddate / 1000).try_into().unwrap(), 0)
                        .unwrap()
                        .with_timezone(&los_angeles_tz);
                Some(datetime.to_rfc3339())
            } else {
                None
            };

            let other_fields = OtherFields {
                idp_issueddate_iso,
                idp_validtime_iso,
                value: &values[i],
                requested_on: &self.requested_on,
                url: &self.url,
            };
            let record = (&self.point, other_fields, &rasters[i].attributes);

            // Add record to records
            records.push(record);
        }

        // Write to csv
        for record in records {
            let _ = wtr.serialize(record);
        }

        wtr.flush()?;

        println!("CSV written to {}", output_file);

        Ok(())
    }
}

// Response from the identify endpoint
// Struct mirrors the same structure as response
// Input data is in camelCase, rename it to make it fit the struct fields
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ResponseBody {
    pub object_id: u32,
    pub name: String,
    pub value: String,
    pub location: Location,
    pub properties: Properties,
    pub catalog_items: CatalogItems,
    pub catalog_item_visibilities: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub spatial_reference: SpatialReference,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SpatialReference {
    pub wkid: u32,
    pub latest_wkid: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Properties {
    pub values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogItems {
    pub features: Vec<Feature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    pub attributes: Attributes,
}

// Some of the fields are null sometimes, but not sure which ones, so make all optional
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attributes {
    pub category: Option<u32>,
    pub centerx: Option<f64>,
    pub centery: Option<f64>,
    pub groupname: Option<String>,
    pub highps: Option<f64>,
    pub idp_current_forecast: Option<u32>,
    pub idp_fcst_hour: Option<u32>,
    pub idp_filedate: Option<u64>,
    pub idp_grb_elem: Option<String>,
    pub idp_grb_level: Option<String>,
    pub idp_ingestdate: Option<u64>,
    pub idp_issueddate: Option<u64>,
    pub idp_source: Option<String>,
    pub idp_subset: Option<String>,
    pub idp_time_series: Option<u32>,
    pub idp_validendtime: Option<u64>,
    pub idp_validtime: Option<u64>,
    pub lowps: Option<f64>,
    pub maxps: Option<f64>,
    pub minps: Option<f64>,
    pub name: Option<String>,
    pub objectid: Option<u32>,
    pub productname: Option<String>,
    pub st_area_shape_: Option<f64>,
    pub tag: Option<String>,
    pub zorder: Option<f64>,
}

fn clean_and_snakecase(input: &str) -> String {
    let cleaned = regex::Regex::new(r"\s+")
        .unwrap()
        .replace_all(input, "_")
        .to_string();

    let transformed = cleaned
        .chars()
        .filter(|&c| c.is_alphanumeric() || c == '_')
        .collect::<String>()
        .to_lowercase();

    let trimmed = transformed.trim_matches('_').to_string();

    trimmed
}

pub fn identify(url: &String, point: Point) -> Result<IdentifyResponse, Box<dyn Error>> {
    // Url
    let base_url = url;

    // Specify query params as a hashmap
    let mut params = std::collections::HashMap::new();
    params.insert("geometry", point.to_json_geometry()?);
    params.insert("geometryType", "esriGeometryPoint".to_string());
    params.insert("returnGeometry", "false".to_string());
    params.insert("returnCatalogItems", "true".to_string());
    params.insert("returnPixelValues", "true".to_string());
    params.insert("processAsMultiDimensional", "false".to_string());
    params.insert("f", "json".to_string());

    // Get the current date and time
    // Specify the time zone using the tz database id
    let requested_on = chrono::Utc::now()
        .with_timezone(&chrono_tz::America::Los_Angeles)
        .to_rfc3339();

    // Make a GET request with query params
    let client = reqwest::blocking::Client::new();
    let response = client.get(base_url).query(&params).send()?;

    // Save the url
    let response_url = response.url().clone();

    // Deserialize the JSON string into struct
    let body = response.text()?;
    let deserialize_body: ResponseBody = serde_json::from_str(&body)?;

    // Track request and response details in a variable
    let identify_response = IdentifyResponse {
        point,
        requested_on,
        url: response_url.to_string(),
        body: deserialize_body,
    };

    Ok(identify_response)
}

pub fn combine_csv_files(
    input_files: &Vec<String>,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    // Open the output file
    println!("Opening output file: {}", output_file);
    let mut combined_writer = csv::Writer::from_path(output_file)?;

    for (i, input_file) in input_files.iter().enumerate() {
        // Open the CSV file
        let file = File::open(input_file)?;

        // Read the CSV file
        let mut rdr = csv::Reader::from_reader(file);

        // Get headers
        let headers = rdr.headers()?.clone();

        // Loop over rdr and write to the combined writer
        for (j, result) in rdr.records().enumerate() {
            let record = result?;
            if i == 0 && j == 0 {
                // Writer header row for the first file and first record
                combined_writer.write_record(&headers)?;
            }
            combined_writer.write_record(&record)?;
        }
    }

    combined_writer.flush()?;

    println!("Combined file at {}", output_file);

    Ok(())
}

pub fn process_csv_file(input_file: &str) -> Result<String, PolarsError> {
    // Read the CSV file into a lazyframe
    let mut lf = LazyCsvReader::new(input_file).has_header(true).finish()?;

    println!("Contents of {}: {}", input_file, lf.clone().collect()?);

    // Columns to keep
    let columns_to_keep = [
        col("point_id"),
        col("lat"),
        col("long"),
        col("idp_issueddate_iso"),
        col("idp_validtime_iso"),
        col("value"),
        col("requested_on"),
        // col("url"),
    ];

    // Select issued date col, determine max, and convert to df
    let latest_issued_date = lf
        .clone()
        .select([col("idp_issueddate_iso")])
        .max()
        .collect()?;

    // Get the max issued date value
    let latest_issued_date = latest_issued_date
        .column("idp_issueddate_iso")
        .unwrap()
        .utf8() // Convert col into utf8array, assuming col contains utf8 encoded strings
        .unwrap()
        .get(0)
        .unwrap();

    // Select only the columns to keep and sort
    lf = lf
        .filter(col("idp_issueddate_iso").eq(lit(latest_issued_date)))
        .filter(col("idp_validtime_iso").is_not_null())
        .sort_by_exprs(
            vec![col("point_id"), col("idp_validtime_iso")],
            vec![false, false],
            false,
            false,
        )
        .select(columns_to_keep);

    // Collect into a dataframe
    let mut df = lf.collect()?;

    println!(
        "Filtered for latest issued date, sorted by name and idp_validtime_iso, \
    and removed columns: {}",
        &df
    );

    // Output file path
    let output_file = input_file.replace(".csv", "_processed.csv");

    // Write to CSV
    let file = File::create(&output_file)?;
    let _ = CsvWriter::new(file).has_header(true).finish(&mut df);

    println!("Processed file at {}", &output_file);

    Ok(output_file)
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    println!("Opening input file: {}", args.input);
    let file = File::open(args.input)?;

    // Read the CSV file
    let mut rdr = csv::Reader::from_reader(file);

    // Create a variable to hold each Point
    let mut points: Vec<Point> = Vec::new();

    // Deserialize each record into a Point
    for result in rdr.deserialize() {
        let point: Point = result?;

        // Add point to points
        points.push(point);
    }

    // Get parent dir for output
    let parent_path = std::path::Path::new(&args.output).parent().unwrap();

    // Path to scratch directory
    let scratch_dir_path = parent_path.join("scratch");

    // Create scratch directory
    match std::fs::create_dir(scratch_dir_path.clone()) {
        Ok(_) => println!(
            "Successfully created scratch directory: {}",
            scratch_dir_path.display()
        ),
        Err(e) => eprintln!(
            "Error creating scratch directory: {} - {}",
            scratch_dir_path.display(),
            e
        ),
    };

    // Create a vec to store the filenames in scratch dir
    let mut current_scratch_files: Vec<String> = Vec::new();

    // Attempt to read the scratch dir
    match std::fs::read_dir(scratch_dir_path.clone()) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        let file_name = dir_entry.file_name();

                        current_scratch_files.push(file_name.to_string_lossy().into_owned());
                    }
                    Err(e) => eprintln!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }

    // Create a vec for file names and its associated point
    let mut filenames_and_points: Vec<(String, Point)> = Vec::new();

    // Loop over points and push to vec a tuple of filename and point
    for i in 0..points.len() {
        // File name for point output
        let scratch_file_name = (
            format!(
                "{}_{}_output.csv",
                i,
                clean_and_snakecase(&points[i].point_id)
            ),
            points[i].clone(),
        );

        // Push scratch file name to vec
        filenames_and_points.push(scratch_file_name);
    }

    // Print how many points will be requested
    println!(
        "Requesting values from {} for {} points...",
        args.url,
        &filenames_and_points.len()
    );

    // Loop over the file name and point tuple
    for (file_name, point) in &filenames_and_points {
        // Check if the point has a file in the scratch dir
        if current_scratch_files.contains(file_name) {
            println!(
                "Skipping {} ({}, {}) because {} already exists in {}",
                &point.point_id,
                &point.lat,
                &point.long,
                file_name,
                &scratch_dir_path.display()
            );

            // Skip point because file is in scratch dir
            continue;
        } else {
            println!(
                "Requesting values for {} ({}, {})...",
                &point.point_id, &point.lat, &point.long
            );

            // If not point not in scratch dir, get pixel values at each point
            let response = identify(&args.url, point.clone())?;

            // Concat scratch dir and output file name
            let output_file_path = scratch_dir_path.join(file_name);

            // Process response and output CSV
            response.process_body(&output_file_path.to_str().unwrap())?;
        }
    }

    // Create a vec of only the file names with scratch dir
    let mut input_files = Vec::new();
    for (file_name, _point) in &filenames_and_points {
        input_files.push(format!("{}/{}", scratch_dir_path.display(), file_name));
    }

    // Combine files to one CSV
    let _ = combine_csv_files(&input_files, &args.output)?;

    // Process combined csv if requested
    if args.process {
        let _processed_file_path = process_csv_file(&args.output)?;
    };

    // Clean up scratch dir
    for (file_name, _point) in &filenames_and_points {
        let file_path = scratch_dir_path.join(file_name);

        if let Err(e) = std::fs::remove_file(file_path.clone()) {
            eprintln!("Error deleting file: {} - {}", file_path.display(), e);
        } else {
            println!("File {} successfully deleted", file_path.display());
        }
    }

    Ok(())
}
