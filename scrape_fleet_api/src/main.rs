use clap::{Command, Parser};
use scraper::{Element, ElementRef, Html, Selector};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Cli {
    /// Only use the cached html to avoid making a request.
    #[clap(short, long)]
    cache: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();

    let html = get_and_save_html(&args).await;

    let fleet_api = parse(&html);
}

async fn get_and_save_html(args: &Cli) -> String {
    // Write to where this project root is, not in the parent project.
    let mut path = PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push("fleet.html");

    if args.cache {
        return std::fs::read_to_string(path).unwrap();
    }

    let response = reqwest::get("https://developer.tesla.com/docs/fleet-api")
        .await
        .unwrap();

    let html = response.text().await.unwrap();

    std::fs::write(path, &html).unwrap();

    html
}

struct FleetApiSpec {
    calls: HashMap<String, Call>,
}

// e.g. serialize to similar: vehicle-endpoints
#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
enum Category {
    ChargingEndpoints,
    PartnerEndpoints,
    UserEndpoints,
    VehicleCommands,
    VehicleEndpoints,
}

/*
Profile Information 	user_data 	Contact information, home address, profile picture, and referral information
Vehicle Information 	vehicle_device_data 	Vehicle live data, location, eligible upgrades, nearby superchargers, ownership, and service scheduling data
Vehicle Commands 	vehicle_cmds 	Commands like add/remove driver, access Live Camera, unlock, wake up, remote start, and schedule software updates
Vehicle Charging Management 	vehicle_charging_cmds 	Vehicle charging history, billed amount, charging location, commands to schedule, and start/stop charging
Energy Product Information 	energy_device_data 	Energy flow history, saving forecast, tariff rates, grid import, calendar, site status, time of use, and ownership
Energy Product Commands 	energy_cmds 	Commands like update storm mode
 */

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Scope {
    /// Profile Information
    ///
    /// Contact information, home address, profile picture, and referral information.
    UserData,

    /// Vehicle Information
    ///
    /// Vehicle live data, location, eligible upgrades, nearby superchargers, ownership, and service scheduling data.
    VehicleDeviceData,

    /// Vehicle Commands
    ///
    /// Commands like add/remove driver, access Live Camera, unlock, wake up, remote start, and schedule software updates.
    VehicleCmds,

    /// Vehicle Charging Management
    ///
    /// Vehicle charging history, billed amount, charging location, commands to schedule, and start/stop charging.
    VehicleChargingCmds,

    /// Energy Product Information
    ///
    /// Energy flow history, saving forecast, tariff rates, grid import, calendar, site status, time of use, and ownership.
    EnergyDeviceData,

    /// Energy Product Commands
    ///
    /// Commands like update storm mode.
    EnergyCmds,
}

/*
Name 	In 	Type 	Required 	Description
vin 	query 	string 	No 	VIN
startTime 	query 	string 	No 	StartTime
endTime 	query 	string 	No 	EndTime
 */

enum InRequestData {
    Query,
    Body,
}

struct Parameter {
    name: String,
    request: InRequestData,
    var_type: String,
    required: bool,
    description: String,
}

struct Call {
    name: String,
    method: reqwest::Method,
    url_definition: String,
    description: String,
    category: Category,
    scopes: Vec<Scope>,
    parameters: Vec<Parameter>,
    request_example: String,
    response_example: String,
}

/*
Example HTML docs for two calls:


<h1 id='vehicle-commands'>Vehicle Commands</h1><h2 id='actuate_trunk'>actuate_trunk</h2>
<p><span class="endpoint"><code>POST /api/1/vehicles/{id}/command/actuate_trunk</code></span></p>

<p>scopes: <em>vehicle_cmds</em></p>
<div class="highlight"><pre class="highlight shell tab-shell"><code>curl <span class="nt">--header</span> <span class="s1">'Content-Type: application/json'</span> <span class="se">\</span>
  <span class="nt">--header</span> <span class="s2">"Authorization: Bearer </span><span class="nv">$TESLA_API_TOKEN</span><span class="s2">"</span> <span class="se">\</span>
  <span class="nt">--data</span> <span class="s1">'{"which_trunk":"string"}'</span> <span class="se">\</span>
  <span class="s1">'https://fleet-api.prd.na.vn.cloud.tesla.com/api/1/vehicles/{id}/command/actuate_trunk'</span>
</code></pre></div><div class="highlight"><pre class="highlight javascript tab-javascript"><code><span class="kd">const</span> <span class="nx">myHeaders</span> <span class="o">=</span> <span class="k">new</span> <span class="nx">Headers</span><span class="p">();</span>
<span class="nx">myHeaders</span><span class="p">.</span><span class="nx">append</span><span class="p">(</span><span class="dl">"</span><span class="s2">Content-Type</span><span class="dl">"</span><span class="p">,</span> <span class="dl">"</span><span class="s2">application/json</span><span class="dl">"</span><span class="p">);</span>
<span class="nx">myHeaders</span><span class="p">.</span><span class="nx">append</span><span class="p">(</span><span class="dl">"</span><span class="s2">Authorization</span><span class="dl">"</span><span class="p">,</span> <span class="s2">`Bearer </span><span class="p">${</span><span class="nx">process</span><span class="p">.</span><span class="nx">env</span><span class="p">.</span><span class="nx">TESLA_API_TOKEN</span><span class="p">}</span><span class="s2">`</span><span class="p">);</span>

<span class="kd">const</span> <span class="nx">body</span> <span class="o">=</span> <span class="nx">JSON</span><span class="p">.</span><span class="nx">stringify</span><span class="p">({</span>
   <span class="dl">"</span><span class="s2">which_trunk</span><span class="dl">"</span><span class="p">:</span> <span class="dl">"</span><span class="s2">string</span><span class="dl">"</span>
<span class="p">});</span>

<span class="kd">const</span> <span class="nx">requestOptions</span> <span class="o">=</span> <span class="p">{</span>
   <span class="na">method</span><span class="p">:</span> <span class="dl">'</span><span class="s1">POST</span><span class="dl">'</span><span class="p">,</span>
   <span class="na">headers</span><span class="p">:</span> <span class="nx">myHeaders</span><span class="p">,</span>
   <span class="nx">body</span><span class="p">,</span>
   <span class="na">redirect</span><span class="p">:</span> <span class="dl">'</span><span class="s1">follow</span><span class="dl">'</span>
<span class="p">};</span>

<span class="nx">fetch</span><span class="p">(</span><span class="dl">"</span><span class="s2">https://fleet-api.prd.na.vn.cloud.tesla.com/api/1/vehicles/{id}/command/actuate_trunk</span><span class="dl">"</span><span class="p">,</span> <span class="nx">requestOptions</span><span class="p">)</span>
   <span class="p">.</span><span class="nx">then</span><span class="p">(</span><span class="nx">response</span> <span class="o">=&gt;</span> <span class="nx">response</span><span class="p">.</span><span class="nx">json</span><span class="p">())</span>
   <span class="p">.</span><span class="nx">then</span><span class="p">(</span><span class="nx">result</span> <span class="o">=&gt;</span> <span class="nx">console</span><span class="p">.</span><span class="nx">log</span><span class="p">(</span><span class="nx">result</span><span class="p">))</span>
   <span class="p">.</span><span class="k">catch</span><span class="p">(</span><span class="nx">error</span> <span class="o">=&gt;</span> <span class="nx">console</span><span class="p">.</span><span class="nx">log</span><span class="p">(</span><span class="dl">'</span><span class="s1">error</span><span class="dl">'</span><span class="p">,</span> <span class="nx">error</span><span class="p">));</span>
</code></pre></div><div class="highlight"><pre class="highlight ruby tab-ruby"><code><span class="nb">require</span> <span class="s2">"uri"</span>
<span class="nb">require</span> <span class="s2">"json"</span>
<span class="nb">require</span> <span class="s2">"net/http"</span>

<span class="n">url</span> <span class="o">=</span> <span class="no">URI</span><span class="p">(</span><span class="s2">"https://fleet-api.prd.na.vn.cloud.tesla.com/api/1/vehicles/{id}/command/actuate_trunk"</span><span class="p">)</span>

<span class="n">https</span> <span class="o">=</span> <span class="no">Net</span><span class="o">::</span><span class="no">HTTP</span><span class="p">.</span><span class="nf">new</span><span class="p">(</span><span class="n">url</span><span class="p">.</span><span class="nf">host</span><span class="p">,</span> <span class="n">url</span><span class="p">.</span><span class="nf">port</span><span class="p">)</span>
<span class="n">https</span><span class="p">.</span><span class="nf">use_ssl</span> <span class="o">=</span> <span class="kp">true</span>

<span class="n">request</span> <span class="o">=</span> <span class="no">Net</span><span class="o">::</span><span class="no">HTTP</span><span class="o">::</span><span class="no">Post</span><span class="p">.</span><span class="nf">new</span><span class="p">(</span><span class="n">url</span><span class="p">)</span>
<span class="n">request</span><span class="p">[</span><span class="s2">"Content-Type"</span><span class="p">]</span> <span class="o">=</span> <span class="s2">"application/json"</span>
<span class="n">request</span><span class="p">[</span><span class="s2">"Authorization"</span><span class="p">]</span> <span class="o">=</span> <span class="s2">"Bearer ENV_TESLA_API_TOKEN"</span>
<span class="n">request</span><span class="p">.</span><span class="nf">body</span> <span class="o">=</span> <span class="no">JSON</span><span class="p">.</span><span class="nf">dump</span><span class="p">({</span>
   <span class="s2">"which_trunk"</span><span class="p">:</span> <span class="s2">"string"</span>
<span class="p">})</span>

<span class="n">response</span> <span class="o">=</span> <span class="n">https</span><span class="p">.</span><span class="nf">request</span><span class="p">(</span><span class="n">request</span><span class="p">)</span>
<span class="nb">puts</span> <span class="n">response</span><span class="p">.</span><span class="nf">read_body</span>

</code></pre></div><div class="highlight"><pre class="highlight python tab-python"><code><span class="kn">import</span> <span class="nn">os</span>
<span class="kn">import</span> <span class="nn">http.client</span>
<span class="kn">import</span> <span class="nn">json</span>

<span class="n">conn</span> <span class="o">=</span> <span class="n">http</span><span class="p">.</span><span class="n">client</span><span class="p">.</span><span class="n">HTTPSConnection</span><span class="p">(</span><span class="s">"fleet-api.prd.na.vn.cloud.tesla.com"</span><span class="p">)</span>
<span class="n">payload</span> <span class="o">=</span> <span class="n">json</span><span class="p">.</span><span class="n">dumps</span><span class="p">({</span>
   <span class="s">"which_trunk"</span><span class="p">:</span> <span class="s">"string"</span>
<span class="p">})</span>
<span class="n">headers</span> <span class="o">=</span> <span class="p">{</span>
   <span class="s">'Content-Type'</span><span class="p">:</span> <span class="s">'application/json'</span><span class="p">,</span>
   <span class="s">'Authorization'</span><span class="p">:</span> <span class="s">'Bearer ENV_TESLA_API_TOKEN'</span>
<span class="p">}</span>
<span class="n">conn</span><span class="p">.</span><span class="n">request</span><span class="p">(</span><span class="s">"POST"</span><span class="p">,</span> <span class="s">"/api/1/vehicles/{id}/command/actuate_trunk"</span><span class="p">,</span> <span class="n">payload</span><span class="p">,</span> <span class="n">headers</span><span class="p">)</span>
<span class="n">res</span> <span class="o">=</span> <span class="n">conn</span><span class="p">.</span><span class="n">getresponse</span><span class="p">()</span>
<span class="n">data</span> <span class="o">=</span> <span class="n">res</span><span class="p">.</span><span class="n">read</span><span class="p">()</span>
<span class="k">print</span><span class="p">(</span><span class="n">data</span><span class="p">.</span><span class="n">decode</span><span class="p">(</span><span class="s">"utf-8"</span><span class="p">))</span>
</code></pre></div>
<p>Controls the front (which_trunk: &quot;front&quot;) or rear (which_trunk: &quot;rear&quot;) trunk.</p>
<h3 id='parameters-10'>Parameters</h3>
<table><thead>
<tr>
<th>Name</th>
<th>In</th>
<th>Type</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead><tbody>
<tr>
<td>id</td>
<td>path</td>
<td>integer</td>
<td>Yes</td>
<td>vehicle id</td>
</tr>
<tr>
<td>which_trunk</td>
<td>body</td>
<td>string</td>
<td>Yes</td>
<td></td>
</tr>
</tbody></table>
<div class="highlight"><pre class="highlight json tab-json"><code><details><summary> Click to view successful response</summary><span class="p">{</span><span class="w">
 </span><span class="nl">"result"</span><span class="p">:</span><span class="w"> </span><span class="kc">true</span><span class="p">,</span><span class="w">
 </span><span class="nl">"reason"</span><span class="p">:</span><span class="w"> </span><span class="s2">""</span><span class="w">
</span><span class="p">}</span><span class="w">
</span></details></code></pre></div>

<h2 id='adjust_volume'>adjust_volume</h2>
<p><span class="endpoint"><code>POST /api/1/vehicles/{id}/command/adjust_volume</code></span></p>

<p>scopes: <em>vehicle_cmds</em></p>
<div class="highlight"><pre class="highlight shell tab-shell"><code>curl <span class="nt">--header</span> <span class="s1">'Content-Type: application/json'</span> <span class="se">\</span>
  <span class="nt">--header</span> <span class="s2">"Authorization: Bearer </span><span class="nv">$TESLA_API_TOKEN</span><span class="s2">"</span> <span class="se">\</span>
  <span class="nt">--data</span> <span class="s1">'{"volume":"integer"}'</span> <span class="se">\</span>
  <span class="s1">'https://fleet-api.prd.na.vn.cloud.tesla.com/api/1/vehicles/{id}/command/adjust_volume'</span>
</code></pre></div><div class="highlight"><pre class="highlight javascript tab-javascript"><code><span class="kd">const</span> <span class="nx">myHeaders</span> <span class="o">=</span> <span class="k">new</span> <span class="nx">Headers</span><span class="p">();</span>
<span class="nx">myHeaders</span><span class="p">.</span><span class="nx">append</span><span class="p">(</span><span class="dl">"</span><span class="s2">Content-Type</span><span class="dl">"</span><span class="p">,</span> <span class="dl">"</span><span class="s2">application/json</span><span class="dl">"</span><span class="p">);</span>
<span class="nx">myHeaders</span><span class="p">.</span><span class="nx">append</span><span class="p">(</span><span class="dl">"</span><span class="s2">Authorization</span><span class="dl">"</span><span class="p">,</span> <span class="s2">`Bearer </span><span class="p">${</span><span class="nx">process</span><span class="p">.</span><span class="nx">env</span><span class="p">.</span><span class="nx">TESLA_API_TOKEN</span><span class="p">}</span><span class="s2">`</span><span class="p">);</span>

<span class="kd">const</span> <span class="nx">body</span> <span class="o">=</span> <span class="nx">JSON</span><span class="p">.</span><span class="nx">stringify</span><span class="p">({</span>
   <span class="dl">"</span><span class="s2">volume</span><span class="dl">"</span><span class="p">:</span> <span class="dl">"</span><span class="s2">integer</span><span class="dl">"</span>
<span class="p">});</span>

<span class="kd">const</span> <span class="nx">requestOptions</span> <span class="o">=</span> <span class="p">{</span>
   <span class="na">method</span><span class="p">:</span> <span class="dl">'</span><span class="s1">POST</span><span class="dl">'</span><span class="p">,</span>
   <span class="na">headers</span><span class="p">:</span> <span class="nx">myHeaders</span><span class="p">,</span>
   <span class="nx">body</span><span class="p">,</span>
   <span class="na">redirect</span><span class="p">:</span> <span class="dl">'</span><span class="s1">follow</span><span class="dl">'</span>
<span class="p">};</span>

<span class="nx">fetch</span><span class="p">(</span><span class="dl">"</span><span class="s2">https://fleet-api.prd.na.vn.cloud.tesla.com/api/1/vehicles/{id}/command/adjust_volume</span><span class="dl">"</span><span class="p">,</span> <span class="nx">requestOptions</span><span class="p">)</span>
   <span class="p">.</span><span class="nx">then</span><span class="p">(</span><span class="nx">response</span> <span class="o">=&gt;</span> <span class="nx">response</span><span class="p">.</span><span class="nx">json</span><span class="p">())</span>
   <span class="p">.</span><span class="nx">then</span><span class="p">(</span><span class="nx">result</span> <span class="o">=&gt;</span> <span class="nx">console</span><span class="p">.</span><span class="nx">log</span><span class="p">(</span><span class="nx">result</span><span class="p">))</span>
   <span class="p">.</span><span class="k">catch</span><span class="p">(</span><span class="nx">error</span> <span class="o">=&gt;</span> <span class="nx">console</span><span class="p">.</span><span class="nx">log</span><span class="p">(</span><span class="dl">'</span><span class="s1">error</span><span class="dl">'</span><span class="p">,</span> <span class="nx">error</span><span class="p">));</span>
</code></pre></div><div class="highlight"><pre class="highlight ruby tab-ruby"><code><span class="nb">require</span> <span class="s2">"uri"</span>
<span class="nb">require</span> <span class="s2">"json"</span>
<span class="nb">require</span> <span class="s2">"net/http"</span>

<span class="n">url</span> <span class="o">=</span> <span class="no">URI</span><span class="p">(</span><span class="s2">"https://fleet-api.prd.na.vn.cloud.tesla.com/api/1/vehicles/{id}/command/adjust_volume"</span><span class="p">)</span>

<span class="n">https</span> <span class="o">=</span> <span class="no">Net</span><span class="o">::</span><span class="no">HTTP</span><span class="p">.</span><span class="nf">new</span><span class="p">(</span><span class="n">url</span><span class="p">.</span><span class="nf">host</span><span class="p">,</span> <span class="n">url</span><span class="p">.</span><span class="nf">port</span><span class="p">)</span>
<span class="n">https</span><span class="p">.</span><span class="nf">use_ssl</span> <span class="o">=</span> <span class="kp">true</span>

<span class="n">request</span> <span class="o">=</span> <span class="no">Net</span><span class="o">::</span><span class="no">HTTP</span><span class="o">::</span><span class="no">Post</span><span class="p">.</span><span class="nf">new</span><span class="p">(</span><span class="n">url</span><span class="p">)</span>
<span class="n">request</span><span class="p">[</span><span class="s2">"Content-Type"</span><span class="p">]</span> <span class="o">=</span> <span class="s2">"application/json"</span>
<span class="n">request</span><span class="p">[</span><span class="s2">"Authorization"</span><span class="p">]</span> <span class="o">=</span> <span class="s2">"Bearer ENV_TESLA_API_TOKEN"</span>
<span class="n">request</span><span class="p">.</span><span class="nf">body</span> <span class="o">=</span> <span class="no">JSON</span><span class="p">.</span><span class="nf">dump</span><span class="p">({</span>
   <span class="s2">"volume"</span><span class="p">:</span> <span class="s2">"integer"</span>
<span class="p">})</span>

<span class="n">response</span> <span class="o">=</span> <span class="n">https</span><span class="p">.</span><span class="nf">request</span><span class="p">(</span><span class="n">request</span><span class="p">)</span>
<span class="nb">puts</span> <span class="n">response</span><span class="p">.</span><span class="nf">read_body</span>

</code></pre></div><div class="highlight"><pre class="highlight python tab-python"><code><span class="kn">import</span> <span class="nn">os</span>
<span class="kn">import</span> <span class="nn">http.client</span>
<span class="kn">import</span> <span class="nn">json</span>

<span class="n">conn</span> <span class="o">=</span> <span class="n">http</span><span class="p">.</span><span class="n">client</span><span class="p">.</span><span class="n">HTTPSConnection</span><span class="p">(</span><span class="s">"fleet-api.prd.na.vn.cloud.tesla.com"</span><span class="p">)</span>
<span class="n">payload</span> <span class="o">=</span> <span class="n">json</span><span class="p">.</span><span class="n">dumps</span><span class="p">({</span>
   <span class="s">"volume"</span><span class="p">:</span> <span class="s">"integer"</span>
<span class="p">})</span>
<span class="n">headers</span> <span class="o">=</span> <span class="p">{</span>
   <span class="s">'Content-Type'</span><span class="p">:</span> <span class="s">'application/json'</span><span class="p">,</span>
   <span class="s">'Authorization'</span><span class="p">:</span> <span class="s">'Bearer ENV_TESLA_API_TOKEN'</span>
<span class="p">}</span>
<span class="n">conn</span><span class="p">.</span><span class="n">request</span><span class="p">(</span><span class="s">"POST"</span><span class="p">,</span> <span class="s">"/api/1/vehicles/{id}/command/adjust_volume"</span><span class="p">,</span> <span class="n">payload</span><span class="p">,</span> <span class="n">headers</span><span class="p">)</span>
<span class="n">res</span> <span class="o">=</span> <span class="n">conn</span><span class="p">.</span><span class="n">getresponse</span><span class="p">()</span>
<span class="n">data</span> <span class="o">=</span> <span class="n">res</span><span class="p">.</span><span class="n">read</span><span class="p">()</span>
<span class="k">print</span><span class="p">(</span><span class="n">data</span><span class="p">.</span><span class="n">decode</span><span class="p">(</span><span class="s">"utf-8"</span><span class="p">))</span>
</code></pre></div>
<p>Adjusts vehicle media playback volume.</p>
<h3 id='parameters-11'>Parameters</h3>
<table><thead>
<tr>
<th>Name</th>
<th>In</th>
<th>Type</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead><tbody>
<tr>
<td>id</td>
<td>path</td>
<td>integer</td>
<td>Yes</td>
<td>vehicle id</td>
</tr>
<tr>
<td>volume</td>
<td>body</td>
<td>integer</td>
<td>Yes</td>
<td></td>
</tr>
</tbody></table>
<div class="highlight"><pre class="highlight json tab-json"><code><details><summary> Click to view successful response</summary><span class="p">{</span><span class="w">
 </span><span class="nl">"result"</span><span class="p">:</span><span class="w"> </span><span class="kc">true</span><span class="p">,</span><span class="w">
 </span><span class="nl">"reason"</span><span class="p">:</span><span class="w"> </span><span class="s2">""</span><span class="w">
</span><span class="p">}</span><span class="w">
</span></details></code></pre></div>

 */

fn parse(html: &str) -> FleetApiSpec {
    let document = Html::parse_document(html);
    let content_selector = selector(".content h1");
    let mut element = document.select(&content_selector).next().unwrap();
    let mut category = None;

    // Iterate over all the elements in the content section until we see a h1 or h2.
    loop {
        match element.value().name() {
            "h1" => {
                let category_name = element.value().id().unwrap();
                category = Category::from_str(&category_name).ok();
            }
            "h2" => {
                if category.is_some() {
                    let name = element.inner_html();
                    println!("{category:?} {name:?}");
                    // let call = parse_call(element);
                }
            }
            _ => {}
        }

        let Some(next_element) = element.next_sibling_element() else {
            println!("exiting...");
            break;
        };
        element = next_element;
    }

    todo!()
}

/// Return None if this is not an endpoint.
///
/// Will panic if it looks like an endpoint and has trouble parsing.
fn parse_call(element: ElementRef) -> Option<Call> {
    let name = element.value().id().unwrap();

    // <p><span class="endpoint"><code>POST /api/1/vehicles/{id}/command/auto_conditioning_start</code></span></p>
    // This section determines if this is an endpoint or not.
    let (fragment, element) = next(element);
    let url = fragment.select(&selector("code")).next()?.inner_html();
    if !url.starts_with("GET ") && !url.starts_with("POST ") {
        return None;
    }

    let (method, url) = url.split_once(' ').unwrap();
    println!("{} {}", method, url);

    // <p>scopes: <em>vehicle_cmds</em></p>
    let (fragment, element) = next(element);
    let scopes = fragment
        .select(&selector("em"))
        .map(|e| e.inner_html())
        .map(|e| Scope::from_str(&e))
        .collect::<Vec<_>>();

    // 4 <div class="highlight"> nodes containing example requests in different languages.
    // TODO: Skip for now
    let mut count = 0;
    let mut element = element;
    loop {
        let (fragment, new_element) = next(element);
        element = new_element;
        if fragment
            .select(&selector(r#"div[class="highlight"]"#))
            .next()
            .is_none()
        {
            break;
        }

        count += 1;
        if count == 10 {
            panic!("Too many examples");
        }
    }
    if count == 0 && name != "api-status" {
        panic!("No examples for {}", name);
    }

    None
}

fn next(element: ElementRef) -> (Html, ElementRef) {
    let element = element.next_sibling_element().unwrap();
    let html = Html::parse_fragment(&element.html());
    (html, element)
}

fn selector(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}
