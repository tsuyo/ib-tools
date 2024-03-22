use clap::Parser;
use reqwest::Client;
use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Build {
    build_id: String,
    initiator_id: String,
    initiator_name: String,
    start_time: String,
    end_time: String,
    duration: usize,
    build_group: String,
    build_title: Option<String>,
    build_status: Option<String>,
    helpers: Option<Vec<String>>,
    command: String,
    total_working_helpers: usize,
    max_initiator_cores: usize,
    avg_initiator_cores: f64,
    max_concurrent_working_helpers: Option<usize>,
    avg_concurrent_working_helpers: Option<f64>,
    avg_busy_helpers_cores: f64,
    max_busy_helpers_cores: usize,
    max_needed_helper_cores: usize,
    avg_needed_helper_cores: f64,
    number_of_local_tasks: usize,
    number_of_remote_tasks: usize,
    number_of_cloud_tasks: usize,
    remote_core_time: f64,
    created_at: String,
    coordinator_id: String,
    build_priority: u32,
    avg_busy_cloud_helpers_cores: f64,
    max_busy_cloud_helpers_cores: usize,
    build_type: String,
    core_limit: usize,
    total_cacheable_tasks: Option<usize>,
    cache_task_hits: Option<usize>,
    cache_task_miss: Option<usize>,
    cache_initiator_role: Option<usize>,
    saved_time: Option<usize>,
    cache_service_endpoint: Option<String>,
    cache_license_allowed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct History {
    total_count: u32,
    builds_count: u32,
    next_page_token: Option<String>,
    builds: Vec<Build>,
}

#[derive(Parser)]
struct BuildHistoryApi {
    // #[arg(short = 'k', long = "api-key")]
    #[arg(short = 'k', long)]
    api_key: String,
    #[arg(short = 'c', long, help = "<hostname or ip>:8000")]
    coord_host: String,
    #[arg(short = 'i', long)]
    coord_id: String,

    #[arg(long)]
    build_start_time_from: Option<String>,
    #[arg(long)]
    build_end_time_to: Option<String>,
    #[arg(long)]
    initiator_id: Option<String>,
    #[arg(long)]
    build_group: Option<String>,
    #[arg(long)]
    build_title: Option<String>,
    #[arg(long)]
    build_status: Option<String>,
    #[arg(long)]
    build_duration_from: Option<usize>,
    #[arg(long)]
    build_duration_to: Option<usize>,
    #[arg(long)]
    build_type: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = BuildHistoryApi::parse();
    // println!(
    //     "api_key: {:?}, coord_host: {:?}, coord_id: {:?}",
    //     args.api_key, args.coord_host, args.coord_id
    // );

    // Building a request URL BEGIN
    let mut request_url_base = format!(
        "https://{coord_host}/api/builds?coordinatorId={coord_id}",
        coord_host = args.coord_host,
        coord_id = args.coord_id
    );
    if let Some(build_start_time_from) = args.build_start_time_from.as_deref() {
        request_url_base = format!("{}&buildStartTimeFrom={}", request_url_base, build_start_time_from);
    }
    if let Some(build_end_time_to) = args.build_end_time_to.as_deref() {
        request_url_base = format!("{}&buildEndTimeTo={}", request_url_base, build_end_time_to);
    }
    if let Some(initiator_id) = args.initiator_id.as_deref() {
        request_url_base = format!("{}&initiatorID={}", request_url_base, initiator_id);
    }
    if let Some(build_group) = args.build_group.as_deref() {
        request_url_base = format!("{}&buildGroup={}", request_url_base, build_group);
    }
    if let Some(build_title) = args.build_title.as_deref() {
        request_url_base = format!("{}&buildTitle={}", request_url_base, build_title);
    }
    if let Some(build_status) = args.build_status.as_deref() {
        request_url_base = format!("{}&buildStatus={}", request_url_base, build_status);
    }
    if let Some(build_duration_from) = args.build_duration_from {
        request_url_base = format!("{}&buildDurationFrom={}", request_url_base, build_duration_from);
    }
    if let Some(build_duration_to) = args.build_duration_to {
        request_url_base = format!("{}&buildDurationTo={}", request_url_base, build_duration_to);
    }
    if let Some(build_type) = args.build_type.as_deref() {
        request_url_base = format!("{}&buildType={}", request_url_base, build_type);
    }
    // Building a request URL END

    let mut request_url = request_url_base.clone();
    // println!("{:?}", request_url);

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut total_count: u32;
    let mut builds_count = 0;
    let mut builds: Vec<Build> = Vec::new();
    loop {
        let response = client
            .get(&request_url)
            .header("client-api-key", &args.api_key)
            .send()
            .await?;
        let histories: History = response.json().await?;
        // dbg!(histories);
        total_count = histories.total_count;
        builds_count += histories.builds_count;
        builds.extend(histories.builds);
        // println!(
        //     "totalCount: {}, buildsCount: {}",
        //     histories.total_count, histories.builds_count
        // );

        match histories.next_page_token {
            Some(token) => {
                request_url = format!("{}&nextPageToken={}", request_url_base, token);
            }
            None => break,
        }
    }
    // println!("totalCount: {}, buildsCount: {}", total_count, builds_count);
    // dbg!(builds.len());
    // dbg!(builds);
    let history = History {
        total_count: total_count,
        builds_count: builds_count,
        next_page_token: None,
        builds: builds,
    };

    let json = serde_json::to_string_pretty(&history).unwrap();
    println!("{}", json);

    Ok(())
}
