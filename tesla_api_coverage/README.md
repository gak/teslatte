# API Coverage

A tool designed to compare the API calls between `teslatte` and the publicly documented Tesla APIs.

**Note:** This tool is bespoke to the build of `teslatte` and is not intended for publishing on crates.io.

This project does (or will do) the following:

* Scrape the teslatte project for what has been implemented.
* Scrape the Fleet API for its list of endpoints.
* Scrape the Command Mode SDK sources for its list of endpoints: https://github.com/teslamotors/vehicle-command/blob/main/cmd/tesla-control/commands.go
* Scrape timdorr/tesla-api's endpoints file: https://github.com/timdorr/tesla-api/blob/master/ownerapi_endpoints.json
* Combine the results into a single list of endpoints.
  * Has a configuration on how to merge the endpoints, e.g. if an endpoint name is different, how to resolve it.
* Output a table of endpoints that are implemented or not, maybe in Markdown.
 

### Brainstorm

Combined format idea:

```json
{
  "honk_horn": {
    
    // If owner-api vs fleet-api methods are different, they should have different entries,
    // otherwise call it "rest":
    "rest": {
      "method": "POST",
      "endpoint": "/vehicles/{vehicle_id}/command/honk_horn"
    },
    "vehicle-command": {
      "endpoint": "honk"
    }
    
    "timdorr-endpoints-file": true,
    "teslatte": true,
    "owners-api": true,
    "fleet-api": true,
  }
}
```