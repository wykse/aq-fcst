# aq-fcst
This project is an exploration of Rust as I learn the language. The goal is to retrieve air quality data from the National Weather Service's ArcGIS REST image service, specifically [air_quality/ndgd_apm25_hr01_bc](https://mapservices.weather.noaa.gov/raster/rest/services/air_quality/ndgd_apm25_hr01_bc/ImageServer).

Learn more about [NWS Air Quality Forecast Guidance](https://vlab.noaa.gov/web/osti-modeling/air-quality).

## Description
`aq-fcst` is a command-line program to get pixel values from an ArcGIS REST image service. The program takes a CSV file of points as input and outputs the results as a CSV. It can also process the output if the option is included.

See [input_example.csv](examples/input/input_example.csv) for example input.

## Usage and options
```
aq-fcst.exe [OPTIONS] <INPUT> <OUTPUT>
```
```
Get NOAA's Air Quality Forecast Guidance as a CSV

Usage: aq-fcst.exe [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Path to CSV containing input points; header must have following field names: point_id, lat, long
  <OUTPUT>  Path to output CSV

Options:
  -u, --url <URL>  Url for ArcGIS image service identify endpoint [default: https://mapservices.weather.noaa.gov/raster/rest/services/air_quality/ndgd_apm25_hr01_bc/ImageServer/identify]
  -p, --process    Output an additional CSV that is processed, e.g., your_output_processed.csv. File is filtered for latest issuance and sorted ascending
  -h, --help       Print help
  -V, --version    Print version
```

### Example with processed output
```
.\aq-fcst.exe .\examples\input\input_example.csv .\examples\output\apm25_hr01_bc_output_example.csv -u https://mapservices.weather.noaa.gov/raster/rest/services/air_quality/ndgd_apm25_hr01_bc/ImageServer/identify -p
```

```
Opening input file: .\examples\input\input_example.csv
Successfully created scratch directory: .\examples\output\scratch
Requesting values from https://mapservices.weather.noaa.gov/raster/rest/services/air_quality/ndgd_apm25_hr01_bc/ImageServer/identify for 3 points...
Requesting values for Yosemite (37.7959, -119.6494)...
CSV written to .\examples\output\scratch\0_yosemite_output.csv
Requesting values for Los Angeles (33.948, -118.2567)...
CSV written to .\examples\output\scratch\1_los_angeles_output.csv
Requesting values for 48 miles E of Fresno (36.7, -119.04)...
CSV written to .\examples\output\scratch\2_48_miles_e_of_fresno_output.csv
Opening output file: .\examples\output\apm25_hr01_bc_output_example.csv
Combined file at .\examples\output\apm25_hr01_bc_output_example.csv
Contents of .\examples\output\apm25_hr01_bc_output_example.csv: shape: (432, 34)
┌──────────────────────┬─────────┬───────────┬───────────────────────────┬───┬─────────────┬────────────────┬─────────┬────────┐
│ point_id             ┆ lat     ┆ long      ┆ idp_issueddate_iso        ┆ … ┆ productname ┆ st_area_shape_ ┆ tag     ┆ zorder │
│ ---                  ┆ ---     ┆ ---       ┆ ---                       ┆   ┆ ---         ┆ ---            ┆ ---     ┆ ---    │
│ str                  ┆ f64     ┆ f64       ┆ str                       ┆   ┆ str         ┆ f64            ┆ str     ┆ str    │
╞══════════════════════╪═════════╪═══════════╪═══════════════════════════╪═══╪═════════════╪════════════════╪═════════╪════════╡
│ Yosemite             ┆ 37.7959 ┆ -119.6494 ┆ 2023-11-10T22:00:00-08:00 ┆ … ┆ null        ┆ 0.0            ┆ Dataset ┆ null   │
│ Yosemite             ┆ 37.7959 ┆ -119.6494 ┆ 2023-11-10T22:00:00-08:00 ┆ … ┆ null        ┆ 0.0            ┆ Dataset ┆ null   │
│ Yosemite             ┆ 37.7959 ┆ -119.6494 ┆ 2023-11-11T04:00:00-08:00 ┆ … ┆ null        ┆ 0.0            ┆ Dataset ┆ null   │
│ Yosemite             ┆ 37.7959 ┆ -119.6494 ┆ 2023-11-11T04:00:00-08:00 ┆ … ┆ null        ┆ 0.0            ┆ Dataset ┆ null   │
│ …                    ┆ …       ┆ …         ┆ …                         ┆ … ┆ …           ┆ …              ┆ …       ┆ …      │
│ 48 miles E of Fresno ┆ 36.7    ┆ -119.04   ┆ 2023-11-10T22:00:00-08:00 ┆ … ┆ null        ┆ 0.0            ┆ Dataset ┆ null   │
│ 48 miles E of Fresno ┆ 36.7    ┆ -119.04   ┆ 2023-11-11T04:00:00-08:00 ┆ … ┆ null        ┆ 0.0            ┆ Dataset ┆ null   │
│ 48 miles E of Fresno ┆ 36.7    ┆ -119.04   ┆ 2023-11-10T22:00:00-08:00 ┆ … ┆ null        ┆ 0.0            ┆ Dataset ┆ null   │
│ 48 miles E of Fresno ┆ 36.7    ┆ -119.04   ┆ 2023-11-11T04:00:00-08:00 ┆ … ┆ null        ┆ 0.0            ┆ Dataset ┆ null   │
└──────────────────────┴─────────┴───────────┴───────────────────────────┴───┴─────────────┴────────────────┴─────────┴────────┘
Filtered for latest issued date, sorted by name and idp_validtime_iso, and removed columns: shape: (216, 8)
┌──────────────────────┬─────────┬───────────┬───────────────────────────┬───────────────────────────┬───────┬────────────────────┬───────────────────────────────────┐
│ point_id             ┆ lat     ┆ long      ┆ idp_issueddate_iso        ┆ idp_validtime_iso         ┆ value ┆ name               ┆ requested_on                      │
│ ---                  ┆ ---     ┆ ---       ┆ ---                       ┆ ---                       ┆ ---   ┆ ---                ┆ ---                               │
│ str                  ┆ f64     ┆ f64       ┆ str                       ┆ str                       ┆ i64   ┆ str                ┆ str                               │
╞══════════════════════╪═════════╪═══════════╪═══════════════════════════╪═══════════════════════════╪═══════╪════════════════════╪═══════════════════════════════════╡
│ Yosemite             ┆ 37.7959 ┆ -119.6494 ┆ 2023-11-11T04:00:00-08:00 ┆ 2023-11-11T05:00:00-08:00 ┆ 3     ┆ ds_apm25h01_bc_B01 ┆ 2023-11-11T19:57:16.485017900-08… │
│ Los Angeles          ┆ 33.948  ┆ -118.2567 ┆ 2023-11-11T04:00:00-08:00 ┆ 2023-11-11T05:00:00-08:00 ┆ 22    ┆ ds_apm25h01_bc_B01 ┆ 2023-11-11T19:57:22.096321400-08… │
│ 48 miles E of Fresno ┆ 36.7    ┆ -119.04   ┆ 2023-11-11T04:00:00-08:00 ┆ 2023-11-11T05:00:00-08:00 ┆ 62    ┆ ds_apm25h01_bc_B01 ┆ 2023-11-11T19:57:27.048402800-08… │
│ Yosemite             ┆ 37.7959 ┆ -119.6494 ┆ 2023-11-11T04:00:00-08:00 ┆ 2023-11-11T06:00:00-08:00 ┆ 3     ┆ ds_apm25h01_bc_B02 ┆ 2023-11-11T19:57:16.485017900-08… │
│ …                    ┆ …       ┆ …         ┆ …                         ┆ …                         ┆ …     ┆ …                  ┆ …                                 │
│ 48 miles E of Fresno ┆ 36.7    ┆ -119.04   ┆ 2023-11-11T04:00:00-08:00 ┆ 2023-11-14T03:00:00-08:00 ┆ 41    ┆ ds_apm25h01_bc_B71 ┆ 2023-11-11T19:57:27.048402800-08… │
│ Yosemite             ┆ 37.7959 ┆ -119.6494 ┆ 2023-11-11T04:00:00-08:00 ┆ 2023-11-14T04:00:00-08:00 ┆ 5     ┆ ds_apm25h01_bc_B72 ┆ 2023-11-11T19:57:16.485017900-08… │
│ Los Angeles          ┆ 33.948  ┆ -118.2567 ┆ 2023-11-11T04:00:00-08:00 ┆ 2023-11-14T04:00:00-08:00 ┆ 35    ┆ ds_apm25h01_bc_B72 ┆ 2023-11-11T19:57:22.096321400-08… │
│ 48 miles E of Fresno ┆ 36.7    ┆ -119.04   ┆ 2023-11-11T04:00:00-08:00 ┆ 2023-11-14T04:00:00-08:00 ┆ 32    ┆ ds_apm25h01_bc_B72 ┆ 2023-11-11T19:57:27.048402800-08… │
└──────────────────────┴─────────┴───────────┴───────────────────────────┴───────────────────────────┴───────┴────────────────────┴───────────────────────────────────┘
Processed file at .\examples\output\apm25_hr01_bc_output_example_processed.csv
File .\examples\output\scratch\0_yosemite_output.csv successfully deleted
File .\examples\output\scratch\1_los_angeles_output.csv successfully deleted
File .\examples\output\scratch\2_48_miles_e_of_fresno_output.csv successfully deleted
Time taken: 15s

```
